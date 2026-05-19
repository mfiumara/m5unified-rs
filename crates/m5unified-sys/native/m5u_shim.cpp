#include "m5u_shim.h"

#include <M5Unified.h>
#include <driver/gpio.h>
#include <driver/sdspi_host.h>
#include <driver/spi_common.h>
#include <esp_err.h>
#include <esp_vfs_fat.h>
#include <sdmmc_cmd.h>
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

#if defined(CONFIG_IDF_TARGET_ESP32C6)
#define M5U_HAS_AW32001 1
#define M5U_HAS_BQ27220 1
#else
#define M5U_HAS_AW32001 0
#define M5U_HAS_BQ27220 0
#endif

#if defined(CONFIG_IDF_TARGET_ESP32S3)
#define M5U_HAS_PY32PMIC 1
#else
#define M5U_HAS_PY32PMIC 0
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
