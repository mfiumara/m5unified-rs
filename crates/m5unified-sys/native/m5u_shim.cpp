#include "m5u_shim.h"

#include <M5Unified.h>
#include <utility/PI4IOE5V6408_Class.hpp>
#include <utility/imu/AK8963_Class.hpp>
#include <utility/imu/BMI270_Class.hpp>
#include <utility/imu/BMM150_Class.hpp>
#include <utility/imu/MPU6886_Class.hpp>
#include <utility/imu/SH200Q_Class.hpp>
#include <utility/led/LED_PowerHub_Class.hpp>
#include <utility/led/LED_Strip_Class.hpp>
#include <utility/rtc/PCF8563_Class.hpp>
#include <utility/rtc/RTC_PowerHub_Class.hpp>
#include <utility/rtc/RX8130_Class.hpp>
#include <driver/gpio.h>
#include <driver/sdspi_host.h>
#include <driver/spi_common.h>
#include <esp_err.h>
#include <esp_vfs_fat.h>
#include <sdmmc_cmd.h>
#include <memory>
#include <string>
#include <vector>

static std::string s_m5u_log_suffixes[3];
static m5u_log_callback_t s_m5u_log_callback = nullptr;
static void* s_m5u_log_callback_user_data = nullptr;
static constexpr const char* M5U_SD_MOUNT_POINT = "/sdcard";
static sdmmc_card_t* s_m5u_sd_card = nullptr;
static spi_host_device_t s_m5u_sd_host = SPI2_HOST;
static bool s_m5u_sd_owns_bus = false;

#if defined(CONFIG_IDF_TARGET_ESP32C3) || defined(CONFIG_IDF_TARGET_ESP32C6) || defined(CONFIG_IDF_TARGET_ESP32P4)
#define M5U_HAS_AXP2101 0
#else
#define M5U_HAS_AXP2101 1
#endif

#if !defined(CONFIG_IDF_TARGET) || defined(CONFIG_IDF_TARGET_ESP32)
#define M5U_HAS_AXP192 1
#else
#define M5U_HAS_AXP192 0
#endif

#if defined(CONFIG_IDF_TARGET_ESP32C6)
#define M5U_HAS_AW32001 1
#define M5U_HAS_BQ27220 1
#else
#define M5U_HAS_AW32001 0
#define M5U_HAS_BQ27220 0
#endif

#if defined(CONFIG_IDF_TARGET_ESP32S3) || defined(CONFIG_IDF_TARGET_ESP32P4)
#define M5U_HAS_INA226 1
#else
#define M5U_HAS_INA226 0
#endif

#if !defined(CONFIG_IDF_TARGET) || defined(CONFIG_IDF_TARGET_ESP32)
#define M5U_HAS_IP5306 1
#define M5U_HAS_INA3221 1
#else
#define M5U_HAS_IP5306 0
#define M5U_HAS_INA3221 0
#endif

#if defined(CONFIG_IDF_TARGET_ESP32S3)
#define M5U_HAS_PY32PMIC 1
#else
#define M5U_HAS_PY32PMIC 0
#endif

#if defined(M5UNIFIED_RMT_VERSION) && M5UNIFIED_RMT_VERSION == 2
#define M5U_HAS_LED_STRIP_RMT 1
#else
#define M5U_HAS_LED_STRIP_RMT 0
#endif

extern "C" {

static auto m5u_apply_config(const m5u_config_t* src) {
    auto cfg = M5.config();
    if (!src) {
        return cfg;
    }
#if defined(ARDUINO)
    cfg.serial_baudrate = src->serial_baudrate;
#endif
    cfg.external_speaker_value = src->external_speaker_value;
    cfg.external_display_value = src->external_display_value;
    cfg.clear_display = src->clear_display != 0;
    cfg.output_power = src->output_power != 0;
    cfg.pmic_button = src->pmic_button != 0;
    cfg.internal_imu = src->internal_imu != 0;
    cfg.internal_rtc = src->internal_rtc != 0;
    cfg.internal_mic = src->internal_mic != 0;
    cfg.internal_spk = src->internal_spk != 0;
    cfg.external_imu = src->external_imu != 0;
    cfg.external_rtc = src->external_rtc != 0;
    cfg.disable_rtc_irq = src->disable_rtc_irq != 0;
    cfg.led_brightness = src->led_brightness;
    if (src->fallback_board >= 0) {
        cfg.fallback_board = (m5::board_t)src->fallback_board;
    }
    return cfg;
}

bool m5u_begin(void) {
    auto cfg = m5u_apply_config(nullptr);
    M5.begin(cfg);
    return true;
}

bool m5u_begin_with_config(const m5u_config_t* config) {
    auto cfg = m5u_apply_config(config);
    M5.begin(cfg);
    return true;
}

void m5u_update(void) {
    M5.update();
}

void m5u_delay_ms(uint32_t ms) {
    M5.delay(ms);
}

uint32_t m5u_millis(void) {
    return m5::M5Unified::millis();
}

uint32_t m5u_micros(void) {
    return m5::M5Unified::micros();
}

uint32_t m5u_get_update_msec(void) {
    return M5.getUpdateMsec();
}

int m5u_get_board(void) {
    return (int)M5.getBoard();
}

int m5u_get_pin(int name) {
    return M5.getPin((m5::pin_name_t)name);
}

bool m5u_set_primary_display_index(size_t index) {
    return M5.setPrimaryDisplay(index);
}

bool m5u_set_primary_display_type(int kind) {
    return M5.setPrimaryDisplayType((m5gfx::board_t)kind);
}

bool m5u_set_primary_display_types(const int* kinds, size_t len) {
    if (!kinds) {
        return false;
    }
    for (size_t i = 0; i < len; ++i) {
        if (M5.setPrimaryDisplayType((m5gfx::board_t)kinds[i])) {
            return true;
        }
    }
    return false;
}

void m5u_set_log_display_index(size_t index) {
    M5.setLogDisplayIndex(index);
}

void m5u_set_log_display_type(int kind) {
    M5.setLogDisplayType((m5gfx::board_t)kind);
}

void m5u_set_log_display_types(const int* kinds, size_t len) {
    if (!kinds) {
        return;
    }
    for (size_t i = 0; i < len; ++i) {
        int index = M5.getDisplayIndex((m5gfx::board_t)kinds[i]);
        if (index >= 0) {
            M5.setLogDisplayIndex((size_t)index);
            return;
        }
    }
}

void m5u_set_touch_button_height(uint16_t pixel) {
    M5.setTouchButtonHeight(pixel);
}

void m5u_set_touch_button_height_by_ratio(uint8_t ratio) {
    M5.setTouchButtonHeightByRatio(ratio);
}

uint16_t m5u_get_touch_button_height(void) {
    return M5.getTouchButtonHeight();
}

static bool m5u_io_expander_valid_index(size_t index) {
    switch (M5.getBoard()) {
#if defined(CONFIG_IDF_TARGET_ESP32P4)
    case m5::board_t::board_M5Tab5:
        return index < 2;
#elif defined(CONFIG_IDF_TARGET_ESP32C6)
    case m5::board_t::board_M5UnitC6L:
        return index == 0;
    case m5::board_t::board_ArduinoNessoN1:
        return index < 2;
#elif defined(CONFIG_IDF_TARGET_ESP32S3)
    case m5::board_t::board_M5StampPLC:
        return index == 0;
#endif
    default:
        return false;
    }
}

static m5::IOExpander_Base* m5u_io_expander(size_t index) {
    return m5u_io_expander_valid_index(index) ? &M5.getIOExpander(index) : nullptr;
}

bool m5u_io_expander_available(size_t index) {
    return m5u_io_expander_valid_index(index);
}

bool m5u_io_expander_set_direction(size_t index, uint8_t pin, bool output) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander || pin >= 8) {
        return false;
    }
    io_expander->setDirection(pin, output);
    return true;
}

bool m5u_io_expander_enable_pull(size_t index, uint8_t pin, bool enable) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander || pin >= 8) {
        return false;
    }
    io_expander->enablePull(pin, enable);
    return true;
}

bool m5u_io_expander_set_pull_mode(size_t index, uint8_t pin, bool pull_up) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander || pin >= 8) {
        return false;
    }
    io_expander->setPullMode(pin, pull_up);
    return true;
}

bool m5u_io_expander_set_high_impedance(size_t index, uint8_t pin, bool enable) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander || pin >= 8) {
        return false;
    }
    io_expander->setHighImpedance(pin, enable);
    return true;
}

bool m5u_io_expander_get_write_value(size_t index, uint8_t pin) {
    auto io_expander = m5u_io_expander(index);
    return io_expander && pin < 8 && io_expander->getWriteValue(pin);
}

bool m5u_io_expander_digital_write(size_t index, uint8_t pin, bool level) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander || pin >= 8) {
        return false;
    }
    io_expander->digitalWrite(pin, level);
    return true;
}

bool m5u_io_expander_digital_read(size_t index, uint8_t pin) {
    auto io_expander = m5u_io_expander(index);
    return io_expander && pin < 8 && io_expander->digitalRead(pin);
}

bool m5u_io_expander_reset_irq(size_t index) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander) {
        return false;
    }
    io_expander->resetIrq();
    return true;
}

bool m5u_io_expander_disable_irq(size_t index) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander) {
        return false;
    }
    io_expander->disableIrq();
    return true;
}

bool m5u_io_expander_enable_irq(size_t index) {
    auto io_expander = m5u_io_expander(index);
    if (!io_expander) {
        return false;
    }
    io_expander->enableIrq();
    return true;
}

static m5::PI4IOE5V6408_Class& m5u_pi4ioe5v6408(void) {
    static m5::PI4IOE5V6408_Class io_expander;
    return io_expander;
}

bool m5u_pi4ioe5v6408_begin(void) {
    return m5u_pi4ioe5v6408().begin();
}

bool m5u_pi4ioe5v6408_set_direction(uint8_t pin, bool output) {
    if (pin >= 8) {
        return false;
    }
    m5u_pi4ioe5v6408().setDirection(pin, output);
    return true;
}

bool m5u_pi4ioe5v6408_enable_pull(uint8_t pin, bool enable) {
    if (pin >= 8) {
        return false;
    }
    m5u_pi4ioe5v6408().enablePull(pin, enable);
    return true;
}

bool m5u_pi4ioe5v6408_set_pull_mode(uint8_t pin, bool pull_up) {
    if (pin >= 8) {
        return false;
    }
    m5u_pi4ioe5v6408().setPullMode(pin, pull_up);
    return true;
}

bool m5u_pi4ioe5v6408_set_high_impedance(uint8_t pin, bool enable) {
    if (pin >= 8) {
        return false;
    }
    m5u_pi4ioe5v6408().setHighImpedance(pin, enable);
    return true;
}

bool m5u_pi4ioe5v6408_get_write_value(uint8_t pin) {
    return pin < 8 && m5u_pi4ioe5v6408().getWriteValue(pin);
}

bool m5u_pi4ioe5v6408_digital_write(uint8_t pin, bool level) {
    if (pin >= 8) {
        return false;
    }
    m5u_pi4ioe5v6408().digitalWrite(pin, level);
    return true;
}

bool m5u_pi4ioe5v6408_digital_read(uint8_t pin) {
    return pin < 8 && m5u_pi4ioe5v6408().digitalRead(pin);
}

void m5u_pi4ioe5v6408_reset_irq(void) {
    m5u_pi4ioe5v6408().resetIrq();
}

void m5u_pi4ioe5v6408_disable_irq(void) {
    m5u_pi4ioe5v6408().disableIrq();
}

void m5u_pi4ioe5v6408_enable_irq(void) {
    m5u_pi4ioe5v6408().enableIrq();
}

static m5::I2C_Class* m5u_i2c_for_bus(int bus) {
    switch (bus) {
        case 0: return &M5.In_I2C;
        case 1: return &M5.Ex_I2C;
        default: return nullptr;
    }
}

void m5u_i2c_set_port(int bus, int port_num, int pin_sda, int pin_scl) {
    auto i2c = m5u_i2c_for_bus(bus);
    if (i2c) {
        i2c->setPort((i2c_port_t)port_num, pin_sda, pin_scl);
    }
}

bool m5u_i2c_begin(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->begin();
}

bool m5u_i2c_begin_with_port(int bus, int port_num, int pin_sda, int pin_scl) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->begin((i2c_port_t)port_num, pin_sda, pin_scl);
}

bool m5u_i2c_release(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->release();
}

bool m5u_i2c_is_enabled(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->isEnabled();
}

int m5u_i2c_get_port(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c ? (int)i2c->getPort() : -1;
}

int m5u_i2c_get_sda(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c ? i2c->getSDA() : -1;
}

int m5u_i2c_get_scl(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c ? i2c->getSCL() : -1;
}

bool m5u_i2c_start(int bus, uint8_t address, bool read, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->start(address, read, freq);
}

bool m5u_i2c_restart(int bus, uint8_t address, bool read, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->restart(address, read, freq);
}

bool m5u_i2c_stop(int bus) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->stop();
}

bool m5u_i2c_write_byte(int bus, uint8_t data) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->write(data);
}

bool m5u_i2c_write(int bus, const uint8_t* data, size_t length) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->write(data, length);
}

bool m5u_i2c_read(int bus, uint8_t* result, size_t length, bool last_nack) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->read(result, length, last_nack);
}

bool m5u_i2c_write_register(int bus, uint8_t address, uint8_t reg, const uint8_t* data, size_t length, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->writeRegister(address, reg, data, length, freq);
}

bool m5u_i2c_read_register(int bus, uint8_t address, uint8_t reg, uint8_t* result, size_t length, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->readRegister(address, reg, result, length, freq);
}

bool m5u_i2c_write_register8(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->writeRegister8(address, reg, data, freq);
}

uint8_t m5u_i2c_read_register8(int bus, uint8_t address, uint8_t reg, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c ? i2c->readRegister8(address, reg, freq) : 0;
}

bool m5u_i2c_bit_on(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->bitOn(address, reg, data, freq);
}

bool m5u_i2c_bit_off(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->bitOff(address, reg, data, freq);
}

void m5u_i2c_scan(int bus, bool* result, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    if (i2c && result) {
        i2c->scanID(result, freq);
    }
}

bool m5u_i2c_scan_address(int bus, uint8_t address, uint32_t freq) {
    auto i2c = m5u_i2c_for_bus(bus);
    return i2c && i2c->scanID(address, freq);
}

int m5u_display_width(void) {
    return M5.Display.width();
}

int m5u_display_height(void) {
    return M5.Display.height();
}

void m5u_display_fill_screen(uint16_t color) {
    M5.Display.fillScreen(color);
}

void m5u_display_set_cursor(int x, int y) {
    M5.Display.setCursor(x, y);
}

void m5u_display_set_text_size(int size) {
    M5.Display.setTextSize(size);
}

void m5u_display_set_text_color(uint16_t fg, uint16_t bg) {
    M5.Display.setTextColor(fg, bg);
}

void m5u_display_print(const char* text) {
    M5.Display.print(text);
}

void m5u_display_println(const char* text) {
    M5.Display.println(text);
}

void m5u_display_draw_line(int x0, int y0, int x1, int y1, uint16_t color) {
    M5.Display.drawLine(x0, y0, x1, y1, color);
}

void m5u_display_draw_rect(int x, int y, int w, int h, uint16_t color) {
    M5.Display.drawRect(x, y, w, h, color);
}

void m5u_display_fill_rect(int x, int y, int w, int h, uint16_t color) {
    M5.Display.fillRect(x, y, w, h, color);
}

void m5u_display_draw_circle(int x, int y, int r, uint16_t color) {
    M5.Display.drawCircle(x, y, r, color);
}

void m5u_display_fill_circle(int x, int y, int r, uint16_t color) {
    M5.Display.fillCircle(x, y, r, color);
}

void m5u_display_set_rotation(int rotation) {
    M5.Display.setRotation(rotation);
}

bool m5u_btn_a_is_pressed(void) {
    return M5.BtnA.isPressed();
}

bool m5u_btn_a_was_pressed(void) {
    return M5.BtnA.wasPressed();
}

bool m5u_btn_a_was_released(void) {
    return M5.BtnA.wasReleased();
}

bool m5u_btn_b_is_pressed(void) {
    return M5.BtnB.isPressed();
}

bool m5u_btn_b_was_pressed(void) {
    return M5.BtnB.wasPressed();
}

bool m5u_btn_b_was_released(void) {
    return M5.BtnB.wasReleased();
}

bool m5u_btn_c_is_pressed(void) {
    return M5.BtnC.isPressed();
}

bool m5u_btn_c_was_pressed(void) {
    return M5.BtnC.wasPressed();
}

bool m5u_btn_c_was_released(void) {
    return M5.BtnC.wasReleased();
}

bool m5u_mic_begin(void) {
    return M5.Mic.begin();
}

bool m5u_mic_is_running(void) {
    return M5.Mic.isRunning();
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    return M5.Mic.record(buffer, samples);
}

bool m5u_mic_record_u8(uint8_t* buffer, size_t samples) {
    return M5.Mic.record(buffer, samples);
}

bool m5u_speaker_begin(void) {
    return M5.Speaker.begin();
}

bool m5u_speaker_is_running(void) {
    return M5.Speaker.isRunning();
}

void m5u_speaker_set_volume(uint8_t volume) {
    M5.Speaker.setVolume(volume);
}

bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms) {
    return M5.Speaker.tone(frequency_hz, duration_ms);
}

bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, false, 1, 0);
}

bool m5u_imu_begin(void) {
    return M5.Imu.begin();
}

bool m5u_imu_begin_for_board(int board) {
    return M5.Imu.begin(nullptr, (m5::board_t)board);
}

bool m5u_imu_get_accel(float* x, float* y, float* z) {
    return M5.Imu.getAccel(x, y, z);
}

bool m5u_imu_get_gyro(float* x, float* y, float* z) {
    return M5.Imu.getGyro(x, y, z);
}

bool m5u_imu_get_mag(float* x, float* y, float* z) {
    return M5.Imu.getMag(x, y, z);
}

bool m5u_imu_get_data(m5u_imu_data_t* out) {
    if (!M5.Imu.isEnabled() || !out) {
        return false;
    }
    auto data = M5.Imu.getImuData();
    out->usec = data.usec;
    out->accel_x = data.accel.x;
    out->accel_y = data.accel.y;
    out->accel_z = data.accel.z;
    out->gyro_x = data.gyro.x;
    out->gyro_y = data.gyro.y;
    out->gyro_z = data.gyro.z;
    out->mag_x = data.mag.x;
    out->mag_y = data.mag.y;
    out->mag_z = data.mag.z;
    return true;
}

bool m5u_imu_get_temp_c(float* temp) {
    return M5.Imu.getTemp(temp);
}

int m5u_touch_count(void) {
    return M5.Touch.getCount();
}

bool m5u_touch_is_enabled(void) {
    return M5.Touch.isEnabled();
}

void m5u_touch_set_hold_thresh(uint16_t ms) {
    M5.Touch.setHoldThresh(ms);
}

void m5u_touch_set_flick_thresh(uint16_t distance) {
    M5.Touch.setFlickThresh(distance);
}

bool m5u_touch_get(int index, int* x, int* y) {
    auto detail = M5.Touch.getDetail(index);
    if (x) { *x = detail.x; }
    if (y) { *y = detail.y; }
    return detail.isPressed();
}

bool m5u_touch_get_raw(int index, int* x, int* y) {
    if (index < 0 || index >= M5.Touch.getCount()) {
        return false;
    }
    auto point = M5.Touch.getTouchPointRaw(index);
    if (x) { *x = point.x; }
    if (y) { *y = point.y; }
    return true;
}

bool m5u_rtc_begin(void) {
    return M5.Rtc.begin();
}

bool m5u_rtc_begin_for_board(int board) {
    return M5.Rtc.begin(nullptr, (m5::board_t)board);
}

bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second) {
    m5::rtc_datetime_t dt;
    if (!M5.Rtc.getDateTime(&dt)) { return false; }
    if (year) { *year = dt.date.year; }
    if (month) { *month = dt.date.month; }
    if (day) { *day = dt.date.date; }
    if (hour) { *hour = dt.time.hours; }
    if (minute) { *minute = dt.time.minutes; }
    if (second) { *second = dt.time.seconds; }
    return true;
}

static void m5u_rtc_to_raw(const m5::rtc_datetime_t& src, m5u_rtc_datetime_t* out) {
    out->year = src.date.year;
    out->month = src.date.month;
    out->day = src.date.date;
    out->weekday = src.date.weekDay;
    out->hour = src.time.hours;
    out->minute = src.time.minutes;
    out->second = src.time.seconds;
}

static void m5u_rtc_date_to_raw(const m5::rtc_date_t& src, m5u_rtc_datetime_t* out) {
    out->year = src.year;
    out->month = src.month;
    out->day = src.date;
    out->weekday = src.weekDay;
    out->hour = 0;
    out->minute = 0;
    out->second = 0;
}

static void m5u_rtc_time_to_raw(const m5::rtc_time_t& src, m5u_rtc_datetime_t* out) {
    out->year = 0;
    out->month = 0;
    out->day = 0;
    out->weekday = -1;
    out->hour = src.hours;
    out->minute = src.minutes;
    out->second = src.seconds;
}

static m5::rtc_datetime_t m5u_rtc_from_raw(const m5u_rtc_datetime_t* src) {
    m5::rtc_datetime_t dt = {};
    dt.date.year = src->year;
    dt.date.month = src->month;
    dt.date.date = src->day;
    dt.date.weekDay = src->weekday;
    dt.time.hours = src->hour;
    dt.time.minutes = src->minute;
    dt.time.seconds = src->second;
    return dt;
}

static m5::rtc_date_t m5u_rtc_date_from_raw(const m5u_rtc_datetime_t* src) {
    m5::rtc_date_t date = {};
    date.year = src->year;
    date.month = src->month;
    date.date = src->day;
    date.weekDay = src->weekday;
    return date;
}

static m5::rtc_time_t m5u_rtc_time_from_raw(const m5u_rtc_datetime_t* src) {
    m5::rtc_time_t time = {};
    time.hours = src->hour;
    time.minutes = src->minute;
    time.seconds = src->second;
    return time;
}

bool m5u_rtc_get_datetime_detail(m5u_rtc_datetime_t* out) {
    if (!out) {
        return false;
    }
    m5::rtc_datetime_t dt;
    if (!M5.Rtc.getDateTime(&dt)) {
        return false;
    }
    m5u_rtc_to_raw(dt, out);
    return true;
}

bool m5u_rtc_get_date_detail(m5u_rtc_datetime_t* out) {
    if (!out) {
        return false;
    }
    m5::rtc_date_t date;
    if (!M5.Rtc.getDate(&date)) {
        return false;
    }
    m5u_rtc_date_to_raw(date, out);
    return true;
}

bool m5u_rtc_get_time_detail(m5u_rtc_datetime_t* out) {
    if (!out) {
        return false;
    }
    m5::rtc_time_t time;
    if (!M5.Rtc.getTime(&time)) {
        return false;
    }
    m5u_rtc_time_to_raw(time, out);
    return true;
}

bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second) {
    m5::rtc_datetime_t dt;
    dt.date.year = year;
    dt.date.month = month;
    dt.date.date = day;
    dt.time.hours = hour;
    dt.time.minutes = minute;
    dt.time.seconds = second;
    M5.Rtc.setDateTime(&dt);
    return true;
}

bool m5u_rtc_set_datetime_detail(const m5u_rtc_datetime_t* datetime) {
    if (!datetime) {
        return false;
    }
    auto dt = m5u_rtc_from_raw(datetime);
    M5.Rtc.setDateTime(&dt);
    return true;
}

bool m5u_rtc_set_date_detail(const m5u_rtc_datetime_t* date) {
    if (!date) {
        return false;
    }
    auto raw = m5u_rtc_date_from_raw(date);
    M5.Rtc.setDate(&raw);
    return true;
}

bool m5u_rtc_set_time_detail(const m5u_rtc_datetime_t* time) {
    if (!time) {
        return false;
    }
    auto raw = m5u_rtc_time_from_raw(time);
    M5.Rtc.setTime(&raw);
    return true;
}

void m5u_rtc_set_system_time_from_rtc(void) {
    M5.Rtc.setSystemTimeFromRtc();
}

bool m5u_rtc_get_volt_low(void) {
    return M5.Rtc.getVoltLow();
}

uint32_t m5u_rtc_set_timer_irq(uint32_t timer_msec) {
    return M5.Rtc.setTimerIRQ(timer_msec);
}

int m5u_rtc_set_alarm_irq_after_seconds(int after_seconds) {
    return M5.Rtc.setAlarmIRQ(after_seconds);
}

int m5u_rtc_set_alarm_irq_datetime(const m5u_rtc_datetime_t* datetime) {
    if (!datetime) {
        return -1;
    }
    auto dt = m5u_rtc_from_raw(datetime);
    return M5.Rtc.setAlarmIRQ(&dt.date, &dt.time);
}

int m5u_rtc_set_alarm_irq_time(const m5u_rtc_datetime_t* time) {
    if (!time) {
        return -1;
    }
    auto raw = m5u_rtc_time_from_raw(time);
    return M5.Rtc.setAlarmIRQ((const m5::rtc_date_t*)nullptr, &raw);
}

bool m5u_rtc_get_irq_status(void) {
    return M5.Rtc.getIRQstatus();
}

void m5u_rtc_clear_irq(void) {
    M5.Rtc.clearIRQ();
}

void m5u_rtc_disable_irq(void) {
    M5.Rtc.disableIRQ();
}

static m5::RTC_Base* m5u_rtc_device_for_kind(int kind) {
    static m5::PCF8563_Class pcf8563;
    static m5::RX8130_Class rx8130;
    static m5::RTC_PowerHub_Class power_hub;

    switch (kind) {
        case 0: return &pcf8563;
        case 1: return &rx8130;
        case 2: return &power_hub;
        default: return nullptr;
    }
}

bool m5u_rtc_device_begin(int kind) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    return rtc ? rtc->begin() : false;
}

bool m5u_rtc_device_get_datetime_detail(int kind, m5u_rtc_datetime_t* out) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !out) {
        return false;
    }
    m5::rtc_date_t date;
    m5::rtc_time_t time;
    if (!rtc->getDateTime(&date, &time)) {
        return false;
    }
    m5u_rtc_to_raw(m5::rtc_datetime_t(date, time), out);
    return true;
}

bool m5u_rtc_device_get_date_detail(int kind, m5u_rtc_datetime_t* out) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !out) {
        return false;
    }
    m5::rtc_date_t date;
    if (!rtc->getDateTime(&date, nullptr)) {
        return false;
    }
    m5u_rtc_date_to_raw(date, out);
    return true;
}

bool m5u_rtc_device_get_time_detail(int kind, m5u_rtc_datetime_t* out) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !out) {
        return false;
    }
    m5::rtc_time_t time;
    if (!rtc->getDateTime(nullptr, &time)) {
        return false;
    }
    m5u_rtc_time_to_raw(time, out);
    return true;
}

bool m5u_rtc_device_set_datetime_detail(int kind, const m5u_rtc_datetime_t* datetime) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !datetime) {
        return false;
    }
    auto raw = m5u_rtc_from_raw(datetime);
    return rtc->setDateTime(&raw.date, &raw.time);
}

bool m5u_rtc_device_set_date_detail(int kind, const m5u_rtc_datetime_t* date) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !date) {
        return false;
    }
    auto raw = m5u_rtc_date_from_raw(date);
    return rtc->setDateTime(&raw, nullptr);
}

bool m5u_rtc_device_set_time_detail(int kind, const m5u_rtc_datetime_t* time) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !time) {
        return false;
    }
    auto raw = m5u_rtc_time_from_raw(time);
    return rtc->setDateTime(nullptr, &raw);
}

bool m5u_rtc_device_get_volt_low(int kind) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    return rtc ? rtc->getVoltLow() : false;
}

uint32_t m5u_rtc_device_set_timer_irq(int kind, uint32_t timer_msec) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    return rtc ? rtc->setTimerIRQ(timer_msec) : 0;
}

int m5u_rtc_device_set_alarm_irq_datetime(int kind, const m5u_rtc_datetime_t* datetime) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !datetime) {
        return -1;
    }
    auto raw = m5u_rtc_from_raw(datetime);
    return rtc->setAlarmIRQ(&raw.date, &raw.time);
}

int m5u_rtc_device_set_alarm_irq_time(int kind, const m5u_rtc_datetime_t* time) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (!rtc || !time) {
        return -1;
    }
    auto raw = m5u_rtc_time_from_raw(time);
    return rtc->setAlarmIRQ(nullptr, &raw);
}

bool m5u_rtc_device_get_irq_status(int kind) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    return rtc ? rtc->getIRQstatus() : false;
}

void m5u_rtc_device_clear_irq(int kind) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (rtc) {
        rtc->clearIRQ();
    }
}

void m5u_rtc_device_disable_irq(int kind) {
    auto rtc = m5u_rtc_device_for_kind(kind);
    if (rtc) {
        rtc->disableIRQ();
    }
}

int m5u_battery_level(void) {
    return M5.Power.getBatteryLevel();
}

int m5u_battery_voltage_mv(void) {
    return M5.Power.getBatteryVoltage();
}

bool m5u_power_begin(void) {
    return M5.Power.begin();
}

int m5u_power_get_type(void) {
    return (int)M5.Power.getType();
}

int m5u_power_get_charge_state(void) {
    return (int)M5.Power.isCharging();
}

bool m5u_power_is_charging(void) {
    return M5.Power.isCharging() == m5::Power_Class::is_charging_t::is_charging;
}

void m5u_power_set_led(uint8_t brightness) {
    M5.Power.setLed(brightness);
}

void m5u_power_set_ext_output(bool enable, uint16_t port_mask) {
    M5.Power.setExtOutput(enable, (m5::ext_port_mask_t)port_mask);
}

bool m5u_power_get_ext_output(void) {
    return M5.Power.getExtOutput();
}

void m5u_power_set_usb_output(bool enable) {
    M5.Power.setUsbOutput(enable);
}

bool m5u_power_get_usb_output(void) {
    return M5.Power.getUsbOutput();
}

void m5u_power_set_battery_charge(bool enable) {
    M5.Power.setBatteryCharge(enable);
}

void m5u_power_set_charge_current(uint16_t max_ma) {
    M5.Power.setChargeCurrent(max_ma);
}

void m5u_power_set_charge_voltage(uint16_t max_mv) {
    M5.Power.setChargeVoltage(max_mv);
}

int m5u_power_get_vbus_voltage_mv(void) {
    return M5.Power.getVBUSVoltage();
}

int m5u_power_get_battery_current_ma(void) {
    return M5.Power.getBatteryCurrent();
}

float m5u_power_get_ext_voltage_mv(uint16_t port_mask) {
    return M5.Power.getExtVoltage((m5::ext_port_mask_t)port_mask);
}

float m5u_power_get_ext_current_ma(uint16_t port_mask) {
    return M5.Power.getExtCurrent((m5::ext_port_mask_t)port_mask);
}

uint8_t m5u_power_get_key_state(void) {
    return M5.Power.getKeyState();
}

void m5u_power_set_ext_port_bus_config(const m5u_power_ext_port_bus_t* config) {
    if (!config) {
        return;
    }
    m5::ext_port_bus_t bus_config;
    bus_config.voltage = config->voltage_mv;
    bus_config.currentLimit = config->current_limit_ma;
    bus_config.enable = config->enable;
    bus_config.direction = config->direction_output;
    M5.Power.setExtPortBusConfig(bus_config);
}

void m5u_power_set_vibration(uint8_t level) {
    M5.Power.setVibration(level);
}

void m5u_power_power_off(void) {
    M5.Power.powerOff();
}

void m5u_power_timer_sleep_seconds(int seconds) {
    M5.Power.timerSleep(seconds);
}

void m5u_power_timer_sleep_time(const m5u_rtc_datetime_t* time) {
    if (!time) {
        return;
    }
    auto raw = m5u_rtc_time_from_raw(time);
    M5.Power.timerSleep(raw);
}

void m5u_power_timer_sleep_date_time(const m5u_rtc_datetime_t* date, const m5u_rtc_datetime_t* time) {
    if (!date || !time) {
        return;
    }
    auto raw_date = m5u_rtc_date_from_raw(date);
    auto raw_time = m5u_rtc_time_from_raw(time);
    M5.Power.timerSleep(raw_date, raw_time);
}

void m5u_power_deep_sleep_us(uint64_t micro_seconds, bool touch_wakeup) {
    M5.Power.deepSleep(micro_seconds, touch_wakeup);
}

void m5u_power_light_sleep_us(uint64_t micro_seconds, bool touch_wakeup) {
    M5.Power.lightSleep(micro_seconds, touch_wakeup);
}

void m5u_log_println(const char* text) {
    M5.Log.println(text);
}

bool m5u_sd_begin(void) {
    m5u_sd_spi_config_t config = {};
    config.pin_sclk = M5.getPin(m5::pin_name_t::sd_spi_sclk);
    config.pin_mosi = M5.getPin(m5::pin_name_t::sd_spi_mosi);
    config.pin_miso = M5.getPin(m5::pin_name_t::sd_spi_miso);
    config.pin_cs = M5.getPin(m5::pin_name_t::sd_spi_cs);
    config.host_id = -1;
    config.frequency_khz = 20000;
    config.max_files = 5;
    config.format_if_mount_failed = 0;
    return m5u_sd_begin_spi(&config);
}

bool m5u_sd_begin_spi(const m5u_sd_spi_config_t* config) {
    if (s_m5u_sd_card) {
        return true;
    }
    if (!config || config->pin_sclk < 0 || config->pin_mosi < 0 || config->pin_miso < 0 || config->pin_cs < 0) {
        return false;
    }

    sdmmc_host_t host = SDSPI_HOST_DEFAULT();
    spi_host_device_t spi_host = static_cast<spi_host_device_t>(host.slot);
    if (config->host_id >= 0) {
        spi_host = static_cast<spi_host_device_t>(config->host_id);
        host.slot = static_cast<int>(spi_host);
    }
    if (config->frequency_khz > 0) {
        host.max_freq_khz = config->frequency_khz;
    }

    spi_bus_config_t bus_config = {};
    bus_config.mosi_io_num = config->pin_mosi;
    bus_config.miso_io_num = config->pin_miso;
    bus_config.sclk_io_num = config->pin_sclk;
    bus_config.quadwp_io_num = GPIO_NUM_NC;
    bus_config.quadhd_io_num = GPIO_NUM_NC;
    bus_config.max_transfer_sz = 4000;

    esp_err_t err = spi_bus_initialize(spi_host, &bus_config, SDSPI_DEFAULT_DMA);
    bool owns_bus = false;
    if (err == ESP_OK) {
        owns_bus = true;
    } else if (err != ESP_ERR_INVALID_STATE) {
        return false;
    }

    sdspi_device_config_t slot_config = SDSPI_DEVICE_CONFIG_DEFAULT();
    slot_config.gpio_cs = (gpio_num_t)config->pin_cs;
    slot_config.host_id = spi_host;

    esp_vfs_fat_mount_config_t mount_config = VFS_FAT_MOUNT_DEFAULT_CONFIG();
    mount_config.format_if_mount_failed = config->format_if_mount_failed != 0;
    if (config->max_files > 0) {
        mount_config.max_files = config->max_files;
    }

    err = esp_vfs_fat_sdspi_mount(M5U_SD_MOUNT_POINT, &host, &slot_config, &mount_config, &s_m5u_sd_card);
    if (err != ESP_OK) {
        s_m5u_sd_card = nullptr;
        if (owns_bus) {
            spi_bus_free(spi_host);
        }
        return false;
    }

    s_m5u_sd_host = spi_host;
    s_m5u_sd_owns_bus = owns_bus;
    return true;
}

bool m5u_sd_is_mounted(void) {
    return s_m5u_sd_card != nullptr;
}

void m5u_sd_end(void) {
    if (!s_m5u_sd_card) {
        return;
    }
    esp_vfs_fat_sdcard_unmount(M5U_SD_MOUNT_POINT, s_m5u_sd_card);
    s_m5u_sd_card = nullptr;
    if (s_m5u_sd_owns_bus) {
        spi_bus_free(s_m5u_sd_host);
    }
    s_m5u_sd_owns_bus = false;
}


static m5::Button_Class* m5u_button_for_id(int button) {
    switch (button) {
    case 0: return &M5.BtnA;
    case 1: return &M5.BtnB;
    case 2: return &M5.BtnC;
    case 3: return &M5.BtnEXT;
    case 4: return &M5.BtnPWR;
    default: return nullptr;
    }
}

static bool m5u_button_state(int button, int query) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    if (!btn) {
        return false;
    }

    switch (query) {
    case 0: return btn->isPressed();
    case 1: return btn->wasPressed();
    case 2: return btn->wasReleased();
    case 3: return btn->wasClicked();
    case 4: return btn->wasHold();
    case 5: return btn->isHolding();
    case 6: return btn->wasDecideClickCount();
    case 7: return btn->wasSingleClicked();
    case 8: return btn->wasDoubleClicked();
    case 9: return btn->wasChangePressed();
    case 10: return btn->isReleased();
    case 11: return btn->wasReleasedAfterHold();
    default: return false;
    }
}

int m5u_display_get_rotation(void) {
    return M5.Display.getRotation();
}

void m5u_display_set_brightness(uint8_t brightness) {
    M5.Display.setBrightness(brightness);
}

void m5u_display_set_epd_fastest(void) {
    M5.Display.setEpdMode(m5gfx::epd_fastest);
}

void m5u_display_set_epd_mode(int mode) {
    switch (mode) {
    case m5gfx::epd_quality:
    case m5gfx::epd_text:
    case m5gfx::epd_fast:
    case m5gfx::epd_fastest:
        M5.Display.setEpdMode((m5gfx::epd_mode_t)mode);
        break;
    default:
        break;
    }
}

void m5u_display_set_text_scroll(bool scroll) {
    M5.Display.setTextScroll(scroll);
}

bool m5u_display_set_font(int font) {
    switch (font) {
    case 0:
        M5.Display.setFont(nullptr);
        return true;
    case 1:
        M5.Display.setFont(&fonts::AsciiFont8x16);
        return true;
    case 2:
        M5.Display.setFont(&fonts::lgfxJapanGothic_12);
        return true;
    case 3:
        M5.Display.setFont(&fonts::DejaVu18);
        return true;
    default:
        return false;
    }
}

void m5u_display_start_write(void) {
    M5.Display.startWrite();
}

void m5u_display_end_write(void) {
    M5.Display.endWrite();
}

void m5u_display_display(void) {
    M5.Display.display();
}

bool m5u_display_display_busy(void) {
    return M5.Display.displayBusy();
}

void m5u_display_wait_display(void) {
    M5.Display.waitDisplay();
}

int m5u_display_get_cursor_y(void) {
    return M5.Display.getCursorY();
}

int m5u_display_font_height(void) {
    return M5.Display.fontHeight();
}

uint16_t m5u_display_get_base_color(void) {
    return M5.Display.getBaseColor();
}

void m5u_display_set_color(uint16_t color) {
    M5.Display.setColor(color);
}

void m5u_display_set_text_wrap(bool wrap_x, bool wrap_y) {
    M5.Display.setTextWrap(wrap_x, wrap_y);
}

void m5u_display_set_text_datum(int datum) {
    M5.Display.setTextDatum((textdatum_t)datum);
}

int m5u_display_draw_string(const char* text, int x, int y) {
    return M5.Display.drawString(text, x, y);
}

void m5u_display_write_pixel(int x, int y, uint16_t color) {
    M5.Display.writePixel(x, y, color);
}

void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color) {
    M5.Display.writeFastVLine(x, y, h, color);
}

void m5u_display_set_clip_rect(int x, int y, int w, int h) {
    M5.Display.setClipRect(x, y, w, h);
}

void m5u_display_clear_clip_rect(void) {
    M5.Display.clearClipRect();
}

uint16_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) {
    return M5.Display.color888(r, g, b);
}

int m5u_display_count(void) {
    return M5.getDisplayCount();
}

int m5u_display_index_for_kind(int kind) {
    return M5.getDisplayIndex((m5::board_t)kind);
}

int m5u_display_index_for_kinds(const int* kinds, size_t len) {
    if (!kinds) {
        return -1;
    }
    for (size_t i = 0; i < len; ++i) {
        int index = M5.getDisplayIndex((m5::board_t)kinds[i]);
        if (index >= 0) {
            return index;
        }
    }
    return -1;
}

int m5u_display_width_at(int index) {
    return M5.Displays(index).width();
}

int m5u_display_height_at(int index) {
    return M5.Displays(index).height();
}

void m5u_display_fill_screen_at(int index, uint16_t color) {
    M5.Displays(index).fillScreen(color);
}

void m5u_display_set_cursor_at(int index, int x, int y) {
    M5.Displays(index).setCursor(x, y);
}

void m5u_display_set_text_size_at(int index, int size) {
    M5.Displays(index).setTextSize(size);
}

void m5u_display_set_text_color_at(int index, uint16_t fg, uint16_t bg) {
    M5.Displays(index).setTextColor(fg, bg);
}

int m5u_display_get_rotation_at(int index) {
    return M5.Displays(index).getRotation();
}

void m5u_display_set_rotation_at(int index, int rotation) {
    M5.Displays(index).setRotation(rotation);
}

void m5u_display_set_color_at(int index, uint16_t color) {
    M5.Displays(index).setColor(color);
}

void m5u_display_start_write_at(int index) {
    M5.Displays(index).startWrite();
}

void m5u_display_end_write_at(int index) {
    M5.Displays(index).endWrite();
}

void m5u_display_print_at(int index, const char* text) {
    M5.Displays(index).print(text);
}

void m5u_display_println_at(int index, const char* text) {
    M5.Displays(index).println(text);
}

int m5u_display_draw_string_at(int index, const char* text, int x, int y) {
    return M5.Displays(index).drawString(text, x, y);
}

void m5u_display_draw_line_at(int index, int x0, int y0, int x1, int y1, uint16_t color) {
    M5.Displays(index).drawLine(x0, y0, x1, y1, color);
}

void m5u_display_draw_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
    M5.Displays(index).drawRect(x, y, w, h, color);
}

void m5u_display_fill_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
    M5.Displays(index).fillRect(x, y, w, h, color);
}

void m5u_display_draw_circle_at(int index, int x, int y, int r, uint16_t color) {
    M5.Displays(index).drawCircle(x, y, r, color);
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
    M5.Displays(index).fillCircle(x, y, r, color);
}

void m5u_display_write_pixel_at(int index, int x, int y, uint16_t color) {
    M5.Displays(index).writePixel(x, y, color);
}

void m5u_display_draw_pixel_at(int index, int x, int y, uint16_t color) {
    M5.Displays(index).drawPixel(x, y, color);
}

bool m5u_button_is_pressed(int button) { return m5u_button_state(button, 0); }
bool m5u_button_was_pressed(int button) { return m5u_button_state(button, 1); }
bool m5u_button_was_released(int button) { return m5u_button_state(button, 2); }
bool m5u_button_was_clicked(int button) { return m5u_button_state(button, 3); }
bool m5u_button_was_hold(int button) { return m5u_button_state(button, 4); }
bool m5u_button_is_holding(int button) { return m5u_button_state(button, 5); }
bool m5u_button_was_decide_click_count(int button) { return m5u_button_state(button, 6); }
bool m5u_button_was_single_clicked(int button) { return m5u_button_state(button, 7); }
bool m5u_button_was_double_clicked(int button) { return m5u_button_state(button, 8); }
bool m5u_button_was_change_pressed(int button) { return m5u_button_state(button, 9); }
bool m5u_button_is_released(int button) { return m5u_button_state(button, 10); }
bool m5u_button_was_released_after_hold(int button) { return m5u_button_state(button, 11); }
int m5u_button_get_click_count(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->getClickCount() : 0;
}
bool m5u_button_was_release_for(int button, uint32_t ms) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->wasReleaseFor(ms) : false;
}
bool m5u_button_pressed_for(int button, uint32_t ms) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->pressedFor(ms) : false;
}
bool m5u_button_released_for(int button, uint32_t ms) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->releasedFor(ms) : false;
}
void m5u_button_set_debounce_thresh(int button, uint32_t ms) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    if (btn) {
        btn->setDebounceThresh(ms);
    }
}
void m5u_button_set_hold_thresh(int button, uint32_t ms) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    if (btn) {
        btn->setHoldThresh(ms);
    }
}
void m5u_button_set_raw_state(int button, uint32_t msec, bool press) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    if (btn) {
        btn->setRawState(msec, press);
    }
}
void m5u_button_set_state(int button, uint32_t msec, uint8_t state) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    if (btn) {
        btn->setState(msec, static_cast<m5::Button_Class::button_state_t>(state));
    }
}
uint8_t m5u_button_get_state(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? static_cast<uint8_t>(btn->getState()) : 0;
}
uint32_t m5u_button_last_change(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->lastChange() : 0;
}
uint32_t m5u_button_get_debounce_thresh(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->getDebounceThresh() : 0;
}
uint32_t m5u_button_get_hold_thresh(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->getHoldThresh() : 0;
}
uint32_t m5u_button_get_update_msec(int button) {
    m5::Button_Class* btn = m5u_button_for_id(button);
    return btn ? btn->getUpdateMsec() : 0;
}

bool m5u_mic_is_enabled(void) {
    return M5.Mic.isEnabled();
}

bool m5u_mic_is_recording(void) {
    return M5.Mic.isRecording();
}

size_t m5u_mic_recording_state(void) {
    return M5.Mic.isRecording();
}

void m5u_mic_end(void) {
    M5.Mic.end();
}

bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz) {
    return M5.Mic.record(buffer, samples, sample_rate_hz);
}

bool m5u_mic_record_i16_ex(int16_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo) {
    return M5.Mic.record(buffer, samples, sample_rate_hz, stereo);
}

bool m5u_mic_record_u8_ex(uint8_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo) {
    return M5.Mic.record(buffer, samples, sample_rate_hz, stereo);
}

void m5u_mic_set_sample_rate(uint32_t sample_rate_hz) {
    M5.Mic.setSampleRate(sample_rate_hz);
}

bool m5u_mic_get_config(m5u_mic_config_t* out) {
    if (!out) {
        return false;
    }
    auto cfg = M5.Mic.config();
    out->pin_data_in = cfg.pin_data_in;
    out->pin_bck = cfg.pin_bck;
    out->pin_mck = cfg.pin_mck;
    out->pin_ws = cfg.pin_ws;
    out->sample_rate = cfg.sample_rate;
    out->left_channel = cfg.left_channel;
    out->stereo = cfg.stereo;
    out->over_sampling = cfg.over_sampling;
    out->magnification = cfg.magnification;
    out->noise_filter_level = cfg.noise_filter_level;
    out->use_adc = cfg.use_adc;
    out->dma_buf_len = cfg.dma_buf_len;
    out->dma_buf_count = cfg.dma_buf_count;
    out->task_priority = cfg.task_priority;
    out->task_pinned_core = cfg.task_pinned_core;
    out->i2s_port = (int)cfg.i2s_port;
    return true;
}

bool m5u_mic_set_config(const m5u_mic_config_t* config) {
    if (!config) {
        return false;
    }
    auto cfg = M5.Mic.config();
    cfg.pin_data_in = config->pin_data_in;
    cfg.pin_bck = config->pin_bck;
    cfg.pin_mck = config->pin_mck;
    cfg.pin_ws = config->pin_ws;
    cfg.sample_rate = config->sample_rate;
    cfg.left_channel = config->left_channel != 0;
    cfg.stereo = config->stereo != 0;
    cfg.over_sampling = config->over_sampling;
    cfg.magnification = config->magnification;
    cfg.noise_filter_level = config->noise_filter_level;
    cfg.use_adc = config->use_adc != 0;
    cfg.dma_buf_len = config->dma_buf_len;
    cfg.dma_buf_count = config->dma_buf_count;
    cfg.task_priority = config->task_priority;
    cfg.task_pinned_core = config->task_pinned_core;
    cfg.i2s_port = (i2s_port_t)config->i2s_port;
    M5.Mic.config(cfg);
    return true;
}

int m5u_mic_get_noise_filter_level(void) {
    return M5.Mic.config().noise_filter_level;
}

bool m5u_mic_set_noise_filter_level(int level) {
    auto cfg = M5.Mic.config();
    cfg.noise_filter_level = level;
    M5.Mic.config(cfg);
    return true;
}

bool m5u_speaker_is_enabled(void) {
    return M5.Speaker.isEnabled();
}

void m5u_speaker_end(void) {
    M5.Speaker.end();
}

uint8_t m5u_speaker_get_volume(void) {
    return M5.Speaker.getVolume();
}

bool m5u_speaker_get_config(m5u_speaker_config_t* out) {
    if (!out) {
        return false;
    }
    auto cfg = M5.Speaker.config();
    out->pin_data_out = cfg.pin_data_out;
    out->pin_bck = cfg.pin_bck;
    out->pin_mck = cfg.pin_mck;
    out->pin_ws = cfg.pin_ws;
    out->sample_rate = cfg.sample_rate;
    out->stereo = cfg.stereo;
    out->buzzer = cfg.buzzer;
    out->use_dac = cfg.use_dac;
    out->dac_zero_level = cfg.dac_zero_level;
    out->magnification = cfg.magnification;
    out->dma_buf_len = cfg.dma_buf_len;
    out->dma_buf_count = cfg.dma_buf_count;
    out->task_priority = cfg.task_priority;
    out->task_pinned_core = cfg.task_pinned_core;
    out->i2s_port = (int)cfg.i2s_port;
    return true;
}

bool m5u_speaker_set_config(const m5u_speaker_config_t* config) {
    if (!config) {
        return false;
    }
    auto cfg = M5.Speaker.config();
    cfg.pin_data_out = config->pin_data_out;
    cfg.pin_bck = config->pin_bck;
    cfg.pin_mck = config->pin_mck;
    cfg.pin_ws = config->pin_ws;
    cfg.sample_rate = config->sample_rate;
    cfg.stereo = config->stereo != 0;
    cfg.buzzer = config->buzzer != 0;
    cfg.use_dac = config->use_dac != 0;
    cfg.dac_zero_level = config->dac_zero_level;
    cfg.magnification = config->magnification;
    cfg.dma_buf_len = config->dma_buf_len;
    cfg.dma_buf_count = config->dma_buf_count;
    cfg.task_priority = config->task_priority;
    cfg.task_pinned_core = config->task_pinned_core;
    cfg.i2s_port = (i2s_port_t)config->i2s_port;
    M5.Speaker.config(cfg);
    return true;
}

bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel) {
    return M5.Speaker.tone(frequency_hz, duration_ms, channel);
}

bool m5u_speaker_tone_options(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound) {
    return M5.Speaker.tone(frequency_hz, duration_ms, channel, stop_current_sound);
}

bool m5u_speaker_tone_full(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound, const uint8_t* raw_data, size_t len, bool stereo) {
    return raw_data && len ? M5.Speaker.tone(frequency_hz, duration_ms, channel, stop_current_sound, raw_data, len, stereo) : false;
}

bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, false, 1, 0);
}

bool m5u_speaker_play_u8_ex(const uint8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, stereo, repeat, channel, stop_current_sound);
}

bool m5u_speaker_play_i8_ex(const int8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, stereo, repeat, channel, stop_current_sound);
}

bool m5u_speaker_play_i16_ex(const int16_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, stereo, repeat, channel, stop_current_sound);
}

bool m5u_speaker_play_wav(const uint8_t* data, size_t len) {
    return M5.Speaker.playWav(data, len);
}

bool m5u_speaker_play_wav_ex(const uint8_t* data, size_t len, uint32_t repeat, int channel, bool stop_current_sound) {
    return M5.Speaker.playWav(data, len, repeat, channel, stop_current_sound);
}

bool m5u_speaker_is_playing(int channel) {
    return channel < 0 ? M5.Speaker.isPlaying() : M5.Speaker.isPlaying(channel);
}

size_t m5u_speaker_playing_channels(void) {
    return M5.Speaker.getPlayingChannels();
}

size_t m5u_speaker_channel_playing_state(int channel) {
    return channel >= 0 ? M5.Speaker.isPlaying(channel) : 0;
}

void m5u_speaker_stop(int channel) {
    if (channel < 0) { M5.Speaker.stop(); } else { M5.Speaker.stop(channel); }
}

uint8_t m5u_speaker_get_channel_volume(int channel) {
    return M5.Speaker.getChannelVolume(channel);
}

void m5u_speaker_set_channel_volume(int channel, uint8_t volume) {
    M5.Speaker.setChannelVolume(channel, volume);
}

void m5u_speaker_set_all_channel_volume(uint8_t volume) {
    M5.Speaker.setAllChannelVolume(volume);
}

bool m5u_imu_is_enabled(void) {
    return M5.Imu.isEnabled();
}

int m5u_imu_get_type(void) {
    return (int)M5.Imu.getType();
}

bool m5u_imu_update(void) {
    return M5.Imu.update();
}

int m5u_imu_update_mask(void) {
    return (int)M5.Imu.update();
}

bool m5u_imu_sleep(void) {
    return M5.Imu.sleep();
}

void m5u_imu_set_clock(uint32_t freq) {
    M5.Imu.setClock(freq);
}

bool m5u_imu_set_axis_order(int axis0, int axis1, int axis2) {
    return M5.Imu.setAxisOrder(
        (m5::IMU_Class::axis_t)axis0,
        (m5::IMU_Class::axis_t)axis1,
        (m5::IMU_Class::axis_t)axis2);
}

bool m5u_imu_set_axis_order_right_handed(int axis0, int axis1) {
    return M5.Imu.setAxisOrderRightHanded(
        (m5::IMU_Class::axis_t)axis0,
        (m5::IMU_Class::axis_t)axis1);
}

bool m5u_imu_set_axis_order_left_handed(int axis0, int axis1) {
    return M5.Imu.setAxisOrderLeftHanded(
        (m5::IMU_Class::axis_t)axis0,
        (m5::IMU_Class::axis_t)axis1);
}

bool m5u_imu_set_int_pin_active_logic(bool level) {
    return M5.Imu.setINTPinActiveLogic(level);
}

bool m5u_imu_load_offset_from_nvs(void) {
    return M5.Imu.loadOffsetFromNVS();
}

bool m5u_imu_save_offset_to_nvs(void) {
    return M5.Imu.saveOffsetToNVS();
}

float m5u_imu_get_offset_data(int index) {
    return M5.Imu.getOffsetData(index);
}

void m5u_imu_set_calibration(float x, float y, float z) {
    M5.Imu.setCalibration(x, y, z);
}

void m5u_imu_set_calibration_strength(uint8_t accel, uint8_t gyro, uint8_t mag) {
    M5.Imu.setCalibration(accel, gyro, mag);
}

void m5u_imu_clear_offset_data(void) {
    M5.Imu.clearOffsetData();
}

void m5u_imu_set_offset_data(size_t index, int32_t value) {
    M5.Imu.setOffsetData(index, value);
}

int32_t m5u_imu_get_offset_data_i32(size_t index) {
    return M5.Imu.getOffsetData(index);
}

int16_t m5u_imu_get_raw_data(size_t index) {
    return M5.Imu.getRawData(index);
}

static m5::IMU_Base* m5u_imu_device_for_kind(int kind) {
    static m5::AK8963_Class ak8963;
    static m5::BMM150_Class bmm150;
    static m5::BMI270_Class bmi270;
    static m5::MPU6886_Class mpu6886;
    static m5::SH200Q_Class sh200q;

    switch (kind) {
        case 0: return &ak8963;
        case 1: return &bmm150;
        case 2: return &bmi270;
        case 3: return &mpu6886;
        case 4: return &sh200q;
        default: return nullptr;
    }
}

int m5u_imu_device_begin(int kind) {
    auto imu = m5u_imu_device_for_kind(kind);
    return imu ? imu->begin() : 0;
}

bool m5u_imu_device_get_raw_data(int kind, m5u_imu_raw_data_t* out) {
    auto imu = m5u_imu_device_for_kind(kind);
    if (!imu || !out) {
        return false;
    }
    m5::IMU_Base::imu_raw_data_t raw = {};
    auto mask = imu->getImuRawData(&raw);
    out->accel_x = raw.accel.x;
    out->accel_y = raw.accel.y;
    out->accel_z = raw.accel.z;
    out->gyro_x = raw.gyro.x;
    out->gyro_y = raw.gyro.y;
    out->gyro_z = raw.gyro.z;
    out->mag_x = raw.mag.x;
    out->mag_y = raw.mag.y;
    out->mag_z = raw.mag.z;
    out->temp = raw.temp;
    out->sensor_mask = static_cast<uint8_t>(mask);
    return mask != m5::IMU_Base::imu_spec_none;
}

bool m5u_imu_device_get_convert_param(int kind, m5u_imu_convert_param_t* out) {
    auto imu = m5u_imu_device_for_kind(kind);
    if (!imu || !out) {
        return false;
    }
    m5::IMU_Base::imu_convert_param_t param;
    imu->getConvertParam(&param);
    out->accel_res = param.accel_res;
    out->gyro_res = param.gyro_res;
    out->mag_res = param.mag_res;
    out->temp_res = param.temp_res;
    out->temp_offset = param.temp_offset;
    return true;
}

bool m5u_imu_device_get_temp_adc(int kind, int16_t* adc) {
    auto imu = m5u_imu_device_for_kind(kind);
    return imu && adc ? imu->getTempAdc(adc) : false;
}

bool m5u_imu_device_sleep(int kind) {
    auto imu = m5u_imu_device_for_kind(kind);
    return imu ? imu->sleep() : false;
}

bool m5u_imu_device_set_int_pin_active_logic(int kind, bool level) {
    auto imu = m5u_imu_device_for_kind(kind);
    return imu ? imu->setINTPinActiveLogic(level) : false;
}

int m5u_imu_device_who_am_i(int kind) {
    switch (kind) {
        case 0: return static_cast<m5::AK8963_Class*>(m5u_imu_device_for_kind(kind))->WhoAmI();
        case 1: return static_cast<m5::BMM150_Class*>(m5u_imu_device_for_kind(kind))->WhoAmI();
        case 2: return static_cast<m5::BMI270_Class*>(m5u_imu_device_for_kind(kind))->WhoAmI();
        case 3: return static_cast<m5::MPU6886_Class*>(m5u_imu_device_for_kind(kind))->whoAmI();
        case 4: return static_cast<m5::SH200Q_Class*>(m5u_imu_device_for_kind(kind))->WhoAmI();
        default:
            return -1;
    }
}

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
    if (!out) { return false; }
    auto d = M5.Touch.getDetail(index);
    out->x = d.x;
    out->y = d.y;
    out->prev_x = d.prev_x;
    out->prev_y = d.prev_y;
    out->base_x = d.base_x;
    out->base_y = d.base_y;
    out->base_msec = d.base_msec;
    out->state = static_cast<uint8_t>(d.state);
    out->is_pressed = d.isPressed();
    out->was_pressed = d.wasPressed();
    out->was_released = d.wasReleased();
    out->was_clicked = d.wasClicked();
    out->was_hold = d.wasHold();
    out->is_holding = d.isHolding();
    out->click_count = d.getClickCount();
    return true;
}

bool m5u_rtc_is_enabled(void) {
    return M5.Rtc.isEnabled();
}

static bool m5u_has_axp2101(void) {
#if M5U_HAS_AXP2101
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_axp2101;
#else
    return false;
#endif
}

static bool m5u_has_axp192(void) {
#if M5U_HAS_AXP192
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_axp192;
#else
    return false;
#endif
}

static bool m5u_has_aw32001(void) {
#if M5U_HAS_AW32001
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_aw32001;
#else
    return false;
#endif
}

static bool m5u_has_bq27220(void) {
#if M5U_HAS_BQ27220
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_aw32001;
#else
    return false;
#endif
}

static bool m5u_has_py32pmic(void) {
#if M5U_HAS_PY32PMIC
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_py32pmic;
#else
    return false;
#endif
}

static bool m5u_has_ip5306(void) {
#if M5U_HAS_IP5306
    return M5.Power.getType() == m5::Power_Class::pmic_t::pmic_ip5306;
#else
    return false;
#endif
}

bool m5u_power_axp192_begin(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.begin();
#else
    return false;
#endif
}

int m5u_power_axp192_get_battery_level(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getBatteryLevel() : -1;
#else
    return -1;
#endif
}

bool m5u_power_axp192_set_battery_charge(bool enable) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setBatteryCharge(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp192_set_charge_current(uint16_t max_ma) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setChargeCurrent(max_ma);
    return true;
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_axp192_set_charge_voltage(uint16_t max_mv) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setChargeVoltage(max_mv);
    return true;
#else
    (void)max_mv;
    return false;
#endif
}

bool m5u_power_axp192_is_charging(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.isCharging();
#else
    return false;
#endif
}

bool m5u_power_axp192_set_dcdc(uint8_t channel, int voltage_mv) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    switch (channel) {
    case 1: M5.Power.Axp192.setDCDC1(voltage_mv); return true;
    case 2: M5.Power.Axp192.setDCDC2(voltage_mv); return true;
    case 3: M5.Power.Axp192.setDCDC3(voltage_mv); return true;
    default: return false;
    }
#else
    (void)channel;
    (void)voltage_mv;
    return false;
#endif
}

bool m5u_power_axp192_set_ldo(uint8_t channel, int voltage_mv) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    switch (channel) {
    case 0: M5.Power.Axp192.setLDO0(voltage_mv); return true;
    case 2: M5.Power.Axp192.setLDO2(voltage_mv); return true;
    case 3: M5.Power.Axp192.setLDO3(voltage_mv); return true;
    default: return false;
    }
#else
    (void)channel;
    (void)voltage_mv;
    return false;
#endif
}

bool m5u_power_axp192_set_gpio(uint8_t gpio_num, bool state) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192() || gpio_num > 4) {
        return false;
    }
    M5.Power.Axp192.setGPIO(gpio_num, state);
    return true;
#else
    (void)gpio_num;
    (void)state;
    return false;
#endif
}

bool m5u_power_axp192_power_off(void) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.powerOff();
    return true;
#else
    return false;
#endif
}

bool m5u_power_axp192_set_adc_state(bool enable) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setAdcState(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp192_set_adc_rate(uint8_t rate) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setAdcRate(rate);
    return true;
#else
    (void)rate;
    return false;
#endif
}

bool m5u_power_axp192_set_exten(bool enable) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setEXTEN(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp192_set_backup(bool enable) {
#if M5U_HAS_AXP192
    if (!m5u_has_axp192()) {
        return false;
    }
    M5.Power.Axp192.setBACKUP(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp192_is_acin(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.isACIN();
#else
    return false;
#endif
}

bool m5u_power_axp192_is_vbus(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.isVBUS();
#else
    return false;
#endif
}

bool m5u_power_axp192_get_bat_state(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.getBatState();
#else
    return false;
#endif
}

bool m5u_power_axp192_get_exten(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() && M5.Power.Axp192.getEXTEN();
#else
    return false;
#endif
}

float m5u_power_axp192_get_battery_voltage_v(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getBatteryVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_battery_discharge_current_ma(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getBatteryDischargeCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_battery_charge_current_ma(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getBatteryChargeCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_battery_power_mw(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getBatteryPower() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_acin_voltage_v(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getACINVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_acin_current_ma(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getACINCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_vbus_voltage_v(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getVBUSVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_vbus_current_ma(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getVBUSCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_aps_voltage_v(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getAPSVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp192_get_internal_temperature_c(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getInternalTemperature() : 0.0f;
#else
    return 0.0f;
#endif
}

uint8_t m5u_power_axp192_get_pek_press(void) {
#if M5U_HAS_AXP192
    return m5u_has_axp192() ? M5.Power.Axp192.getPekPress() : 0;
#else
    return 0;
#endif
}

bool m5u_power_aw32001_begin(void) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() && M5.Power.Aw32001.begin();
#else
    return false;
#endif
}

bool m5u_power_aw32001_set_battery_charge(bool enable) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() && M5.Power.Aw32001.setBatteryCharge(enable);
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_aw32001_set_charge_current(uint16_t max_ma) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() && M5.Power.Aw32001.setChargeCurrent(max_ma);
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_aw32001_set_charge_voltage(uint16_t max_mv) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() && M5.Power.Aw32001.setChargeVoltage(max_mv);
#else
    (void)max_mv;
    return false;
#endif
}

bool m5u_power_aw32001_is_charging(void) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() && M5.Power.Aw32001.isCharging();
#else
    return false;
#endif
}

uint16_t m5u_power_aw32001_get_charge_current(void) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() ? M5.Power.Aw32001.getChargeCurrent() : 0;
#else
    return 0;
#endif
}

uint16_t m5u_power_aw32001_get_charge_voltage(void) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() ? M5.Power.Aw32001.getChargeVoltage() : 0;
#else
    return 0;
#endif
}

int m5u_power_aw32001_get_charge_status(void) {
#if M5U_HAS_AW32001
    return m5u_has_aw32001() ? M5.Power.Aw32001.getChargeStatus() : -1;
#else
    return -1;
#endif
}

bool m5u_power_bq27220_begin(void) {
#if M5U_HAS_BQ27220
    return m5u_has_bq27220() && M5.Power.Bq27220.begin();
#else
    return false;
#endif
}

int16_t m5u_power_bq27220_get_current_ma(void) {
#if M5U_HAS_BQ27220
    return m5u_has_bq27220() ? M5.Power.Bq27220.getCurrent_mA() : 0;
#else
    return 0;
#endif
}

int16_t m5u_power_bq27220_get_voltage_mv(void) {
#if M5U_HAS_BQ27220
    return m5u_has_bq27220() ? M5.Power.Bq27220.getVoltage_mV() : 0;
#else
    return 0;
#endif
}

float m5u_power_bq27220_get_current_a(void) {
#if M5U_HAS_BQ27220
    return m5u_has_bq27220() ? M5.Power.Bq27220.getCurrent_F() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_bq27220_get_voltage_v(void) {
#if M5U_HAS_BQ27220
    return m5u_has_bq27220() ? M5.Power.Bq27220.getVoltage_F() : 0.0f;
#else
    return 0.0f;
#endif
}

bool m5u_power_ina226_begin(void) {
#if M5U_HAS_INA226
    return M5.Power.Ina226.begin();
#else
    return false;
#endif
}

bool m5u_power_ina226_config(const m5u_power_ina226_config_t* config) {
#if M5U_HAS_INA226
    if (!config) {
        return false;
    }

    m5::INA226_Class::config_t cfg;
    cfg.shunt_res = config->shunt_res;
    cfg.max_expected_current = config->max_expected_current;
    cfg.sampling_rate = static_cast<m5::INA226_Class::Sampling>(config->sampling_rate & 0x07);
    cfg.shunt_conversion_time = static_cast<m5::INA226_Class::ConversionTime>(config->shunt_conversion_time & 0x07);
    cfg.bus_conversion_time = static_cast<m5::INA226_Class::ConversionTime>(config->bus_conversion_time & 0x07);
    cfg.mode = static_cast<m5::INA226_Class::Mode>(config->mode & 0x07);
    M5.Power.Ina226.config(cfg);
    return true;
#else
    (void)config;
    return false;
#endif
}

float m5u_power_ina226_get_bus_voltage_v(void) {
#if M5U_HAS_INA226
    return M5.Power.Ina226.getBusVoltage();
#else
    return 0.0f;
#endif
}

float m5u_power_ina226_get_shunt_voltage_v(void) {
#if M5U_HAS_INA226
    return M5.Power.Ina226.getShuntVoltage();
#else
    return 0.0f;
#endif
}

float m5u_power_ina226_get_shunt_current_a(void) {
#if M5U_HAS_INA226
    return M5.Power.Ina226.getShuntCurrent();
#else
    return 0.0f;
#endif
}

float m5u_power_ina226_get_power_w(void) {
#if M5U_HAS_INA226
    return M5.Power.Ina226.getPower();
#else
    return 0.0f;
#endif
}

static bool m5u_power_ina3221_valid_index(size_t index) {
#if M5U_HAS_INA3221
    return index < 2;
#else
    (void)index;
    return false;
#endif
}

bool m5u_power_ina3221_begin(size_t index) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) && M5.Power.Ina3221[index].begin();
#else
    (void)index;
    return false;
#endif
}

float m5u_power_ina3221_get_bus_voltage_v(size_t index, uint8_t channel) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) ? M5.Power.Ina3221[index].getBusVoltage(channel) : 0.0f;
#else
    (void)index;
    (void)channel;
    return 0.0f;
#endif
}

float m5u_power_ina3221_get_shunt_voltage_v(size_t index, uint8_t channel) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) ? M5.Power.Ina3221[index].getShuntVoltage(channel) : 0.0f;
#else
    (void)index;
    (void)channel;
    return 0.0f;
#endif
}

float m5u_power_ina3221_get_current_a(size_t index, uint8_t channel) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) ? M5.Power.Ina3221[index].getCurrent(channel) : 0.0f;
#else
    (void)index;
    (void)channel;
    return 0.0f;
#endif
}

int32_t m5u_power_ina3221_get_bus_voltage_mv(size_t index, uint8_t channel) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) ? M5.Power.Ina3221[index].getBusMilliVoltage(channel) : 0;
#else
    (void)index;
    (void)channel;
    return 0;
#endif
}

int32_t m5u_power_ina3221_get_shunt_voltage_mv(size_t index, uint8_t channel) {
#if M5U_HAS_INA3221
    return m5u_power_ina3221_valid_index(index) ? M5.Power.Ina3221[index].getShuntMilliVoltage(channel) : 0;
#else
    (void)index;
    (void)channel;
    return 0;
#endif
}

bool m5u_power_ina3221_set_shunt_res(size_t index, uint8_t channel, uint32_t res) {
#if M5U_HAS_INA3221
    if (!m5u_power_ina3221_valid_index(index)) {
        return false;
    }
    M5.Power.Ina3221[index].setShuntRes(channel, res);
    return channel < 3;
#else
    (void)index;
    (void)channel;
    (void)res;
    return false;
#endif
}

bool m5u_power_ip5306_begin(void) {
#if M5U_HAS_IP5306
    return m5u_has_ip5306() && M5.Power.Ip5306.begin();
#else
    return false;
#endif
}

int m5u_power_ip5306_get_battery_level(void) {
#if M5U_HAS_IP5306
    return m5u_has_ip5306() ? M5.Power.Ip5306.getBatteryLevel() : -1;
#else
    return -1;
#endif
}

bool m5u_power_ip5306_set_battery_charge(bool enable) {
#if M5U_HAS_IP5306
    if (!m5u_has_ip5306()) {
        return false;
    }
    M5.Power.Ip5306.setBatteryCharge(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_ip5306_set_charge_current(uint16_t max_ma) {
#if M5U_HAS_IP5306
    if (!m5u_has_ip5306()) {
        return false;
    }
    M5.Power.Ip5306.setChargeCurrent(max_ma);
    return true;
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_ip5306_set_charge_voltage(uint16_t max_mv) {
#if M5U_HAS_IP5306
    if (!m5u_has_ip5306()) {
        return false;
    }
    M5.Power.Ip5306.setChargeVoltage(max_mv);
    return true;
#else
    (void)max_mv;
    return false;
#endif
}

bool m5u_power_ip5306_is_charging(void) {
#if M5U_HAS_IP5306
    return m5u_has_ip5306() && M5.Power.Ip5306.isCharging();
#else
    return false;
#endif
}

bool m5u_power_ip5306_set_power_boost_keep_on(bool enable) {
#if M5U_HAS_IP5306
    return m5u_has_ip5306() && M5.Power.Ip5306.setPowerBoostKeepOn(enable);
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_py32pmic_begin(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.begin();
#else
    return false;
#endif
}

bool m5u_power_py32pmic_set_ext_output(bool enable) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.setExtOutput(enable);
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_py32pmic_set_battery_charge(bool enable) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.setBatteryCharge(enable);
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_py32pmic_set_charge_current(uint16_t max_ma) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.setChargeCurrent(max_ma);
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_py32pmic_set_charge_voltage(uint16_t max_mv) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.setChargeVoltage(max_mv);
#else
    (void)max_mv;
    return false;
#endif
}

bool m5u_power_py32pmic_is_charging(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.isCharging();
#else
    return false;
#endif
}

uint16_t m5u_power_py32pmic_get_charge_current(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() ? M5.Power.PY32pmic.getChargeCurrent() : 0;
#else
    return 0;
#endif
}

uint16_t m5u_power_py32pmic_get_charge_voltage(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() ? M5.Power.PY32pmic.getChargeVoltage() : 0;
#else
    return 0;
#endif
}

uint8_t m5u_power_py32pmic_get_pek_press(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() ? M5.Power.PY32pmic.getPekPress() : 0;
#else
    return 0;
#endif
}

bool m5u_power_py32pmic_power_off(void) {
#if M5U_HAS_PY32PMIC
    return m5u_has_py32pmic() && M5.Power.PY32pmic.powerOff();
#else
    return false;
#endif
}

bool m5u_power_axp2101_begin(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.begin();
#else
    return false;
#endif
}

int m5u_power_axp2101_get_battery_level(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getBatteryLevel() : -1;
#else
    return -1;
#endif
}

bool m5u_power_axp2101_set_battery_charge(bool enable) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setBatteryCharge(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp2101_set_pre_charge_current(uint16_t max_ma) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setPreChargeCurrent(max_ma);
    return true;
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_axp2101_set_charge_current(uint16_t max_ma) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setChargeCurrent(max_ma);
    return true;
#else
    (void)max_ma;
    return false;
#endif
}

bool m5u_power_axp2101_set_charge_voltage(uint16_t max_mv) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setChargeVoltage(max_mv);
    return true;
#else
    (void)max_mv;
    return false;
#endif
}

int m5u_power_axp2101_get_charge_status(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getChargeStatus() : -2;
#else
    return -2;
#endif
}

bool m5u_power_axp2101_is_charging(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isCharging();
#else
    return false;
#endif
}

bool m5u_power_axp2101_set_ldo(int kind, int channel, int voltage_mv) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }

    switch (kind) {
    case 0:
        switch (channel) {
        case 1: M5.Power.Axp2101.setALDO1(voltage_mv); return true;
        case 2: M5.Power.Axp2101.setALDO2(voltage_mv); return true;
        case 3: M5.Power.Axp2101.setALDO3(voltage_mv); return true;
        case 4: M5.Power.Axp2101.setALDO4(voltage_mv); return true;
        default: return false;
        }
    case 1:
        switch (channel) {
        case 1: M5.Power.Axp2101.setBLDO1(voltage_mv); return true;
        case 2: M5.Power.Axp2101.setBLDO2(voltage_mv); return true;
        default: return false;
        }
    case 2:
        switch (channel) {
        case 1: M5.Power.Axp2101.setDLDO1(voltage_mv); return true;
        case 2: M5.Power.Axp2101.setDLDO2(voltage_mv); return true;
        default: return false;
        }
    default:
        return false;
    }
#else
    (void)kind;
    (void)channel;
    (void)voltage_mv;
    return false;
#endif
}

bool m5u_power_axp2101_get_ldo_enabled(int kind, int channel) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }

    switch (kind) {
    case 0:
        switch (channel) {
        case 1: return M5.Power.Axp2101.getALDO1Enabled();
        case 2: return M5.Power.Axp2101.getALDO2Enabled();
        case 3: return M5.Power.Axp2101.getALDO3Enabled();
        case 4: return M5.Power.Axp2101.getALDO4Enabled();
        default: return false;
        }
    case 1:
        switch (channel) {
        case 1: return M5.Power.Axp2101.getBLDO1Enabled();
        case 2: return M5.Power.Axp2101.getBLDO2Enabled();
        default: return false;
        }
    default:
        return false;
    }
#else
    (void)kind;
    (void)channel;
    return false;
#endif
}

bool m5u_power_axp2101_power_off(void) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.powerOff();
    return true;
#else
    return false;
#endif
}

bool m5u_power_axp2101_set_adc_state(bool enable) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setAdcState(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp2101_set_adc_rate(uint8_t rate) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setAdcRate(rate);
    return true;
#else
    (void)rate;
    return false;
#endif
}

bool m5u_power_axp2101_set_backup(bool enable) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.setBACKUP(enable);
    return true;
#else
    (void)enable;
    return false;
#endif
}

bool m5u_power_axp2101_is_acin(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isACIN();
#else
    return false;
#endif
}

bool m5u_power_axp2101_is_vbus(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isVBUS();
#else
    return false;
#endif
}

bool m5u_power_axp2101_get_bat_state(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.getBatState();
#else
    return false;
#endif
}

float m5u_power_axp2101_get_battery_voltage_v(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getBatteryVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_battery_discharge_current_ma(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getBatteryDischargeCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_battery_charge_current_ma(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getBatteryChargeCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_battery_power_mw(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getBatteryPower() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_acin_voltage_v(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getACINVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_acin_current_ma(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getACINCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_vbus_voltage_v(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getVBUSVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_vbus_current_ma(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getVBUSCurrent() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_ts_voltage_v(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getTSVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_aps_voltage_v(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getAPSVoltage() : 0.0f;
#else
    return 0.0f;
#endif
}

float m5u_power_axp2101_get_internal_temperature_c(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getInternalTemperature() : 0.0f;
#else
    return 0.0f;
#endif
}

uint8_t m5u_power_axp2101_get_pek_press(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getPekPress() : 0;
#else
    return 0;
#endif
}

bool m5u_power_axp2101_disable_irq(uint64_t mask) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.disableIRQ(mask);
#else
    (void)mask;
    return false;
#endif
}

bool m5u_power_axp2101_enable_irq(uint64_t mask) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.enableIRQ(mask);
#else
    (void)mask;
    return false;
#endif
}

bool m5u_power_axp2101_clear_irq_statuses(void) {
#if M5U_HAS_AXP2101
    if (!m5u_has_axp2101()) {
        return false;
    }
    M5.Power.Axp2101.clearIRQStatuses();
    return true;
#else
    return false;
#endif
}

uint64_t m5u_power_axp2101_get_irq_statuses(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() ? M5.Power.Axp2101.getIRQStatuses() : 0;
#else
    return 0;
#endif
}

bool m5u_power_axp2101_is_bat_charger_under_temperature_irq(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isBatChargerUnderTemperatureIrq();
#else
    return false;
#endif
}

bool m5u_power_axp2101_is_bat_charger_over_temperature_irq(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isBatChargerOverTemperatureIrq();
#else
    return false;
#endif
}

bool m5u_power_axp2101_is_vbus_insert_irq(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isVbusInsertIrq();
#else
    return false;
#endif
}

bool m5u_power_axp2101_is_vbus_remove_irq(void) {
#if M5U_HAS_AXP2101
    return m5u_has_axp2101() && M5.Power.Axp2101.isVbusRemoveIrq();
#else
    return false;
#endif
}

bool m5u_led_begin(void) {
    return M5.Led.begin();
}

void m5u_led_display(void) {
    M5.Led.display();
}

void m5u_led_set_auto_display(bool enable) {
    M5.Led.setAutoDisplay(enable);
}

size_t m5u_led_count(void) {
    return M5.Led.getCount();
}

void m5u_led_set_brightness(uint8_t brightness) {
    M5.Led.setBrightness(brightness);
}

void m5u_led_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
    M5.Led.setColor(index, r, g, b);
}

void m5u_led_set_all_color_rgb(uint8_t r, uint8_t g, uint8_t b) {
    M5.Led.setAllColor(RGBColor{r, g, b});
}

void m5u_led_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
    if (!colors || !length) {
        return;
    }
    std::vector<RGBColor> rgb;
    rgb.reserve(length);
    for (size_t i = 0; i < length; ++i) {
        rgb.push_back(RGBColor{colors[i].r, colors[i].g, colors[i].b});
    }
    M5.Led.setColors(rgb.data(), index, length);
}

int m5u_led_get_type(size_t index) {
    return (int)M5.Led.getLedType(index);
}

bool m5u_led_is_enabled(void) {
    return M5.Led.isEnabled();
}

#if defined(CONFIG_IDF_TARGET_ESP32S3)
static m5::LED_PowerHub_Class& m5u_led_power_hub(void) {
    static m5::LED_PowerHub_Class led;
    return led;
}
#endif

bool m5u_led_power_hub_begin(void) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    return m5u_led_power_hub().begin();
#else
    return false;
#endif
}

size_t m5u_led_power_hub_count(void) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    return m5u_led_power_hub().getCount();
#else
    return 0;
#endif
}

void m5u_led_power_hub_set_brightness(uint8_t brightness) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    m5u_led_power_hub().setBrightness(brightness);
#else
    (void)brightness;
#endif
}

void m5u_led_power_hub_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    if (index < m5u_led_power_hub().getCount()) {
        RGBColor color{r, g, b};
        m5u_led_power_hub().setColors(&color, index, 1);
    }
#else
    (void)index; (void)r; (void)g; (void)b;
#endif
}

void m5u_led_power_hub_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    if (!colors || !length) {
        return;
    }
    std::vector<RGBColor> rgb;
    rgb.reserve(length);
    for (size_t i = 0; i < length; ++i) {
        rgb.push_back(RGBColor{colors[i].r, colors[i].g, colors[i].b});
    }
    m5u_led_power_hub().setColors(rgb.data(), index, length);
#else
    (void)colors; (void)index; (void)length;
#endif
}

void m5u_led_power_hub_display(void) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    m5u_led_power_hub().display();
#endif
}

int m5u_led_power_hub_get_type(size_t index) {
#if defined(CONFIG_IDF_TARGET_ESP32S3)
    return (int)m5u_led_power_hub().getLedType(index);
#else
    (void)index;
    return 0;
#endif
}

#if M5U_HAS_LED_STRIP_RMT
static m5::LED_Strip_Class& m5u_led_strip(void) {
    static m5::LED_Strip_Class led;
    return led;
}

static std::shared_ptr<m5::LedBus_RMT>& m5u_led_strip_rmt_bus(void) {
    static std::shared_ptr<m5::LedBus_RMT> bus;
    return bus;
}

static m5::LED_Strip_Class::config_t::color_order_t m5u_led_strip_color_order(int color_order) {
    using color_order_t = m5::LED_Strip_Class::config_t::color_order_t;
    switch (color_order) {
        case 0: return color_order_t::color_order_rgb;
        case 1: return color_order_t::color_order_rbg;
        case 2: return color_order_t::color_order_grb;
        case 3: return color_order_t::color_order_gbr;
        case 4: return color_order_t::color_order_brg;
        case 5: return color_order_t::color_order_bgr;
        default: return color_order_t::color_order_grb;
    }
}
#endif

bool m5u_led_strip_set_config(const m5u_led_strip_config_t* config) {
#if M5U_HAS_LED_STRIP_RMT
    if (!config) {
        return false;
    }
    m5::LED_Strip_Class::config_t cfg;
    cfg.led_count = config->led_count;
    cfg.color_order = m5u_led_strip_color_order(config->color_order);
    cfg.byte_per_led = config->byte_per_led;
    m5u_led_strip().setConfig(cfg);
    return true;
#else
    (void)config;
    return false;
#endif
}

bool m5u_led_strip_set_rmt_bus_config(const m5u_led_strip_rmt_config_t* config) {
#if M5U_HAS_LED_STRIP_RMT
    if (!config) {
        return false;
    }
    auto& bus = m5u_led_strip_rmt_bus();
    if (bus) {
        bus->release();
    }
    bus = std::make_shared<m5::LedBus_RMT>();
    auto cfg = bus->config();
    cfg.frequency = config->frequency;
    cfg.t0h_ns = config->t0h_ns;
    cfg.t0l_ns = config->t0l_ns;
    cfg.t1h_ns = config->t1h_ns;
    cfg.t1l_ns = config->t1l_ns;
    cfg.reset_us = config->reset_us;
    cfg.pin_data = config->pin_data;
    bus->setConfig(cfg);
    m5u_led_strip().setBus(bus);
    return true;
#else
    (void)config;
    return false;
#endif
}

bool m5u_led_strip_begin(void) {
#if M5U_HAS_LED_STRIP_RMT
    return m5u_led_strip().begin();
#else
    return false;
#endif
}

size_t m5u_led_strip_count(void) {
#if M5U_HAS_LED_STRIP_RMT
    return m5u_led_strip().getCount();
#else
    return 0;
#endif
}

void m5u_led_strip_set_brightness(uint8_t brightness) {
#if M5U_HAS_LED_STRIP_RMT
    m5u_led_strip().setBrightness(brightness);
#else
    (void)brightness;
#endif
}

void m5u_led_strip_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
#if M5U_HAS_LED_STRIP_RMT
    if (index < m5u_led_strip().getCount()) {
        RGBColor color{r, g, b};
        m5u_led_strip().setColors(&color, index, 1);
    }
#else
    (void)index; (void)r; (void)g; (void)b;
#endif
}

void m5u_led_strip_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
#if M5U_HAS_LED_STRIP_RMT
    auto& led = m5u_led_strip();
    const size_t count = led.getCount();
    if (!colors || !length || index >= count) {
        return;
    }
    const size_t available = count - index;
    if (length > available) {
        length = available;
    }
    std::vector<RGBColor> rgb;
    rgb.reserve(length);
    for (size_t i = 0; i < length; ++i) {
        rgb.push_back(RGBColor{colors[i].r, colors[i].g, colors[i].b});
    }
    led.setColors(rgb.data(), index, length);
#else
    (void)colors; (void)index; (void)length;
#endif
}

void m5u_led_strip_display(void) {
#if M5U_HAS_LED_STRIP_RMT
    m5u_led_strip().display();
#endif
}

int m5u_led_strip_get_type(size_t index) {
#if M5U_HAS_LED_STRIP_RMT
    return (int)m5u_led_strip().getLedType(index);
#else
    (void)index;
    return 0;
#endif
}

void m5u_log_print(const char* text) {
    M5.Log.print(text);
}

void m5u_log_println_empty(void) {
    M5.Log.println();
}

void m5u_log_level(int level, const char* text) {
    M5.Log((esp_log_level_t)level, "%s", text);
}

void m5u_log_dump(const void* addr, uint32_t len, int level) {
    M5.Log.dump(addr, len, (esp_log_level_t)level);
}

const char* m5u_log_path_to_file_name(const char* path) {
    return path ? m5::Log_Class::pathToFileName(path) : nullptr;
}

bool m5u_log_set_callback(m5u_log_callback_t callback, void* user_data) {
    s_m5u_log_callback = callback;
    s_m5u_log_callback_user_data = user_data;
    if (!callback) {
        M5.Log.setCallback(nullptr);
        return true;
    }
    M5.Log.setCallback([](esp_log_level_t level, bool use_color, const char* text) {
        if (s_m5u_log_callback) {
            s_m5u_log_callback((int)level, use_color, text, s_m5u_log_callback_user_data);
        }
    });
    return true;
}

static bool m5u_log_valid_target(int target) {
    return target >= m5::log_target_serial && target < m5::log_target_max;
}

static bool m5u_log_valid_level(int level) {
    return level >= ESP_LOG_NONE && level <= ESP_LOG_VERBOSE;
}

bool m5u_log_set_enable_color(int target, bool enable) {
    if (!m5u_log_valid_target(target)) {
        return false;
    }
    M5.Log.setEnableColor((m5::log_target_t)target, enable);
    return true;
}

bool m5u_log_get_enable_color(int target) {
    if (!m5u_log_valid_target(target)) {
        return false;
    }
    return M5.Log.getEnableColor((m5::log_target_t)target);
}

bool m5u_log_set_level(int target, int level) {
    if (!m5u_log_valid_target(target) || !m5u_log_valid_level(level)) {
        return false;
    }
    M5.Log.setLogLevel((m5::log_target_t)target, (esp_log_level_t)level);
    return true;
}

int m5u_log_get_level(int target) {
    if (!m5u_log_valid_target(target)) {
        return -1;
    }
    return (int)M5.Log.getLogLevel((m5::log_target_t)target);
}

bool m5u_log_set_suffix(int target, const char* suffix) {
    if (!m5u_log_valid_target(target) || !suffix) {
        return false;
    }
    s_m5u_log_suffixes[target] = suffix;
    M5.Log.setSuffix((m5::log_target_t)target, s_m5u_log_suffixes[target].c_str());
    return true;
}

} // extern "C"
