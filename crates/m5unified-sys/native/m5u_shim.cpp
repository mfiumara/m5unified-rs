#include "m5u_shim.h"

#include <string.h>

#include <driver/ledc.h>

#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
#include <Arduino.h>
#endif

#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
#include <Arduino.h>
#endif

#include <M5Unified.h>

#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
#include <M5Cardputer.h>
#endif

#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
#include <SD.h>
#include <SPI.h>
#endif

#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
#include <Arduino.h>
#include <SPI.h>
#endif

#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
#include <Wire.h>
#endif

#ifdef M5UNIFIED_RS_USE_ARDUINO_IRREMOTE
#ifndef DISABLE_CODE_FOR_RECEIVER
#define DISABLE_CODE_FOR_RECEIVER
#endif
#ifndef SEND_PWM_BY_TIMER
#define SEND_PWM_BY_TIMER
#endif
#include <IRremote.hpp>
#endif

#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
#include <M5StackChan.h>
#endif

extern "C" {

static constexpr uint8_t M5U_SERVO_MAX_CHANNELS = 8;
static constexpr uint8_t M5U_SERVO_RESOLUTION_BITS = 14;
static constexpr uint32_t M5U_SERVO_DUTY_MAX = (1u << M5U_SERVO_RESOLUTION_BITS) - 1u;

struct m5u_servo_state_t {
    bool attached;
    int pin;
    int timer;
    uint32_t frequency_hz;
    uint16_t min_us;
    uint16_t max_us;
};

static m5u_servo_state_t m5u_servo_states[M5U_SERVO_MAX_CHANNELS] = {};

static bool m5u_servo_valid_channel(int channel) {
    return channel >= 0 && channel < M5U_SERVO_MAX_CHANNELS;
}

static uint32_t m5u_servo_duty_from_pulse_us(uint32_t frequency_hz, uint16_t pulse_us) {
    const uint64_t numerator = static_cast<uint64_t>(pulse_us) * frequency_hz * M5U_SERVO_DUTY_MAX;
    return static_cast<uint32_t>(numerator / 1000000u);
}

#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
static auto m5u_log_target_from_int(int target) -> decltype(m5::log_target_serial) {
    switch (target) {
    case 1: return m5::log_target_display;
    case 2: return m5::log_target_callback;
    default: return m5::log_target_serial;
    }
}
#endif

#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
static auto m5u_config_from_c(const m5u_config_t* config) -> decltype(M5.config()) {
    auto cfg = M5.config();
    if (config == nullptr) {
        return cfg;
    }

    cfg.serial_baudrate = config->serial_baudrate;
    cfg.clear_display = config->clear_display;
    cfg.output_power = config->output_power;
    cfg.pmic_button = config->pmic_button;
    cfg.internal_imu = config->internal_imu;
    cfg.internal_rtc = config->internal_rtc;
    cfg.internal_mic = config->internal_mic;
    cfg.internal_spk = config->internal_spk;
    cfg.external_imu = config->external_imu;
    cfg.external_rtc = config->external_rtc;
    cfg.disable_rtc_irq = config->disable_rtc_irq;
    cfg.led_brightness = config->led_brightness;
    cfg.external_speaker_value = config->external_speaker_value;
    cfg.external_display_value = config->external_display_value;
    return cfg;
}
#endif

bool m5u_begin(void) {
    M5.begin(m5u_config_from_c(nullptr));
    return true;
}

bool m5u_begin_with_config(const m5u_config_t* config) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.begin(m5u_config_from_c(config));
    return true;
#else
    (void)config;
    return false;
#endif
}

void m5u_update(void) {
    M5.update();
}

void m5u_delay_ms(uint32_t ms) {
    M5.delay(ms);
}

uint32_t m5u_millis(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.millis();
#else
    return 0;
#endif
}

uint32_t m5u_micros(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.micros();
#else
    return 0;
#endif
}

uint32_t m5u_get_update_msec(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.getUpdateMsec();
#else
    return 0;
#endif
}

int m5u_get_board(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.getBoard();
#else
    return 0;
#endif
}

int m5u_get_pin(int name) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (name < 0 || name >= m5::pin_name_max) {
        return -1;
    }
    return M5.getPin((m5::pin_name_t)name);
#else
    (void)name;
    return -1;
#endif
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

void m5u_display_draw_pixel(int x, int y, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawPixel(x, y, color);
#else
    (void)x; (void)y; (void)color;
#endif
}

uint16_t m5u_display_read_pixel(int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.readPixel(x, y);
#else
    (void)x; (void)y;
    return 0;
#endif
}

void m5u_display_draw_fast_hline(int x, int y, int w, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawFastHLine(x, y, w, color);
#else
    (void)x; (void)y; (void)w; (void)color;
#endif
}

void m5u_display_draw_fast_vline(int x, int y, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawFastVLine(x, y, h, color);
#else
    (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_display_draw_rect(int x, int y, int w, int h, uint16_t color) {
    M5.Display.drawRect(x, y, w, h, color);
}

void m5u_display_fill_rect(int x, int y, int w, int h, uint16_t color) {
    M5.Display.fillRect(x, y, w, h, color);
}

void m5u_display_fill_rect_alpha(int x, int y, int w, int h, uint8_t alpha, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillRectAlpha(x, y, w, h, alpha, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)alpha; (void)color;
#endif
}

void m5u_display_draw_round_rect(int x, int y, int w, int h, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawRoundRect(x, y, w, h, r, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_display_fill_round_rect(int x, int y, int w, int h, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillRoundRect(x, y, w, h, r, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_display_draw_circle(int x, int y, int r, uint16_t color) {
    M5.Display.drawCircle(x, y, r, color);
}

void m5u_display_fill_circle(int x, int y, int r, uint16_t color) {
    M5.Display.fillCircle(x, y, r, color);
}

void m5u_display_draw_ellipse(int x, int y, int rx, int ry, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawEllipse(x, y, rx, ry, color);
#else
    (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_display_fill_ellipse(int x, int y, int rx, int ry, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillEllipse(x, y, rx, ry, color);
#else
    (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_display_draw_arc(int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawArc(x, y, r0, r1, angle0, angle1, color);
#else
    (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_display_fill_arc(int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillArc(x, y, r0, r1, angle0, angle1, color);
#else
    (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_display_draw_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawTriangle(x0, y0, x1, y1, x2, y2, color);
#else
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_display_fill_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillTriangle(x0, y0, x1, y1, x2, y2, color);
#else
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_display_progress_bar(int x, int y, int w, int h, uint8_t value) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.progressBar(x, y, w, h, value);
#else
    (void)x; (void)y; (void)w; (void)h; (void)value;
#endif
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

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    return M5.Mic.record(buffer, samples);
}

bool m5u_speaker_begin(void) {
    return M5.Speaker.begin();
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

bool m5u_imu_get_accel(float* x, float* y, float* z) {
    return M5.Imu.getAccel(x, y, z);
}

bool m5u_imu_get_gyro(float* x, float* y, float* z) {
    return M5.Imu.getGyro(x, y, z);
}

bool m5u_imu_get_mag(float* x, float* y, float* z) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getMag(x, y, z);
#else
    (void)x; (void)y; (void)z; return false;
#endif
}

bool m5u_imu_get_temp_c(float* temp) {
    return M5.Imu.getTemp(temp);
}

void m5u_touch_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Touch.begin(&M5.Display);
#endif
}

void m5u_touch_update(uint32_t msec) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Touch.update(msec);
#else
    (void)msec;
#endif
}

bool m5u_touch_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Touch.isEnabled();
#else
    return false;
#endif
}

void m5u_touch_end(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Touch.end();
#endif
}

int m5u_touch_count(void) {
    return M5.Touch.getCount();
}

bool m5u_touch_get(int index, int* x, int* y) {
    if (index < 0 || index >= M5.Touch.getCount()) { return false; }
    auto detail = M5.Touch.getDetail(index);
    if (x) { *x = detail.x; }
    if (y) { *y = detail.y; }
    return detail.isPressed();
}

bool m5u_touch_get_raw(int index, m5u_touch_point_t* out) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!out || index < 0 || index >= M5.Touch.getCount()) { return false; }
    auto point = M5.Touch.getTouchPointRaw(static_cast<size_t>(index));
    out->x = point.x;
    out->y = point.y;
    out->size = point.size;
    out->id = point.id;
    return true;
#else
    (void)index; (void)out; return false;
#endif
}

void m5u_touch_set_hold_thresh(uint16_t msec) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Touch.setHoldThresh(msec);
#else
    (void)msec;
#endif
}

void m5u_touch_set_flick_thresh(uint16_t distance) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Touch.setFlickThresh(distance);
#else
    (void)distance;
#endif
}

bool m5u_rtc_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Rtc.begin();
#else
    return false;
#endif
}

bool m5u_rtc_get_volt_low(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Rtc.getVoltLow();
#else
    return false;
#endif
}

bool m5u_rtc_get_date(int* year, int* month, int* day, int* weekday) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_date_t date;
    if (!M5.Rtc.getDate(&date)) { return false; }
    if (year) { *year = date.year; }
    if (month) { *month = date.month; }
    if (day) { *day = date.date; }
    if (weekday) { *weekday = date.weekDay; }
    return true;
#else
    (void)year; (void)month; (void)day; (void)weekday; return false;
#endif
}

bool m5u_rtc_get_time(int* hour, int* minute, int* second) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_time_t time;
    if (!M5.Rtc.getTime(&time)) { return false; }
    if (hour) { *hour = time.hours; }
    if (minute) { *minute = time.minutes; }
    if (second) { *second = time.seconds; }
    return true;
#else
    (void)hour; (void)minute; (void)second; return false;
#endif
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

bool m5u_rtc_set_date(int year, int month, int day, int weekday) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_date_t date;
    date.year = year;
    date.month = month;
    date.date = day;
    date.weekDay = weekday;
    M5.Rtc.setDate(&date);
    return true;
#else
    (void)year; (void)month; (void)day; (void)weekday; return false;
#endif
}

bool m5u_rtc_set_time(int hour, int minute, int second) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_time_t time;
    time.hours = hour;
    time.minutes = minute;
    time.seconds = second;
    M5.Rtc.setTime(&time);
    return true;
#else
    (void)hour; (void)minute; (void)second; return false;
#endif
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

void m5u_rtc_set_system_time_from_rtc(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Rtc.setSystemTimeFromRtc();
#endif
}

bool m5u_rtc_set_alarm_irq_after(int seconds) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Rtc.setAlarmIRQ(seconds);
#else
    (void)seconds; return false;
#endif
}

bool m5u_rtc_get_irq_status(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Rtc.getIRQstatus();
#else
    return false;
#endif
}

void m5u_rtc_clear_irq(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Rtc.clearIRQ();
#endif
}

void m5u_rtc_disable_irq(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Rtc.disableIRQ();
#endif
}

int m5u_battery_level(void) {
    return M5.Power.getBatteryLevel();
}

int m5u_battery_voltage_mv(void) {
    return M5.Power.getBatteryVoltage();
}

int m5u_battery_current_ma(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getBatteryCurrent();
#else
    return 0;
#endif
}

bool m5u_power_is_charging(void) {
    return static_cast<int>(M5.Power.isCharging()) == 1;
}

int m5u_power_charging_state(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<int>(M5.Power.isCharging());
#else
    return 0;
#endif
}

bool m5u_power_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.begin();
#else
    return false;
#endif
}

void m5u_power_set_ext_output(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setExtOutput(enable);
#else
    (void)enable;
#endif
}

bool m5u_power_get_ext_output(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getExtOutput();
#else
    return false;
#endif
}

void m5u_power_set_usb_output(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setUsbOutput(enable);
#else
    (void)enable;
#endif
}

bool m5u_power_get_usb_output(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getUsbOutput();
#else
    return false;
#endif
}

void m5u_power_set_led(uint8_t brightness) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setLed(brightness);
#else
    (void)brightness;
#endif
}

void m5u_power_power_off(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.powerOff();
#endif
}

void m5u_power_timer_sleep(int seconds) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.timerSleep(seconds);
#else
    (void)seconds;
#endif
}

void m5u_power_deep_sleep(uint64_t micro_seconds, bool touch_wakeup) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.deepSleep(micro_seconds, touch_wakeup);
#else
    (void)micro_seconds; (void)touch_wakeup;
#endif
}

void m5u_power_light_sleep(uint64_t micro_seconds, bool touch_wakeup) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.lightSleep(micro_seconds, touch_wakeup);
#else
    (void)micro_seconds; (void)touch_wakeup;
#endif
}

void m5u_power_set_battery_charge(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setBatteryCharge(enable);
#else
    (void)enable;
#endif
}

void m5u_power_set_charge_current(uint16_t max_ma) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setChargeCurrent(max_ma);
#else
    (void)max_ma;
#endif
}

void m5u_power_set_charge_voltage(uint16_t max_mv) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setChargeVoltage(max_mv);
#else
    (void)max_mv;
#endif
}

uint8_t m5u_power_get_key_state(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getKeyState();
#else
    return 0;
#endif
}

void m5u_power_set_vibration(uint8_t level) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Power.setVibration(level);
#else
    (void)level;
#endif
}

int m5u_power_get_type(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<int>(M5.Power.getType());
#else
    return 0;
#endif
}

bool m5u_led_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Led.begin();
#else
    return false;
#endif
}

bool m5u_led_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Led.isEnabled();
#else
    return false;
#endif
}

size_t m5u_led_count(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Led.getCount();
#else
    return 0;
#endif
}

void m5u_led_display(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Led.display();
#endif
}

void m5u_led_set_auto_display(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Led.setAutoDisplay(enable);
#else
    (void)enable;
#endif
}

void m5u_led_set_brightness(uint8_t brightness) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Led.setBrightness(brightness);
#else
    (void)brightness;
#endif
}

void m5u_led_set_color(size_t index, uint32_t rgb) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Led.setColor(index, rgb);
#else
    (void)index; (void)rgb;
#endif
}

void m5u_led_set_all_color(uint32_t rgb) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Led.setAllColor(rgb);
#else
    (void)rgb;
#endif
}

void m5u_log_println(const char* text) {
    M5_LOGI("%s", text);
}

bool m5u_sd_begin(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.begin();
#else
    return false;
#endif
}

void m5u_sd_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    SD.end();
#endif
}

int m5u_sd_card_type(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    switch (SD.cardType()) {
    case CARD_NONE: return 0;
    case CARD_MMC: return 1;
    case CARD_SD: return 2;
    case CARD_SDHC: return 3;
    default: return 4;
    }
#else
    return 0;
#endif
}

uint64_t m5u_sd_card_size_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.cardSize();
#else
    return 0;
#endif
}

uint64_t m5u_sd_total_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.totalBytes();
#else
    return 0;
#endif
}

uint64_t m5u_sd_used_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.usedBytes();
#else
    return 0;
#endif
}

bool m5u_sd_exists(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.exists(path);
#else
    (void)path;
    return false;
#endif
}

uint64_t m5u_sd_file_size(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return 0;
    }

    File file = SD.open(path, FILE_READ);
    if (!file || file.isDirectory()) {
        return 0;
    }

    return file.size();
#else
    (void)path;
    return 0;
#endif
}

bool m5u_sd_is_directory(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }

    File file = SD.open(path, FILE_READ);
    return file && file.isDirectory();
#else
    (void)path;
    return false;
#endif
}

size_t m5u_sd_read_file(const char* path, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || data == nullptr || len == 0) {
        return 0;
    }

    File file = SD.open(path, FILE_READ);
    if (!file || file.isDirectory()) {
        return 0;
    }

    return file.read(data, len);
#else
    (void)path; (void)data; (void)len;
    return 0;
#endif
}

size_t m5u_sd_write_file(const char* path, const uint8_t* data, size_t len, bool append) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || data == nullptr || len == 0) {
        return 0;
    }

    File file = SD.open(path, append ? FILE_APPEND : FILE_WRITE);
    if (!file) {
        return 0;
    }

    return file.write(data, len);
#else
    (void)path; (void)data; (void)len; (void)append;
    return 0;
#endif
}

bool m5u_sd_remove(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.remove(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_sd_mkdir(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.mkdir(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_sd_rmdir(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.rmdir(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_sd_rename(const char* from_path, const char* to_path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (from_path == nullptr || to_path == nullptr) {
        return false;
    }
    return SD.rename(from_path, to_path);
#else
    (void)from_path; (void)to_path;
    return false;
#endif
}

size_t m5u_sd_list_dir(const char* path, m5u_cardputer_sd_dir_entry_t* entries, size_t capacity) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || entries == nullptr || capacity == 0) {
        return 0;
    }

    File root = SD.open(path, FILE_READ);
    if (!root || !root.isDirectory()) {
        return 0;
    }

    size_t count = 0;
    while (count < capacity) {
        File file = root.openNextFile();
        if (!file) {
            break;
        }

        memset(&entries[count], 0, sizeof(entries[count]));
        const char* name = file.name();
        if (name != nullptr) {
            strncpy(entries[count].name, name, M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY - 1);
            entries[count].name[M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY - 1] = '\0';
        }
        entries[count].is_directory = file.isDirectory();
        entries[count].size = entries[count].is_directory ? 0 : file.size();
        ++count;
    }

    return count;
#else
    (void)path; (void)entries; (void)capacity;
    return 0;
#endif
}

bool m5u_i2c_begin(int sda, int scl, uint32_t frequency_hz) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    return Wire.begin(sda, scl, frequency_hz);
#else
    (void)sda; (void)scl; (void)frequency_hz;
    return false;
#endif
}

void m5u_i2c_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.end();
#endif
}

bool m5u_i2c_probe(uint8_t address) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    return Wire.endTransmission() == 0;
#else
    (void)address;
    return false;
#endif
}

bool m5u_i2c_write(uint8_t address, const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    if (data && len) {
        Wire.write(data, len);
    }
    return Wire.endTransmission() == 0;
#else
    (void)address; (void)data; (void)len;
    return false;
#endif
}

size_t m5u_i2c_read(uint8_t address, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    if (!data || !len) { return 0; }
    size_t requested = Wire.requestFrom((int)address, (int)len);
    size_t read_len = 0;
    while (Wire.available() && read_len < requested && read_len < len) {
        data[read_len++] = (uint8_t)Wire.read();
    }
    return read_len;
#else
    (void)address; (void)data; (void)len;
    return 0;
#endif
}

bool m5u_i2c_write_reg(uint8_t address, uint8_t reg, const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    Wire.write(reg);
    if (data && len) {
        Wire.write(data, len);
    }
    return Wire.endTransmission() == 0;
#else
    (void)address; (void)reg; (void)data; (void)len;
    return false;
#endif
}

size_t m5u_i2c_read_reg(uint8_t address, uint8_t reg, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    if (!data || !len) { return 0; }
    Wire.beginTransmission(address);
    Wire.write(reg);
    if (Wire.endTransmission(false) != 0) { return 0; }

    size_t requested = Wire.requestFrom((int)address, (int)len);
    size_t read_len = 0;
    while (Wire.available() && read_len < requested && read_len < len) {
        data[read_len++] = (uint8_t)Wire.read();
    }
    return read_len;
#else
    (void)address; (void)reg; (void)data; (void)len;
    return 0;
#endif
}

bool m5u_uart_begin(int rx, int tx, uint32_t baud) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial2.begin(baud, SERIAL_8N1, rx, tx);
    return true;
#else
    (void)rx; (void)tx; (void)baud;
    return false;
#endif
}

void m5u_uart_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial2.end();
#endif
}

size_t m5u_uart_available(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    int available = Serial2.available();
    return available > 0 ? (size_t)available : 0;
#else
    return 0;
#endif
}

size_t m5u_uart_read(uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    if (!data || !len) { return 0; }
    size_t read_len = 0;
    while (read_len < len && Serial2.available() > 0) {
        int value = Serial2.read();
        if (value < 0) { break; }
        data[read_len++] = (uint8_t)value;
    }
    return read_len;
#else
    (void)data; (void)len;
    return 0;
#endif
}

size_t m5u_uart_write(const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    if (!data || !len) { return 0; }
    return Serial2.write(data, len);
#else
    (void)data; (void)len;
    return 0;
#endif
}

void m5u_uart_flush(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial2.flush();
#endif
}

bool m5u_gpio_pin_mode(int pin, int mode) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    switch (mode) {
    case 0: pinMode(pin, INPUT); return true;
    case 1: pinMode(pin, OUTPUT); return true;
    case 2: pinMode(pin, INPUT_PULLUP); return true;
    case 3: pinMode(pin, INPUT_PULLDOWN); return true;
    default: return false;
    }
#else
    (void)pin; (void)mode;
    return false;
#endif
}

bool m5u_gpio_write(int pin, bool high) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    digitalWrite(pin, high ? HIGH : LOW);
    return true;
#else
    (void)pin; (void)high;
    return false;
#endif
}

int m5u_gpio_read(int pin) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    return digitalRead(pin) == HIGH ? 1 : 0;
#else
    (void)pin;
    return -1;
#endif
}

int m5u_gpio_analog_read(int pin) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    return analogRead(pin);
#else
    (void)pin;
    return -1;
#endif
}

int m5u_gpio_analog_read_millivolts(int pin) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    return analogReadMilliVolts(pin);
#else
    (void)pin;
    return -1;
#endif
}

bool m5u_gpio_analog_write(int pin, uint8_t duty) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    analogWrite(pin, duty);
    return true;
#else
    (void)pin; (void)duty;
    return false;
#endif
}

bool m5u_gpio_analog_write_frequency(int pin, uint32_t frequency_hz) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    analogWriteFrequency(pin, frequency_hz);
    return true;
#else
    (void)pin; (void)frequency_hz;
    return false;
#endif
}

bool m5u_gpio_analog_write_resolution(int pin, uint8_t resolution_bits) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    analogWriteResolution(pin, resolution_bits);
    return true;
#else
    (void)pin; (void)resolution_bits;
    return false;
#endif
}

bool m5u_servo_attach(int pin, int channel, int timer, uint32_t frequency_hz, uint16_t min_us, uint16_t max_us) {
    if (pin < 0 || !m5u_servo_valid_channel(channel) || timer < 0 || timer > 3) {
        return false;
    }
    if (frequency_hz == 0 || min_us == 0 || min_us >= max_us) {
        return false;
    }

    ledc_timer_config_t timer_config = {};
    timer_config.speed_mode = LEDC_LOW_SPEED_MODE;
    timer_config.duty_resolution = LEDC_TIMER_14_BIT;
    timer_config.timer_num = static_cast<ledc_timer_t>(timer);
    timer_config.freq_hz = frequency_hz;
    timer_config.clk_cfg = LEDC_AUTO_CLK;
    if (ledc_timer_config(&timer_config) != ESP_OK) {
        return false;
    }

    ledc_channel_config_t channel_config = {};
    channel_config.gpio_num = pin;
    channel_config.speed_mode = LEDC_LOW_SPEED_MODE;
    channel_config.channel = static_cast<ledc_channel_t>(channel);
    channel_config.intr_type = LEDC_INTR_DISABLE;
    channel_config.timer_sel = static_cast<ledc_timer_t>(timer);
    channel_config.duty = 0;
    channel_config.hpoint = 0;
    if (ledc_channel_config(&channel_config) != ESP_OK) {
        return false;
    }

    m5u_servo_states[channel] = {
        true,
        pin,
        timer,
        frequency_hz,
        min_us,
        max_us,
    };
    return true;
}

bool m5u_servo_detach(int channel) {
    if (!m5u_servo_valid_channel(channel) || !m5u_servo_states[channel].attached) {
        return false;
    }

    const auto ledc_channel = static_cast<ledc_channel_t>(channel);
    const bool ok = ledc_stop(LEDC_LOW_SPEED_MODE, ledc_channel, 0) == ESP_OK;
    m5u_servo_states[channel] = {};
    return ok;
}

bool m5u_servo_write_pulse_us(int channel, uint16_t pulse_us) {
    if (!m5u_servo_valid_channel(channel) || !m5u_servo_states[channel].attached) {
        return false;
    }

    const auto& state = m5u_servo_states[channel];
    if (pulse_us < state.min_us) {
        pulse_us = state.min_us;
    } else if (pulse_us > state.max_us) {
        pulse_us = state.max_us;
    }

    const auto ledc_channel = static_cast<ledc_channel_t>(channel);
    const uint32_t duty = m5u_servo_duty_from_pulse_us(state.frequency_hz, pulse_us);
    return ledc_set_duty(LEDC_LOW_SPEED_MODE, ledc_channel, duty) == ESP_OK
        && ledc_update_duty(LEDC_LOW_SPEED_MODE, ledc_channel) == ESP_OK;
}

bool m5u_stackchan_motion_begin(void) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.begin();
    M5StackChan.Motion.setAutoAngleSyncEnabled(false);
    M5StackChan.Motion.setAutoTorqueReleaseEnabled(true);
    M5StackChan.Motion.goHome(500);
    return true;
#else
    return false;
#endif
}

void m5u_stackchan_motion_update(void) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.update();
#endif
}

bool m5u_stackchan_motion_move(int16_t yaw_tenths, int16_t pitch_tenths, uint16_t speed_bsp) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.Motion.move(yaw_tenths, pitch_tenths, speed_bsp);
    return true;
#else
    (void)yaw_tenths;
    (void)pitch_tenths;
    (void)speed_bsp;
    return false;
#endif
}

bool m5u_stackchan_motion_home(uint16_t speed_bsp) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.Motion.goHome(speed_bsp);
    return true;
#else
    (void)speed_bsp;
    return false;
#endif
}

bool m5u_stackchan_motion_nod(void) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.Motion.moveY(300, 500);
    m5u_delay_ms(200);
    M5StackChan.Motion.moveY(50, 600);
    m5u_delay_ms(200);
    M5StackChan.Motion.moveY(300, 500);
    m5u_delay_ms(200);
    M5StackChan.Motion.goHome(500);
    return true;
#else
    return false;
#endif
}

bool m5u_stackchan_motion_shake(void) {
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    M5StackChan.Motion.moveX(-400, 600);
    m5u_delay_ms(200);
    M5StackChan.Motion.moveX(400, 600);
    m5u_delay_ms(200);
    M5StackChan.Motion.moveX(-400, 600);
    m5u_delay_ms(200);
    M5StackChan.Motion.goHome(500);
    return true;
#else
    return false;
#endif
}

bool m5u_stackchan_motion_status(m5u_stackchan_motion_status_t* out) {
    if (out == nullptr) {
        return false;
    }
#ifdef M5UNIFIED_RS_USE_STACKCHAN_BSP
    out->ready = true;
    out->moving = M5StackChan.Motion.isMoving();
    out->yaw_tenths = M5StackChan.Motion.getCurrentAngleX();
    out->pitch_tenths = M5StackChan.Motion.getCurrentAngleY();
    return true;
#else
    out->ready = false;
    out->moving = false;
    out->yaw_tenths = 0;
    out->pitch_tenths = 0;
    return false;
#endif
}

#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
static int m5u_spi_cs_pin = -1;

static uint8_t m5u_spi_mode_from_u8(uint8_t mode) {
    switch (mode) {
    case 1: return SPI_MODE1;
    case 2: return SPI_MODE2;
    case 3: return SPI_MODE3;
    default: return SPI_MODE0;
    }
}

static void m5u_spi_select(void) {
    if (m5u_spi_cs_pin >= 0) {
        digitalWrite(m5u_spi_cs_pin, LOW);
    }
}

static void m5u_spi_deselect(void) {
    if (m5u_spi_cs_pin >= 0) {
        digitalWrite(m5u_spi_cs_pin, HIGH);
    }
}
#endif

bool m5u_spi_begin(int sck, int miso, int mosi, int cs) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
    SPI.begin(sck, miso, mosi, cs);
    m5u_spi_cs_pin = cs;
    if (m5u_spi_cs_pin >= 0) {
        pinMode(m5u_spi_cs_pin, OUTPUT);
        digitalWrite(m5u_spi_cs_pin, HIGH);
    }
    return true;
#else
    (void)sck; (void)miso; (void)mosi; (void)cs;
    return false;
#endif
}

void m5u_spi_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
    m5u_spi_deselect();
    SPI.end();
    m5u_spi_cs_pin = -1;
#endif
}

uint8_t m5u_spi_transfer_byte(uint8_t value, uint32_t frequency_hz, uint8_t mode, bool lsb_first) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
    SPI.beginTransaction(SPISettings(frequency_hz, lsb_first ? LSBFIRST : MSBFIRST, m5u_spi_mode_from_u8(mode)));
    m5u_spi_select();
    uint8_t result = SPI.transfer(value);
    m5u_spi_deselect();
    SPI.endTransaction();
    return result;
#else
    (void)value; (void)frequency_hz; (void)mode; (void)lsb_first;
    return 0;
#endif
}

bool m5u_spi_transfer(const uint8_t* tx, uint8_t* rx, size_t len, uint32_t frequency_hz, uint8_t mode, bool lsb_first) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
    if (rx == nullptr && len != 0) {
        return false;
    }

    SPI.beginTransaction(SPISettings(frequency_hz, lsb_first ? LSBFIRST : MSBFIRST, m5u_spi_mode_from_u8(mode)));
    m5u_spi_select();
    for (size_t i = 0; i < len; ++i) {
        rx[i] = SPI.transfer(tx != nullptr ? tx[i] : 0);
    }
    m5u_spi_deselect();
    SPI.endTransaction();
    return true;
#else
    (void)tx; (void)rx; (void)len; (void)frequency_hz; (void)mode; (void)lsb_first;
    return false;
#endif
}

bool m5u_spi_write(const uint8_t* data, size_t len, uint32_t frequency_hz, uint8_t mode, bool lsb_first) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SPI
    if (data == nullptr && len != 0) {
        return false;
    }

    SPI.beginTransaction(SPISettings(frequency_hz, lsb_first ? LSBFIRST : MSBFIRST, m5u_spi_mode_from_u8(mode)));
    m5u_spi_select();
    if (len != 0) {
        SPI.writeBytes(data, len);
    }
    m5u_spi_deselect();
    SPI.endTransaction();
    return true;
#else
    (void)data; (void)len; (void)frequency_hz; (void)mode; (void)lsb_first;
    return false;
#endif
}


#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
static m5::Button_Class* m5u_button_from_int(int button) {
    switch (button) {
    case 0: return &M5.BtnA;
    case 1: return &M5.BtnB;
    case 2: return &M5.BtnC;
    case 3: return &M5.BtnPWR;
    case 4: return &M5.BtnEXT;
    default: return nullptr;
    }
}
#endif

static bool m5u_button_state(int button, int query) {
    m5::Button_Class* btn = m5u_button_from_int(button);
    if (!btn) { return false; }
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

uint8_t m5u_display_get_brightness(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getBrightness();
#else
    return 0;
#endif
}

void m5u_display_set_color_depth(uint8_t depth) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setColorDepth(depth);
#else
    (void)depth;
#endif
}

uint8_t m5u_display_get_color_depth(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<uint8_t>(M5.Display.getColorDepth());
#else
    return 16;
#endif
}

bool m5u_display_is_epd(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.isEPD();
#else
    return false;
#endif
}

void m5u_display_set_epd_mode(int mode) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    switch (mode) {
    case 1: M5.Display.setEpdMode(m5gfx::epd_quality); break;
    case 2: M5.Display.setEpdMode(m5gfx::epd_text); break;
    case 3: M5.Display.setEpdMode(m5gfx::epd_fast); break;
    case 4: M5.Display.setEpdMode(m5gfx::epd_fastest); break;
    default: break;
    }
#else
    (void)mode;
#endif
}

int m5u_display_get_epd_mode(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<int>(M5.Display.getEpdMode());
#else
    return 0;
#endif
}

void m5u_display_set_epd_fastest(void) {
    M5.Display.setEpdMode(m5gfx::epd_fastest);
}

bool m5u_display_set_resolution(uint16_t logical_width, uint16_t logical_height, float refresh_rate, uint16_t output_width, uint16_t output_height, uint8_t scale_w, uint8_t scale_h, uint32_t pixel_clock) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.setResolution(logical_width, logical_height, refresh_rate, output_width, output_height, scale_w, scale_h, pixel_clock);
#else
    (void)logical_width; (void)logical_height; (void)refresh_rate; (void)output_width; (void)output_height; (void)scale_w; (void)scale_h; (void)pixel_clock; return false;
#endif
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

void m5u_display_sleep(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.sleep();
#endif
}

void m5u_display_wakeup(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.wakeup();
#endif
}

void m5u_display_power_save_on(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.powerSaveOn();
#endif
}

void m5u_display_power_save_off(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.powerSaveOff();
#endif
}

void m5u_display_power_save(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.powerSave(enable);
#else
    (void)enable;
#endif
}

void m5u_display_invert_display(bool invert) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.invertDisplay(invert);
#else
    (void)invert;
#endif
}

int m5u_display_get_cursor_x(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getCursorX();
#else
    return 0;
#endif
}

int m5u_display_get_cursor_y(void) {
    return M5.Display.getCursorY();
}

void m5u_display_set_pivot(float x, float y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setPivot(x, y);
#else
    (void)x; (void)y;
#endif
}

float m5u_display_get_pivot_x(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getPivotX();
#else
    return 0.0f;
#endif
}

float m5u_display_get_pivot_y(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getPivotY();
#else
    return 0.0f;
#endif
}

int m5u_display_font_height(void) {
    return M5.Display.fontHeight();
}

int m5u_display_font_width(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.fontWidth();
#else
    return 8;
#endif
}

bool m5u_display_set_font(int font) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
#define M5U_SET_FONT_CASE(id, name) case id: M5.Display.setFont(&fonts::name); return true
    switch (font) {
    M5U_SET_FONT_CASE(0, Font0);
    M5U_SET_FONT_CASE(1, Font2);
    M5U_SET_FONT_CASE(2, Font4);
    M5U_SET_FONT_CASE(3, Font6);
    M5U_SET_FONT_CASE(4, Font7);
    M5U_SET_FONT_CASE(5, Font8);
    M5U_SET_FONT_CASE(6, Font8x8C64);
    M5U_SET_FONT_CASE(7, AsciiFont8x16);
    M5U_SET_FONT_CASE(8, AsciiFont24x48);
    M5U_SET_FONT_CASE(9, TomThumb);
    M5U_SET_FONT_CASE(10, FreeMono9pt7b);
    M5U_SET_FONT_CASE(11, FreeMono12pt7b);
    M5U_SET_FONT_CASE(12, FreeMono18pt7b);
    M5U_SET_FONT_CASE(13, FreeMono24pt7b);
    M5U_SET_FONT_CASE(14, FreeMonoBold9pt7b);
    M5U_SET_FONT_CASE(15, FreeMonoBold12pt7b);
    M5U_SET_FONT_CASE(16, FreeMonoBold18pt7b);
    M5U_SET_FONT_CASE(17, FreeMonoBold24pt7b);
    M5U_SET_FONT_CASE(18, FreeMonoOblique9pt7b);
    M5U_SET_FONT_CASE(19, FreeMonoOblique12pt7b);
    M5U_SET_FONT_CASE(20, FreeMonoOblique18pt7b);
    M5U_SET_FONT_CASE(21, FreeMonoOblique24pt7b);
    M5U_SET_FONT_CASE(22, FreeMonoBoldOblique9pt7b);
    M5U_SET_FONT_CASE(23, FreeMonoBoldOblique12pt7b);
    M5U_SET_FONT_CASE(24, FreeMonoBoldOblique18pt7b);
    M5U_SET_FONT_CASE(25, FreeMonoBoldOblique24pt7b);
    M5U_SET_FONT_CASE(26, FreeSans9pt7b);
    M5U_SET_FONT_CASE(27, FreeSans12pt7b);
    M5U_SET_FONT_CASE(28, FreeSans18pt7b);
    M5U_SET_FONT_CASE(29, FreeSans24pt7b);
    M5U_SET_FONT_CASE(30, FreeSansBold9pt7b);
    M5U_SET_FONT_CASE(31, FreeSansBold12pt7b);
    M5U_SET_FONT_CASE(32, FreeSansBold18pt7b);
    M5U_SET_FONT_CASE(33, FreeSansBold24pt7b);
    M5U_SET_FONT_CASE(34, FreeSansOblique9pt7b);
    M5U_SET_FONT_CASE(35, FreeSansOblique12pt7b);
    M5U_SET_FONT_CASE(36, FreeSansOblique18pt7b);
    M5U_SET_FONT_CASE(37, FreeSansOblique24pt7b);
    M5U_SET_FONT_CASE(38, FreeSansBoldOblique9pt7b);
    M5U_SET_FONT_CASE(39, FreeSansBoldOblique12pt7b);
    M5U_SET_FONT_CASE(40, FreeSansBoldOblique18pt7b);
    M5U_SET_FONT_CASE(41, FreeSansBoldOblique24pt7b);
    M5U_SET_FONT_CASE(42, FreeSerif9pt7b);
    M5U_SET_FONT_CASE(43, FreeSerif12pt7b);
    M5U_SET_FONT_CASE(44, FreeSerif18pt7b);
    M5U_SET_FONT_CASE(45, FreeSerif24pt7b);
    M5U_SET_FONT_CASE(46, FreeSerifItalic9pt7b);
    M5U_SET_FONT_CASE(47, FreeSerifItalic12pt7b);
    M5U_SET_FONT_CASE(48, FreeSerifItalic18pt7b);
    M5U_SET_FONT_CASE(49, FreeSerifItalic24pt7b);
    M5U_SET_FONT_CASE(50, FreeSerifBold9pt7b);
    M5U_SET_FONT_CASE(51, FreeSerifBold12pt7b);
    M5U_SET_FONT_CASE(52, FreeSerifBold18pt7b);
    M5U_SET_FONT_CASE(53, FreeSerifBold24pt7b);
    M5U_SET_FONT_CASE(54, FreeSerifBoldItalic9pt7b);
    M5U_SET_FONT_CASE(55, FreeSerifBoldItalic12pt7b);
    M5U_SET_FONT_CASE(56, FreeSerifBoldItalic18pt7b);
    M5U_SET_FONT_CASE(57, FreeSerifBoldItalic24pt7b);
    M5U_SET_FONT_CASE(58, Orbitron_Light_24);
    M5U_SET_FONT_CASE(59, Orbitron_Light_32);
    M5U_SET_FONT_CASE(60, Roboto_Thin_24);
    M5U_SET_FONT_CASE(61, Satisfy_24);
    M5U_SET_FONT_CASE(62, Yellowtail_32);
    M5U_SET_FONT_CASE(63, DejaVu9);
    M5U_SET_FONT_CASE(64, DejaVu12);
    M5U_SET_FONT_CASE(65, DejaVu18);
    M5U_SET_FONT_CASE(66, DejaVu24);
    M5U_SET_FONT_CASE(67, DejaVu40);
    M5U_SET_FONT_CASE(68, DejaVu56);
    M5U_SET_FONT_CASE(69, DejaVu72);
    default: return false;
    }
#undef M5U_SET_FONT_CASE
#else
    return font >= 0 && font <= 69;
#endif
}

bool m5u_display_show_font(uint32_t duration_ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.showFont(duration_ms);
    return true;
#else
    (void)duration_ms;
    return true;
#endif
}

void m5u_display_unload_font(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.unloadFont();
#endif
}

int m5u_display_font_height_for(int font) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
#define M5U_FONT_HEIGHT_CASE(id, name) case id: return M5.Display.fontHeight(&fonts::name)
    switch (font) {
    M5U_FONT_HEIGHT_CASE(0, Font0);
    M5U_FONT_HEIGHT_CASE(1, Font2);
    M5U_FONT_HEIGHT_CASE(2, Font4);
    M5U_FONT_HEIGHT_CASE(3, Font6);
    M5U_FONT_HEIGHT_CASE(4, Font7);
    M5U_FONT_HEIGHT_CASE(5, Font8);
    M5U_FONT_HEIGHT_CASE(6, Font8x8C64);
    M5U_FONT_HEIGHT_CASE(7, AsciiFont8x16);
    M5U_FONT_HEIGHT_CASE(8, AsciiFont24x48);
    M5U_FONT_HEIGHT_CASE(9, TomThumb);
    M5U_FONT_HEIGHT_CASE(10, FreeMono9pt7b);
    M5U_FONT_HEIGHT_CASE(11, FreeMono12pt7b);
    M5U_FONT_HEIGHT_CASE(12, FreeMono18pt7b);
    M5U_FONT_HEIGHT_CASE(13, FreeMono24pt7b);
    M5U_FONT_HEIGHT_CASE(14, FreeMonoBold9pt7b);
    M5U_FONT_HEIGHT_CASE(15, FreeMonoBold12pt7b);
    M5U_FONT_HEIGHT_CASE(16, FreeMonoBold18pt7b);
    M5U_FONT_HEIGHT_CASE(17, FreeMonoBold24pt7b);
    M5U_FONT_HEIGHT_CASE(18, FreeMonoOblique9pt7b);
    M5U_FONT_HEIGHT_CASE(19, FreeMonoOblique12pt7b);
    M5U_FONT_HEIGHT_CASE(20, FreeMonoOblique18pt7b);
    M5U_FONT_HEIGHT_CASE(21, FreeMonoOblique24pt7b);
    M5U_FONT_HEIGHT_CASE(22, FreeMonoBoldOblique9pt7b);
    M5U_FONT_HEIGHT_CASE(23, FreeMonoBoldOblique12pt7b);
    M5U_FONT_HEIGHT_CASE(24, FreeMonoBoldOblique18pt7b);
    M5U_FONT_HEIGHT_CASE(25, FreeMonoBoldOblique24pt7b);
    M5U_FONT_HEIGHT_CASE(26, FreeSans9pt7b);
    M5U_FONT_HEIGHT_CASE(27, FreeSans12pt7b);
    M5U_FONT_HEIGHT_CASE(28, FreeSans18pt7b);
    M5U_FONT_HEIGHT_CASE(29, FreeSans24pt7b);
    M5U_FONT_HEIGHT_CASE(30, FreeSansBold9pt7b);
    M5U_FONT_HEIGHT_CASE(31, FreeSansBold12pt7b);
    M5U_FONT_HEIGHT_CASE(32, FreeSansBold18pt7b);
    M5U_FONT_HEIGHT_CASE(33, FreeSansBold24pt7b);
    M5U_FONT_HEIGHT_CASE(34, FreeSansOblique9pt7b);
    M5U_FONT_HEIGHT_CASE(35, FreeSansOblique12pt7b);
    M5U_FONT_HEIGHT_CASE(36, FreeSansOblique18pt7b);
    M5U_FONT_HEIGHT_CASE(37, FreeSansOblique24pt7b);
    M5U_FONT_HEIGHT_CASE(38, FreeSansBoldOblique9pt7b);
    M5U_FONT_HEIGHT_CASE(39, FreeSansBoldOblique12pt7b);
    M5U_FONT_HEIGHT_CASE(40, FreeSansBoldOblique18pt7b);
    M5U_FONT_HEIGHT_CASE(41, FreeSansBoldOblique24pt7b);
    M5U_FONT_HEIGHT_CASE(42, FreeSerif9pt7b);
    M5U_FONT_HEIGHT_CASE(43, FreeSerif12pt7b);
    M5U_FONT_HEIGHT_CASE(44, FreeSerif18pt7b);
    M5U_FONT_HEIGHT_CASE(45, FreeSerif24pt7b);
    M5U_FONT_HEIGHT_CASE(46, FreeSerifItalic9pt7b);
    M5U_FONT_HEIGHT_CASE(47, FreeSerifItalic12pt7b);
    M5U_FONT_HEIGHT_CASE(48, FreeSerifItalic18pt7b);
    M5U_FONT_HEIGHT_CASE(49, FreeSerifItalic24pt7b);
    M5U_FONT_HEIGHT_CASE(50, FreeSerifBold9pt7b);
    M5U_FONT_HEIGHT_CASE(51, FreeSerifBold12pt7b);
    M5U_FONT_HEIGHT_CASE(52, FreeSerifBold18pt7b);
    M5U_FONT_HEIGHT_CASE(53, FreeSerifBold24pt7b);
    M5U_FONT_HEIGHT_CASE(54, FreeSerifBoldItalic9pt7b);
    M5U_FONT_HEIGHT_CASE(55, FreeSerifBoldItalic12pt7b);
    M5U_FONT_HEIGHT_CASE(56, FreeSerifBoldItalic18pt7b);
    M5U_FONT_HEIGHT_CASE(57, FreeSerifBoldItalic24pt7b);
    M5U_FONT_HEIGHT_CASE(58, Orbitron_Light_24);
    M5U_FONT_HEIGHT_CASE(59, Orbitron_Light_32);
    M5U_FONT_HEIGHT_CASE(60, Roboto_Thin_24);
    M5U_FONT_HEIGHT_CASE(61, Satisfy_24);
    M5U_FONT_HEIGHT_CASE(62, Yellowtail_32);
    M5U_FONT_HEIGHT_CASE(63, DejaVu9);
    M5U_FONT_HEIGHT_CASE(64, DejaVu12);
    M5U_FONT_HEIGHT_CASE(65, DejaVu18);
    M5U_FONT_HEIGHT_CASE(66, DejaVu24);
    M5U_FONT_HEIGHT_CASE(67, DejaVu40);
    M5U_FONT_HEIGHT_CASE(68, DejaVu56);
    M5U_FONT_HEIGHT_CASE(69, DejaVu72);
    default: return 0;
    }
#undef M5U_FONT_HEIGHT_CASE
#else
    (void)font;
    return 16;
#endif
}

int m5u_display_font_width_for(int font) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
#define M5U_FONT_WIDTH_CASE(id, name) case id: return M5.Display.fontWidth(&fonts::name)
    switch (font) {
    M5U_FONT_WIDTH_CASE(0, Font0);
    M5U_FONT_WIDTH_CASE(1, Font2);
    M5U_FONT_WIDTH_CASE(2, Font4);
    M5U_FONT_WIDTH_CASE(3, Font6);
    M5U_FONT_WIDTH_CASE(4, Font7);
    M5U_FONT_WIDTH_CASE(5, Font8);
    M5U_FONT_WIDTH_CASE(6, Font8x8C64);
    M5U_FONT_WIDTH_CASE(7, AsciiFont8x16);
    M5U_FONT_WIDTH_CASE(8, AsciiFont24x48);
    M5U_FONT_WIDTH_CASE(9, TomThumb);
    M5U_FONT_WIDTH_CASE(10, FreeMono9pt7b);
    M5U_FONT_WIDTH_CASE(11, FreeMono12pt7b);
    M5U_FONT_WIDTH_CASE(12, FreeMono18pt7b);
    M5U_FONT_WIDTH_CASE(13, FreeMono24pt7b);
    M5U_FONT_WIDTH_CASE(14, FreeMonoBold9pt7b);
    M5U_FONT_WIDTH_CASE(15, FreeMonoBold12pt7b);
    M5U_FONT_WIDTH_CASE(16, FreeMonoBold18pt7b);
    M5U_FONT_WIDTH_CASE(17, FreeMonoBold24pt7b);
    M5U_FONT_WIDTH_CASE(18, FreeMonoOblique9pt7b);
    M5U_FONT_WIDTH_CASE(19, FreeMonoOblique12pt7b);
    M5U_FONT_WIDTH_CASE(20, FreeMonoOblique18pt7b);
    M5U_FONT_WIDTH_CASE(21, FreeMonoOblique24pt7b);
    M5U_FONT_WIDTH_CASE(22, FreeMonoBoldOblique9pt7b);
    M5U_FONT_WIDTH_CASE(23, FreeMonoBoldOblique12pt7b);
    M5U_FONT_WIDTH_CASE(24, FreeMonoBoldOblique18pt7b);
    M5U_FONT_WIDTH_CASE(25, FreeMonoBoldOblique24pt7b);
    M5U_FONT_WIDTH_CASE(26, FreeSans9pt7b);
    M5U_FONT_WIDTH_CASE(27, FreeSans12pt7b);
    M5U_FONT_WIDTH_CASE(28, FreeSans18pt7b);
    M5U_FONT_WIDTH_CASE(29, FreeSans24pt7b);
    M5U_FONT_WIDTH_CASE(30, FreeSansBold9pt7b);
    M5U_FONT_WIDTH_CASE(31, FreeSansBold12pt7b);
    M5U_FONT_WIDTH_CASE(32, FreeSansBold18pt7b);
    M5U_FONT_WIDTH_CASE(33, FreeSansBold24pt7b);
    M5U_FONT_WIDTH_CASE(34, FreeSansOblique9pt7b);
    M5U_FONT_WIDTH_CASE(35, FreeSansOblique12pt7b);
    M5U_FONT_WIDTH_CASE(36, FreeSansOblique18pt7b);
    M5U_FONT_WIDTH_CASE(37, FreeSansOblique24pt7b);
    M5U_FONT_WIDTH_CASE(38, FreeSansBoldOblique9pt7b);
    M5U_FONT_WIDTH_CASE(39, FreeSansBoldOblique12pt7b);
    M5U_FONT_WIDTH_CASE(40, FreeSansBoldOblique18pt7b);
    M5U_FONT_WIDTH_CASE(41, FreeSansBoldOblique24pt7b);
    M5U_FONT_WIDTH_CASE(42, FreeSerif9pt7b);
    M5U_FONT_WIDTH_CASE(43, FreeSerif12pt7b);
    M5U_FONT_WIDTH_CASE(44, FreeSerif18pt7b);
    M5U_FONT_WIDTH_CASE(45, FreeSerif24pt7b);
    M5U_FONT_WIDTH_CASE(46, FreeSerifItalic9pt7b);
    M5U_FONT_WIDTH_CASE(47, FreeSerifItalic12pt7b);
    M5U_FONT_WIDTH_CASE(48, FreeSerifItalic18pt7b);
    M5U_FONT_WIDTH_CASE(49, FreeSerifItalic24pt7b);
    M5U_FONT_WIDTH_CASE(50, FreeSerifBold9pt7b);
    M5U_FONT_WIDTH_CASE(51, FreeSerifBold12pt7b);
    M5U_FONT_WIDTH_CASE(52, FreeSerifBold18pt7b);
    M5U_FONT_WIDTH_CASE(53, FreeSerifBold24pt7b);
    M5U_FONT_WIDTH_CASE(54, FreeSerifBoldItalic9pt7b);
    M5U_FONT_WIDTH_CASE(55, FreeSerifBoldItalic12pt7b);
    M5U_FONT_WIDTH_CASE(56, FreeSerifBoldItalic18pt7b);
    M5U_FONT_WIDTH_CASE(57, FreeSerifBoldItalic24pt7b);
    M5U_FONT_WIDTH_CASE(58, Orbitron_Light_24);
    M5U_FONT_WIDTH_CASE(59, Orbitron_Light_32);
    M5U_FONT_WIDTH_CASE(60, Roboto_Thin_24);
    M5U_FONT_WIDTH_CASE(61, Satisfy_24);
    M5U_FONT_WIDTH_CASE(62, Yellowtail_32);
    M5U_FONT_WIDTH_CASE(63, DejaVu9);
    M5U_FONT_WIDTH_CASE(64, DejaVu12);
    M5U_FONT_WIDTH_CASE(65, DejaVu18);
    M5U_FONT_WIDTH_CASE(66, DejaVu24);
    M5U_FONT_WIDTH_CASE(67, DejaVu40);
    M5U_FONT_WIDTH_CASE(68, DejaVu56);
    M5U_FONT_WIDTH_CASE(69, DejaVu72);
    default: return 0;
    }
#undef M5U_FONT_WIDTH_CASE
#else
    (void)font;
    return 8;
#endif
}

uint16_t m5u_display_get_base_color(void) {
    return M5.Display.getBaseColor();
}

void m5u_display_set_base_color(uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setBaseColor(color);
#else
    (void)color;
#endif
}

void m5u_display_set_color(uint16_t color) {
    M5.Display.setColor(color);
}

void m5u_display_set_rgb_color(uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setColor(r, g, b);
#else
    (void)r; (void)g; (void)b;
#endif
}

void m5u_display_set_raw_color(uint32_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setRawColor(color);
#else
    (void)color;
#endif
}

uint32_t m5u_display_get_raw_color(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getRawColor();
#else
    return 0;
#endif
}

uint32_t m5u_display_get_palette_count(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getPaletteCount();
#else
    return 0;
#endif
}

void m5u_display_set_swap_bytes(bool swap) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setSwapBytes(swap);
#else
    (void)swap;
#endif
}

bool m5u_display_get_swap_bytes(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getSwapBytes();
#else
    return false;
#endif
}

uint16_t m5u_display_swap565(uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.swap565(r, g, b);
#else
    uint16_t rgb565 = ((uint16_t)(r & 0xF8) << 8) | ((uint16_t)(g & 0xFC) << 3) | ((uint16_t)b >> 3);
    return (uint16_t)((rgb565 << 8) | (rgb565 >> 8));
#endif
}

uint32_t m5u_display_swap888(uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.swap888(r, g, b);
#else
    return ((uint32_t)b << 16) | ((uint32_t)g << 8) | (uint32_t)r;
#endif
}

void m5u_display_set_text_wrap(bool wrap_x, bool wrap_y) {
    M5.Display.setTextWrap(wrap_x, wrap_y);
}

void m5u_display_set_text_datum(int datum) {
    M5.Display.setTextDatum((textdatum_t)datum);
}

int m5u_display_get_text_datum(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getTextDatum();
#else
    return 0;
#endif
}

void m5u_display_set_text_padding(uint32_t padding_x) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextPadding(padding_x);
#else
    (void)padding_x;
#endif
}

uint32_t m5u_display_get_text_padding(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getTextPadding();
#else
    return 0;
#endif
}

uint8_t m5u_display_get_text_size_x(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getTextSizeX();
#else
    return 1;
#endif
}

uint8_t m5u_display_get_text_size_y(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getTextSizeY();
#else
    return 1;
#endif
}

int m5u_display_text_length(const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.textLength(text);
#else
    return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

int m5u_display_text_width(const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.textWidth(text);
#else
    return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

int m5u_display_draw_center_string(const char* text, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.drawCenterString(text, x, y);
#else
    (void)x; (void)y; return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

int m5u_display_draw_string(const char* text, int x, int y) {
    return M5.Display.drawString(text, x, y);
}

int m5u_display_draw_char(uint32_t codepoint, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.drawChar(codepoint, x, y);
#else
    (void)codepoint; (void)x; (void)y; return 8;
#endif
}

int m5u_display_draw_number(int32_t value, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.drawNumber(value, x, y);
#else
    (void)value; (void)x; (void)y; return 0;
#endif
}

int m5u_display_draw_float(float value, uint8_t decimals, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.drawFloat(value, decimals, x, y);
#else
    (void)value; (void)decimals; (void)x; (void)y; return 0;
#endif
}

bool m5u_display_draw_bmp(const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Display.drawBmp(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

bool m5u_display_draw_jpg(const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Display.drawJpg(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

bool m5u_display_draw_png(const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Display.drawPng(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

bool m5u_display_push_image_rgb565(int x, int y, int w, int h, const uint16_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || w <= 0 || h <= 0 || len < static_cast<size_t>(w) * static_cast<size_t>(h)) { return false; }
    M5.Display.pushImage(x, y, w, h, data);
    return true;
#else
    (void)x; (void)y; (void)w; (void)h; (void)data; (void)len;
    return false;
#endif
}

void m5u_display_write_pixel(int x, int y, uint16_t color) {
    M5.Display.writePixel(x, y, color);
}

void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color) {
    M5.Display.writeFastVLine(x, y, h, color);
}

void m5u_display_set_addr_window(int x, int y, int w, int h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setAddrWindow(x, y, w, h);
#else
    (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_display_set_window(int xs, int ys, int xe, int ye) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setWindow((uint_fast16_t)xs, (uint_fast16_t)ys, (uint_fast16_t)xe, (uint_fast16_t)ye);
#else
    (void)xs; (void)ys; (void)xe; (void)ye;
#endif
}

void m5u_display_set_clip_rect(int x, int y, int w, int h) {
    M5.Display.setClipRect(x, y, w, h);
}

void m5u_display_get_clip_rect(int* x, int* y, int* w, int* h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.getClipRect(x, y, w, h);
#else
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 320; }
    if (h) { *h = 240; }
#endif
}

void m5u_display_clear_clip_rect(void) {
    M5.Display.clearClipRect();
}

void m5u_display_scroll(int dx, int dy) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.scroll(dx, dy);
#else
    (void)dx; (void)dy;
#endif
}

void m5u_display_set_text_scroll(bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextScroll(enable);
#else
    (void)enable;
#endif
}

void m5u_display_set_scroll_rect(int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setScrollRect(x, y, w, h, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_get_scroll_rect(int* x, int* y, int* w, int* h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.getScrollRect(x, y, w, h);
#else
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
#endif
}

void m5u_display_clear_scroll_rect(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.clearScrollRect();
#endif
}

uint16_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) {
    return M5.Display.color888(r, g, b);
}

m5u_canvas_t* m5u_canvas_create_for_display(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return reinterpret_cast<m5u_canvas_t*>(new M5Canvas(&M5.Display));
#else
    return nullptr;
#endif
}

m5u_canvas_t* m5u_canvas_create_for_cardputer_display(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return reinterpret_cast<m5u_canvas_t*>(new M5Canvas(&M5Cardputer.Display));
#else
    return nullptr;
#endif
}

void m5u_canvas_delete(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    delete reinterpret_cast<M5Canvas*>(canvas);
#else
    (void)canvas;
#endif
}

bool m5u_canvas_create_sprite(m5u_canvas_t* canvas, int w, int h) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (!canvas) { return false; }
    return reinterpret_cast<M5Canvas*>(canvas)->createSprite(w, h) != nullptr;
#else
    (void)canvas; (void)w; (void)h; return false;
#endif
}

void m5u_canvas_delete_sprite(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->deleteSprite(); }
#else
    (void)canvas;
#endif
}

void m5u_canvas_push_sprite(m5u_canvas_t* canvas, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->pushSprite(x, y); }
#else
    (void)canvas; (void)x; (void)y;
#endif
}

int m5u_canvas_width(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->width() : 0;
#else
    (void)canvas; return 0;
#endif
}

int m5u_canvas_height(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->height() : 0;
#else
    (void)canvas; return 0;
#endif
}

void m5u_canvas_fill_screen(m5u_canvas_t* canvas, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillScreen(color); }
#else
    (void)canvas; (void)color;
#endif
}

void m5u_canvas_set_cursor(m5u_canvas_t* canvas, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setCursor(x, y); }
#else
    (void)canvas; (void)x; (void)y;
#endif
}

void m5u_canvas_set_text_size(m5u_canvas_t* canvas, float size) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setTextSize(size); }
#else
    (void)canvas; (void)size;
#endif
}

void m5u_canvas_set_text_color(m5u_canvas_t* canvas, uint16_t fg, uint16_t bg) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setTextColor(fg, bg); }
#else
    (void)canvas; (void)fg; (void)bg;
#endif
}

void m5u_canvas_set_text_scroll(m5u_canvas_t* canvas, bool enable) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setTextScroll(enable); }
#else
    (void)canvas; (void)enable;
#endif
}

void m5u_canvas_set_text_datum(m5u_canvas_t* canvas, int datum) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setTextDatum((textdatum_t)datum); }
#else
    (void)canvas; (void)datum;
#endif
}

int m5u_canvas_get_text_datum(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getTextDatum() : 0;
#else
    (void)canvas;
    return 0;
#endif
}

void m5u_canvas_set_text_padding(m5u_canvas_t* canvas, uint32_t padding_x) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setTextPadding(padding_x); }
#else
    (void)canvas; (void)padding_x;
#endif
}

uint32_t m5u_canvas_get_text_padding(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getTextPadding() : 0;
#else
    (void)canvas;
    return 0;
#endif
}

uint8_t m5u_canvas_get_text_size_x(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getTextSizeX() : 1;
#else
    (void)canvas;
    return 1;
#endif
}

uint8_t m5u_canvas_get_text_size_y(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getTextSizeY() : 1;
#else
    (void)canvas;
    return 1;
#endif
}

uint16_t m5u_canvas_get_base_color(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getBaseColor() : 0;
#else
    (void)canvas;
    return 0;
#endif
}

void m5u_canvas_set_base_color(m5u_canvas_t* canvas, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setBaseColor(color); }
#else
    (void)canvas; (void)color;
#endif
}

void m5u_canvas_set_color(m5u_canvas_t* canvas, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setColor(color); }
#else
    (void)canvas; (void)color;
#endif
}

void m5u_canvas_set_rgb_color(m5u_canvas_t* canvas, uint8_t r, uint8_t g, uint8_t b) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setRGBColor(r, g, b); }
#else
    (void)canvas; (void)r; (void)g; (void)b;
#endif
}

void m5u_canvas_set_raw_color(m5u_canvas_t* canvas, uint32_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setRawColor(color); }
#else
    (void)canvas; (void)color;
#endif
}

uint32_t m5u_canvas_get_raw_color(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getRawColor() : 0;
#else
    (void)canvas;
    return 0;
#endif
}

void m5u_canvas_set_swap_bytes(m5u_canvas_t* canvas, bool swap) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setSwapBytes(swap); }
#else
    (void)canvas; (void)swap;
#endif
}

bool m5u_canvas_get_swap_bytes(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->getSwapBytes() : false;
#else
    (void)canvas;
    return false;
#endif
}

bool m5u_canvas_set_font(m5u_canvas_t* canvas, int font) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (!canvas) { return false; }
#define M5U_CANVAS_SET_FONT_CASE(id, name) case id: reinterpret_cast<M5Canvas*>(canvas)->setFont(&fonts::name); return true
    switch (font) {
    M5U_CANVAS_SET_FONT_CASE(0, Font0);
    M5U_CANVAS_SET_FONT_CASE(1, Font2);
    M5U_CANVAS_SET_FONT_CASE(2, Font4);
    M5U_CANVAS_SET_FONT_CASE(3, Font6);
    M5U_CANVAS_SET_FONT_CASE(4, Font7);
    M5U_CANVAS_SET_FONT_CASE(5, Font8);
    M5U_CANVAS_SET_FONT_CASE(6, Font8x8C64);
    M5U_CANVAS_SET_FONT_CASE(7, AsciiFont8x16);
    M5U_CANVAS_SET_FONT_CASE(8, AsciiFont24x48);
    M5U_CANVAS_SET_FONT_CASE(9, TomThumb);
    M5U_CANVAS_SET_FONT_CASE(10, FreeMono9pt7b);
    M5U_CANVAS_SET_FONT_CASE(11, FreeMono12pt7b);
    M5U_CANVAS_SET_FONT_CASE(12, FreeMono18pt7b);
    M5U_CANVAS_SET_FONT_CASE(13, FreeMono24pt7b);
    M5U_CANVAS_SET_FONT_CASE(14, FreeMonoBold9pt7b);
    M5U_CANVAS_SET_FONT_CASE(15, FreeMonoBold12pt7b);
    M5U_CANVAS_SET_FONT_CASE(16, FreeMonoBold18pt7b);
    M5U_CANVAS_SET_FONT_CASE(17, FreeMonoBold24pt7b);
    M5U_CANVAS_SET_FONT_CASE(18, FreeMonoOblique9pt7b);
    M5U_CANVAS_SET_FONT_CASE(19, FreeMonoOblique12pt7b);
    M5U_CANVAS_SET_FONT_CASE(20, FreeMonoOblique18pt7b);
    M5U_CANVAS_SET_FONT_CASE(21, FreeMonoOblique24pt7b);
    M5U_CANVAS_SET_FONT_CASE(22, FreeMonoBoldOblique9pt7b);
    M5U_CANVAS_SET_FONT_CASE(23, FreeMonoBoldOblique12pt7b);
    M5U_CANVAS_SET_FONT_CASE(24, FreeMonoBoldOblique18pt7b);
    M5U_CANVAS_SET_FONT_CASE(25, FreeMonoBoldOblique24pt7b);
    M5U_CANVAS_SET_FONT_CASE(26, FreeSans9pt7b);
    M5U_CANVAS_SET_FONT_CASE(27, FreeSans12pt7b);
    M5U_CANVAS_SET_FONT_CASE(28, FreeSans18pt7b);
    M5U_CANVAS_SET_FONT_CASE(29, FreeSans24pt7b);
    M5U_CANVAS_SET_FONT_CASE(30, FreeSansBold9pt7b);
    M5U_CANVAS_SET_FONT_CASE(31, FreeSansBold12pt7b);
    M5U_CANVAS_SET_FONT_CASE(32, FreeSansBold18pt7b);
    M5U_CANVAS_SET_FONT_CASE(33, FreeSansBold24pt7b);
    M5U_CANVAS_SET_FONT_CASE(34, FreeSansOblique9pt7b);
    M5U_CANVAS_SET_FONT_CASE(35, FreeSansOblique12pt7b);
    M5U_CANVAS_SET_FONT_CASE(36, FreeSansOblique18pt7b);
    M5U_CANVAS_SET_FONT_CASE(37, FreeSansOblique24pt7b);
    M5U_CANVAS_SET_FONT_CASE(38, FreeSansBoldOblique9pt7b);
    M5U_CANVAS_SET_FONT_CASE(39, FreeSansBoldOblique12pt7b);
    M5U_CANVAS_SET_FONT_CASE(40, FreeSansBoldOblique18pt7b);
    M5U_CANVAS_SET_FONT_CASE(41, FreeSansBoldOblique24pt7b);
    M5U_CANVAS_SET_FONT_CASE(42, FreeSerif9pt7b);
    M5U_CANVAS_SET_FONT_CASE(43, FreeSerif12pt7b);
    M5U_CANVAS_SET_FONT_CASE(44, FreeSerif18pt7b);
    M5U_CANVAS_SET_FONT_CASE(45, FreeSerif24pt7b);
    M5U_CANVAS_SET_FONT_CASE(46, FreeSerifItalic9pt7b);
    M5U_CANVAS_SET_FONT_CASE(47, FreeSerifItalic12pt7b);
    M5U_CANVAS_SET_FONT_CASE(48, FreeSerifItalic18pt7b);
    M5U_CANVAS_SET_FONT_CASE(49, FreeSerifItalic24pt7b);
    M5U_CANVAS_SET_FONT_CASE(50, FreeSerifBold9pt7b);
    M5U_CANVAS_SET_FONT_CASE(51, FreeSerifBold12pt7b);
    M5U_CANVAS_SET_FONT_CASE(52, FreeSerifBold18pt7b);
    M5U_CANVAS_SET_FONT_CASE(53, FreeSerifBold24pt7b);
    M5U_CANVAS_SET_FONT_CASE(54, FreeSerifBoldItalic9pt7b);
    M5U_CANVAS_SET_FONT_CASE(55, FreeSerifBoldItalic12pt7b);
    M5U_CANVAS_SET_FONT_CASE(56, FreeSerifBoldItalic18pt7b);
    M5U_CANVAS_SET_FONT_CASE(57, FreeSerifBoldItalic24pt7b);
    M5U_CANVAS_SET_FONT_CASE(58, Orbitron_Light_24);
    M5U_CANVAS_SET_FONT_CASE(59, Orbitron_Light_32);
    M5U_CANVAS_SET_FONT_CASE(60, Roboto_Thin_24);
    M5U_CANVAS_SET_FONT_CASE(61, Satisfy_24);
    M5U_CANVAS_SET_FONT_CASE(62, Yellowtail_32);
    M5U_CANVAS_SET_FONT_CASE(63, DejaVu9);
    M5U_CANVAS_SET_FONT_CASE(64, DejaVu12);
    M5U_CANVAS_SET_FONT_CASE(65, DejaVu18);
    M5U_CANVAS_SET_FONT_CASE(66, DejaVu24);
    M5U_CANVAS_SET_FONT_CASE(67, DejaVu40);
    M5U_CANVAS_SET_FONT_CASE(68, DejaVu56);
    M5U_CANVAS_SET_FONT_CASE(69, DejaVu72);
    default: return false;
    }
#undef M5U_CANVAS_SET_FONT_CASE
#else
    (void)canvas; return font >= 0 && font <= 69;
#endif
}

int m5u_canvas_font_height(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->fontHeight() : 0;
#else
    (void)canvas;
    return 16;
#endif
}

int m5u_canvas_font_width(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->fontWidth() : 0;
#else
    (void)canvas;
    return 8;
#endif
}

bool m5u_canvas_show_font(m5u_canvas_t* canvas, uint32_t duration_ms) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->showFont(duration_ms) : false;
#else
    (void)canvas; (void)duration_ms;
    return true;
#endif
}

void m5u_canvas_unload_font(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->unloadFont(); }
#else
    (void)canvas;
#endif
}

int m5u_canvas_text_width(m5u_canvas_t* canvas, const char* text) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && text) ? reinterpret_cast<M5Canvas*>(canvas)->textWidth(text) : 0;
#else
    (void)canvas; return text ? (int)strlen(text) * 8 : 0;
#endif
}

void m5u_canvas_print(m5u_canvas_t* canvas, const char* text) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas && text) { reinterpret_cast<M5Canvas*>(canvas)->print(text); }
#else
    (void)canvas; (void)text;
#endif
}

void m5u_canvas_println(m5u_canvas_t* canvas, const char* text) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas && text) { reinterpret_cast<M5Canvas*>(canvas)->println(text); }
#else
    (void)canvas; (void)text;
#endif
}

int m5u_canvas_draw_center_string(m5u_canvas_t* canvas, const char* text, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && text) ? reinterpret_cast<M5Canvas*>(canvas)->drawCenterString(text, x, y) : 0;
#else
    (void)canvas; (void)x; (void)y; return text ? (int)strlen(text) * 8 : 0;
#endif
}

int m5u_canvas_draw_string(m5u_canvas_t* canvas, const char* text, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && text) ? reinterpret_cast<M5Canvas*>(canvas)->drawString(text, x, y) : 0;
#else
    (void)canvas; return text ? (int)strlen(text) * 8 : 0;
#endif
}

void m5u_canvas_draw_line(m5u_canvas_t* canvas, int x0, int y0, int x1, int y1, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawLine(x0, y0, x1, y1, color); }
#else
    (void)canvas; (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
#endif
}

void m5u_canvas_draw_rect(m5u_canvas_t* canvas, int x, int y, int w, int h, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawRect(x, y, w, h, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_canvas_fill_rect(m5u_canvas_t* canvas, int x, int y, int w, int h, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillRect(x, y, w, h, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_canvas_draw_circle(m5u_canvas_t* canvas, int x, int y, int r, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawCircle(x, y, r, color); }
#else
    (void)canvas; (void)x; (void)y; (void)r; (void)color;
#endif
}

void m5u_canvas_fill_circle(m5u_canvas_t* canvas, int x, int y, int r, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillCircle(x, y, r, color); }
#else
    (void)canvas; (void)x; (void)y; (void)r; (void)color;
#endif
}

void m5u_canvas_draw_pixel(m5u_canvas_t* canvas, int x, int y, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawPixel(x, y, color); }
#else
    (void)canvas; (void)x; (void)y; (void)color;
#endif
}

uint16_t m5u_canvas_read_pixel(m5u_canvas_t* canvas, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->readPixel(x, y) : 0;
#else
    (void)canvas; (void)x; (void)y;
    return 0;
#endif
}

void m5u_canvas_draw_fast_hline(m5u_canvas_t* canvas, int x, int y, int w, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawFastHLine(x, y, w, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)color;
#endif
}

void m5u_canvas_draw_fast_vline(m5u_canvas_t* canvas, int x, int y, int h, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawFastVLine(x, y, h, color); }
#else
    (void)canvas; (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_canvas_draw_round_rect(m5u_canvas_t* canvas, int x, int y, int w, int h, int r, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawRoundRect(x, y, w, h, r, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_canvas_fill_round_rect(m5u_canvas_t* canvas, int x, int y, int w, int h, int r, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillRoundRect(x, y, w, h, r, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_canvas_draw_ellipse(m5u_canvas_t* canvas, int x, int y, int rx, int ry, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawEllipse(x, y, rx, ry, color); }
#else
    (void)canvas; (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_canvas_fill_ellipse(m5u_canvas_t* canvas, int x, int y, int rx, int ry, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillEllipse(x, y, rx, ry, color); }
#else
    (void)canvas; (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_canvas_draw_arc(m5u_canvas_t* canvas, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawArc(x, y, r0, r1, angle0, angle1, color); }
#else
    (void)canvas; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_canvas_fill_arc(m5u_canvas_t* canvas, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillArc(x, y, r0, r1, angle0, angle1, color); }
#else
    (void)canvas; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_canvas_draw_triangle(m5u_canvas_t* canvas, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->drawTriangle(x0, y0, x1, y1, x2, y2, color); }
#else
    (void)canvas; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_canvas_fill_triangle(m5u_canvas_t* canvas, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->fillTriangle(x0, y0, x1, y1, x2, y2, color); }
#else
    (void)canvas; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_canvas_progress_bar(m5u_canvas_t* canvas, int x, int y, int w, int h, uint8_t value) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->progressBar(x, y, w, h, value); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)value;
#endif
}

int m5u_canvas_text_length(m5u_canvas_t* canvas, const char* text) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && text) ? reinterpret_cast<M5Canvas*>(canvas)->textLength(text) : 0;
#else
    (void)canvas; return text ? (int)strlen(text) * 8 : 0;
#endif
}

int m5u_canvas_draw_char(m5u_canvas_t* canvas, uint32_t codepoint, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->drawChar(codepoint, x, y) : 0;
#else
    (void)canvas; (void)codepoint; (void)x; (void)y; return 8;
#endif
}

int m5u_canvas_draw_number(m5u_canvas_t* canvas, int32_t value, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->drawNumber(value, x, y) : 0;
#else
    (void)canvas; (void)value; (void)x; (void)y; return 0;
#endif
}

int m5u_canvas_draw_float(m5u_canvas_t* canvas, float value, uint8_t decimals, int x, int y) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return canvas ? reinterpret_cast<M5Canvas*>(canvas)->drawFloat(value, decimals, x, y) : 0;
#else
    (void)canvas; (void)value; (void)decimals; (void)x; (void)y; return 0;
#endif
}

bool m5u_canvas_draw_bmp(m5u_canvas_t* canvas, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && data) ? reinterpret_cast<M5Canvas*>(canvas)->drawBmp(data, len, x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, datum) : false;
#else
    (void)canvas; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum; return false;
#endif
}

bool m5u_canvas_draw_jpg(m5u_canvas_t* canvas, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && data) ? reinterpret_cast<M5Canvas*>(canvas)->drawJpg(data, len, x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, datum) : false;
#else
    (void)canvas; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum; return false;
#endif
}

bool m5u_canvas_draw_png(m5u_canvas_t* canvas, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    return (canvas && data) ? reinterpret_cast<M5Canvas*>(canvas)->drawPng(data, len, x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, datum) : false;
#else
    (void)canvas; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum; return false;
#endif
}

bool m5u_canvas_push_image_rgb565(m5u_canvas_t* canvas, int x, int y, int w, int h, const uint16_t* data, size_t len) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (!canvas || !data || w <= 0 || h <= 0 || len < static_cast<size_t>(w) * static_cast<size_t>(h)) { return false; }
    reinterpret_cast<M5Canvas*>(canvas)->pushImage(x, y, w, h, data);
    return true;
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)data; (void)len; return false;
#endif
}

void m5u_canvas_write_pixel(m5u_canvas_t* canvas, int x, int y, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->writePixel(x, y, color); }
#else
    (void)canvas; (void)x; (void)y; (void)color;
#endif
}

void m5u_canvas_write_fast_vline(m5u_canvas_t* canvas, int x, int y, int h, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->writeFastVLine(x, y, h, color); }
#else
    (void)canvas; (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_canvas_set_addr_window(m5u_canvas_t* canvas, int x, int y, int w, int h) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setAddrWindow(x, y, w, h); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_canvas_set_window(m5u_canvas_t* canvas, int xs, int ys, int xe, int ye) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setWindow((uint_fast16_t)xs, (uint_fast16_t)ys, (uint_fast16_t)xe, (uint_fast16_t)ye); }
#else
    (void)canvas; (void)xs; (void)ys; (void)xe; (void)ye;
#endif
}

void m5u_canvas_set_clip_rect(m5u_canvas_t* canvas, int x, int y, int w, int h) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setClipRect(x, y, w, h); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_canvas_get_clip_rect(m5u_canvas_t* canvas, int* x, int* y, int* w, int* h) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) {
        reinterpret_cast<M5Canvas*>(canvas)->getClipRect(x, y, w, h);
        return;
    }
#else
    (void)canvas;
#endif
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_canvas_clear_clip_rect(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->clearClipRect(); }
#else
    (void)canvas;
#endif
}

void m5u_canvas_scroll(m5u_canvas_t* canvas, int dx, int dy) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->scroll(dx, dy); }
#else
    (void)canvas; (void)dx; (void)dy;
#endif
}

void m5u_canvas_set_scroll_rect(m5u_canvas_t* canvas, int x, int y, int w, int h, uint16_t color) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->setScrollRect(x, y, w, h, color); }
#else
    (void)canvas; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_canvas_get_scroll_rect(m5u_canvas_t* canvas, int* x, int* y, int* w, int* h) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) {
        reinterpret_cast<M5Canvas*>(canvas)->getScrollRect(x, y, w, h);
        return;
    }
#else
    (void)canvas;
#endif
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_canvas_clear_scroll_rect(m5u_canvas_t* canvas) {
#if defined(M5UNIFIED_RS_USE_REAL_M5UNIFIED) || defined(M5UNIFIED_RS_USE_REAL_M5CARDPUTER)
    if (canvas) { reinterpret_cast<M5Canvas*>(canvas)->clearScrollRect(); }
#else
    (void)canvas;
#endif
}

int m5u_display_count(void) {
    return M5.getDisplayCount();
}

int m5u_display_index_for_kind(int kind) {
    return M5.getDisplayIndex((m5::board_t)kind);
}

bool m5u_display_set_primary(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (index < 0) {
        return false;
    }
    return M5.setPrimaryDisplay((size_t)index);
#else
    return index == 0;
#endif
}

bool m5u_display_set_primary_kind(int kind) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.setPrimaryDisplayType((m5::board_t)kind);
#else
    (void)kind;
    return false;
#endif
}

void m5u_display_set_rotation_at(int index, int rotation) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setRotation(rotation);
#else
    (void)index; (void)rotation;
#endif
}

int m5u_display_get_rotation_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getRotation();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_brightness_at(int index, uint8_t brightness) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setBrightness(brightness);
#else
    (void)index; (void)brightness;
#endif
}

uint8_t m5u_display_get_brightness_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getBrightness();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_color_depth_at(int index, uint8_t depth) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setColorDepth(depth);
#else
    (void)index; (void)depth;
#endif
}

uint8_t m5u_display_get_color_depth_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<uint8_t>(M5.Displays(index).getColorDepth());
#else
    (void)index;
    return 16;
#endif
}

bool m5u_display_is_epd_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).isEPD();
#else
    (void)index;
    return false;
#endif
}

void m5u_display_set_epd_mode_at(int index, int mode) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    switch (mode) {
    case 1: M5.Displays(index).setEpdMode(m5gfx::epd_quality); break;
    case 2: M5.Displays(index).setEpdMode(m5gfx::epd_text); break;
    case 3: M5.Displays(index).setEpdMode(m5gfx::epd_fast); break;
    case 4: M5.Displays(index).setEpdMode(m5gfx::epd_fastest); break;
    default: break;
    }
#else
    (void)index; (void)mode;
#endif
}

int m5u_display_get_epd_mode_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return static_cast<int>(M5.Displays(index).getEpdMode());
#else
    (void)index;
    return 0;
#endif
}

bool m5u_display_set_resolution_at(int index, uint16_t logical_width, uint16_t logical_height, float refresh_rate, uint16_t output_width, uint16_t output_height, uint8_t scale_w, uint8_t scale_h, uint32_t pixel_clock) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).setResolution(logical_width, logical_height, refresh_rate, output_width, output_height, scale_w, scale_h, pixel_clock);
#else
    (void)index; (void)logical_width; (void)logical_height; (void)refresh_rate; (void)output_width; (void)output_height; (void)scale_w; (void)scale_h; (void)pixel_clock;
    return false;
#endif
}

int m5u_display_width_at(int index) {
    return M5.Displays(index).width();
}

int m5u_display_height_at(int index) {
    return M5.Displays(index).height();
}

void m5u_display_start_write_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).startWrite();
#else
    (void)index;
#endif
}

void m5u_display_end_write_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).endWrite();
#else
    (void)index;
#endif
}

void m5u_display_display_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).display();
#else
    (void)index;
#endif
}

bool m5u_display_display_busy_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).displayBusy();
#else
    (void)index;
    return false;
#endif
}

void m5u_display_wait_display_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).waitDisplay();
#else
    (void)index;
#endif
}

void m5u_display_sleep_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).sleep();
#else
    (void)index;
#endif
}

void m5u_display_wakeup_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).wakeup();
#else
    (void)index;
#endif
}

void m5u_display_power_save_on_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).powerSaveOn();
#else
    (void)index;
#endif
}

void m5u_display_power_save_off_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).powerSaveOff();
#else
    (void)index;
#endif
}

void m5u_display_power_save_at(int index, bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).powerSave(enable);
#else
    (void)index; (void)enable;
#endif
}

void m5u_display_invert_display_at(int index, bool invert) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).invertDisplay(invert);
#else
    (void)index; (void)invert;
#endif
}

int m5u_display_get_cursor_x_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getCursorX();
#else
    (void)index;
    return 0;
#endif
}

int m5u_display_get_cursor_y_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getCursorY();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_pivot_at(int index, float x, float y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setPivot(x, y);
#else
    (void)index; (void)x; (void)y;
#endif
}

float m5u_display_get_pivot_x_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getPivotX();
#else
    (void)index;
    return 0.0f;
#endif
}

float m5u_display_get_pivot_y_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getPivotY();
#else
    (void)index;
    return 0.0f;
#endif
}

void m5u_display_clear_at(int index, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).clear(color);
#else
    (void)index; (void)color;
#endif
}

void m5u_display_set_cursor_at(int index, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setCursor(x, y);
#else
    (void)index; (void)x; (void)y;
#endif
}

void m5u_display_set_text_size_at(int index, int size) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextSize(size);
#else
    (void)index; (void)size;
#endif
}

void m5u_display_set_text_color_at(int index, uint16_t fg, uint16_t bg) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextColor(fg, bg);
#else
    (void)index; (void)fg; (void)bg;
#endif
}

void m5u_display_set_text_datum_at(int index, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextDatum((textdatum_t)datum);
#else
    (void)index; (void)datum;
#endif
}

int m5u_display_get_text_datum_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getTextDatum();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_text_padding_at(int index, uint32_t padding_x) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextPadding(padding_x);
#else
    (void)index; (void)padding_x;
#endif
}

uint32_t m5u_display_get_text_padding_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getTextPadding();
#else
    (void)index;
    return 0;
#endif
}

uint8_t m5u_display_get_text_size_x_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getTextSizeX();
#else
    (void)index;
    return 1;
#endif
}

uint8_t m5u_display_get_text_size_y_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getTextSizeY();
#else
    (void)index;
    return 1;
#endif
}

int m5u_display_font_height_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).fontHeight();
#else
    (void)index;
    return 16;
#endif
}

int m5u_display_font_width_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).fontWidth();
#else
    (void)index;
    return 8;
#endif
}

bool m5u_display_set_font_at(int index, int font) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
#define M5U_SET_FONT_AT_CASE(id, name) case id: M5.Displays(index).setFont(&fonts::name); return true
    switch (font) {
    M5U_SET_FONT_AT_CASE(0, Font0);
    M5U_SET_FONT_AT_CASE(1, Font2);
    M5U_SET_FONT_AT_CASE(2, Font4);
    M5U_SET_FONT_AT_CASE(3, Font6);
    M5U_SET_FONT_AT_CASE(4, Font7);
    M5U_SET_FONT_AT_CASE(5, Font8);
    M5U_SET_FONT_AT_CASE(6, Font8x8C64);
    M5U_SET_FONT_AT_CASE(7, AsciiFont8x16);
    M5U_SET_FONT_AT_CASE(8, AsciiFont24x48);
    M5U_SET_FONT_AT_CASE(9, TomThumb);
    M5U_SET_FONT_AT_CASE(10, FreeMono9pt7b);
    M5U_SET_FONT_AT_CASE(11, FreeMono12pt7b);
    M5U_SET_FONT_AT_CASE(12, FreeMono18pt7b);
    M5U_SET_FONT_AT_CASE(13, FreeMono24pt7b);
    M5U_SET_FONT_AT_CASE(14, FreeMonoBold9pt7b);
    M5U_SET_FONT_AT_CASE(15, FreeMonoBold12pt7b);
    M5U_SET_FONT_AT_CASE(16, FreeMonoBold18pt7b);
    M5U_SET_FONT_AT_CASE(17, FreeMonoBold24pt7b);
    M5U_SET_FONT_AT_CASE(18, FreeMonoOblique9pt7b);
    M5U_SET_FONT_AT_CASE(19, FreeMonoOblique12pt7b);
    M5U_SET_FONT_AT_CASE(20, FreeMonoOblique18pt7b);
    M5U_SET_FONT_AT_CASE(21, FreeMonoOblique24pt7b);
    M5U_SET_FONT_AT_CASE(22, FreeMonoBoldOblique9pt7b);
    M5U_SET_FONT_AT_CASE(23, FreeMonoBoldOblique12pt7b);
    M5U_SET_FONT_AT_CASE(24, FreeMonoBoldOblique18pt7b);
    M5U_SET_FONT_AT_CASE(25, FreeMonoBoldOblique24pt7b);
    M5U_SET_FONT_AT_CASE(26, FreeSans9pt7b);
    M5U_SET_FONT_AT_CASE(27, FreeSans12pt7b);
    M5U_SET_FONT_AT_CASE(28, FreeSans18pt7b);
    M5U_SET_FONT_AT_CASE(29, FreeSans24pt7b);
    M5U_SET_FONT_AT_CASE(30, FreeSansBold9pt7b);
    M5U_SET_FONT_AT_CASE(31, FreeSansBold12pt7b);
    M5U_SET_FONT_AT_CASE(32, FreeSansBold18pt7b);
    M5U_SET_FONT_AT_CASE(33, FreeSansBold24pt7b);
    M5U_SET_FONT_AT_CASE(34, FreeSansOblique9pt7b);
    M5U_SET_FONT_AT_CASE(35, FreeSansOblique12pt7b);
    M5U_SET_FONT_AT_CASE(36, FreeSansOblique18pt7b);
    M5U_SET_FONT_AT_CASE(37, FreeSansOblique24pt7b);
    M5U_SET_FONT_AT_CASE(38, FreeSansBoldOblique9pt7b);
    M5U_SET_FONT_AT_CASE(39, FreeSansBoldOblique12pt7b);
    M5U_SET_FONT_AT_CASE(40, FreeSansBoldOblique18pt7b);
    M5U_SET_FONT_AT_CASE(41, FreeSansBoldOblique24pt7b);
    M5U_SET_FONT_AT_CASE(42, FreeSerif9pt7b);
    M5U_SET_FONT_AT_CASE(43, FreeSerif12pt7b);
    M5U_SET_FONT_AT_CASE(44, FreeSerif18pt7b);
    M5U_SET_FONT_AT_CASE(45, FreeSerif24pt7b);
    M5U_SET_FONT_AT_CASE(46, FreeSerifItalic9pt7b);
    M5U_SET_FONT_AT_CASE(47, FreeSerifItalic12pt7b);
    M5U_SET_FONT_AT_CASE(48, FreeSerifItalic18pt7b);
    M5U_SET_FONT_AT_CASE(49, FreeSerifItalic24pt7b);
    M5U_SET_FONT_AT_CASE(50, FreeSerifBold9pt7b);
    M5U_SET_FONT_AT_CASE(51, FreeSerifBold12pt7b);
    M5U_SET_FONT_AT_CASE(52, FreeSerifBold18pt7b);
    M5U_SET_FONT_AT_CASE(53, FreeSerifBold24pt7b);
    M5U_SET_FONT_AT_CASE(54, FreeSerifBoldItalic9pt7b);
    M5U_SET_FONT_AT_CASE(55, FreeSerifBoldItalic12pt7b);
    M5U_SET_FONT_AT_CASE(56, FreeSerifBoldItalic18pt7b);
    M5U_SET_FONT_AT_CASE(57, FreeSerifBoldItalic24pt7b);
    M5U_SET_FONT_AT_CASE(58, Orbitron_Light_24);
    M5U_SET_FONT_AT_CASE(59, Orbitron_Light_32);
    M5U_SET_FONT_AT_CASE(60, Roboto_Thin_24);
    M5U_SET_FONT_AT_CASE(61, Satisfy_24);
    M5U_SET_FONT_AT_CASE(62, Yellowtail_32);
    M5U_SET_FONT_AT_CASE(63, DejaVu9);
    M5U_SET_FONT_AT_CASE(64, DejaVu12);
    M5U_SET_FONT_AT_CASE(65, DejaVu18);
    M5U_SET_FONT_AT_CASE(66, DejaVu24);
    M5U_SET_FONT_AT_CASE(67, DejaVu40);
    M5U_SET_FONT_AT_CASE(68, DejaVu56);
    M5U_SET_FONT_AT_CASE(69, DejaVu72);
    default: return false;
    }
#undef M5U_SET_FONT_AT_CASE
#else
    (void)index;
    return font >= 0 && font <= 69;
#endif
}

bool m5u_display_show_font_at(int index, uint32_t duration_ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).showFont(duration_ms);
    return true;
#else
    (void)index; (void)duration_ms;
    return true;
#endif
}

void m5u_display_unload_font_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).unloadFont();
#else
    (void)index;
#endif
}

uint16_t m5u_display_get_base_color_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getBaseColor();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_base_color_at(int index, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setBaseColor(color);
#else
    (void)index; (void)color;
#endif
}

void m5u_display_set_color_at(int index, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setColor(color);
#else
    (void)index; (void)color;
#endif
}

void m5u_display_set_rgb_color_at(int index, uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setColor(r, g, b);
#else
    (void)index; (void)r; (void)g; (void)b;
#endif
}

void m5u_display_set_raw_color_at(int index, uint32_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setRawColor(color);
#else
    (void)index; (void)color;
#endif
}

uint32_t m5u_display_get_raw_color_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getRawColor();
#else
    (void)index;
    return 0;
#endif
}

uint32_t m5u_display_get_palette_count_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getPaletteCount();
#else
    (void)index;
    return 0;
#endif
}

void m5u_display_set_swap_bytes_at(int index, bool swap) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setSwapBytes(swap);
#else
    (void)index; (void)swap;
#endif
}

bool m5u_display_get_swap_bytes_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).getSwapBytes();
#else
    (void)index;
    return false;
#endif
}

uint16_t m5u_display_swap565_at(int index, uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).swap565(r, g, b);
#else
    (void)index;
    uint16_t rgb565 = ((uint16_t)(r & 0xF8) << 8) | ((uint16_t)(g & 0xFC) << 3) | ((uint16_t)b >> 3);
    return (uint16_t)((rgb565 << 8) | (rgb565 >> 8));
#endif
}

uint32_t m5u_display_swap888_at(int index, uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).swap888(r, g, b);
#else
    (void)index;
    return ((uint32_t)b << 16) | ((uint32_t)g << 8) | (uint32_t)r;
#endif
}

void m5u_display_set_text_wrap_at(int index, bool wrap_x, bool wrap_y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextWrap(wrap_x, wrap_y);
#else
    (void)index; (void)wrap_x; (void)wrap_y;
#endif
}

uint16_t m5u_display_color888_at(int index, uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).color888(r, g, b);
#else
    (void)index;
    return ((uint16_t)(r & 0xF8) << 8) | ((uint16_t)(g & 0xFC) << 3) | ((uint16_t)b >> 3);
#endif
}

int m5u_display_text_length_at(int index, const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).textLength(text);
#else
    (void)index;
    return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

int m5u_display_text_width_at(int index, const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).textWidth(text);
#else
    (void)index;
    return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

void m5u_display_print_at(int index, const char* text) {
    M5.Displays(index).print(text);
}

void m5u_display_println_at(int index, const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).println(text);
#else
    (void)index; (void)text;
#endif
}

int m5u_display_draw_center_string_at(int index, const char* text, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).drawCenterString(text, x, y);
#else
    (void)index; (void)x; (void)y;
    return text ? static_cast<int>(strlen(text)) * 8 : 0;
#endif
}

int m5u_display_draw_string_at(int index, const char* text, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).drawString(text, x, y);
#else
    (void)index; (void)text; (void)x; (void)y;
    return 0;
#endif
}

int m5u_display_draw_char_at(int index, uint32_t codepoint, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).drawChar(codepoint, x, y);
#else
    (void)index; (void)codepoint; (void)x; (void)y;
    return 8;
#endif
}

int m5u_display_draw_number_at(int index, int32_t value, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).drawNumber(value, x, y);
#else
    (void)index; (void)value; (void)x; (void)y;
    return 0;
#endif
}

int m5u_display_draw_float_at(int index, float value, uint8_t decimals, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).drawFloat(value, decimals, x, y);
#else
    (void)index; (void)value; (void)decimals; (void)x; (void)y;
    return 0;
#endif
}

bool m5u_display_draw_bmp_at(int index, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Displays(index).drawBmp(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)index; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

bool m5u_display_draw_jpg_at(int index, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Displays(index).drawJpg(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)index; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

bool m5u_display_draw_png_at(int index, const uint8_t* data, size_t len, int x, int y, int max_width, int max_height, int off_x, int off_y, float scale_x, float scale_y, int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || len == 0) { return false; }
    M5.Displays(index).drawPng(data, static_cast<uint32_t>(len), x, y, max_width, max_height, off_x, off_y, scale_x, scale_y, static_cast<datum_t>(datum));
    return true;
#else
    (void)index; (void)data; (void)len; (void)x; (void)y; (void)max_width; (void)max_height; (void)off_x; (void)off_y; (void)scale_x; (void)scale_y; (void)datum;
    return false;
#endif
}

void m5u_display_draw_line_at(int index, int x0, int y0, int x1, int y1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawLine(x0, y0, x1, y1, color);
#else
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
#endif
}

void m5u_display_draw_pixel_at(int index, int x, int y, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawPixel(x, y, color);
#else
    (void)index; (void)x; (void)y; (void)color;
#endif
}

uint16_t m5u_display_read_pixel_at(int index, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).readPixel(x, y);
#else
    (void)index; (void)x; (void)y;
    return 0;
#endif
}

void m5u_display_draw_fast_hline_at(int index, int x, int y, int w, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawFastHLine(x, y, w, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)color;
#endif
}

void m5u_display_draw_fast_vline_at(int index, int x, int y, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawFastVLine(x, y, h, color);
#else
    (void)index; (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_display_draw_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawRect(x, y, w, h, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_fill_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillRect(x, y, w, h, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_fill_rect_alpha_at(int index, int x, int y, int w, int h, uint8_t alpha, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillRectAlpha(x, y, w, h, alpha, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)alpha; (void)color;
#endif
}

void m5u_display_draw_round_rect_at(int index, int x, int y, int w, int h, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawRoundRect(x, y, w, h, r, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_display_fill_round_rect_at(int index, int x, int y, int w, int h, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillRoundRect(x, y, w, h, r, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
#endif
}

void m5u_display_draw_circle_at(int index, int x, int y, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawCircle(x, y, r, color);
#else
    (void)index; (void)x; (void)y; (void)r; (void)color;
#endif
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
    M5.Displays(index).fillCircle(x, y, r, color);
}

void m5u_display_draw_ellipse_at(int index, int x, int y, int rx, int ry, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawEllipse(x, y, rx, ry, color);
#else
    (void)index; (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_display_fill_ellipse_at(int index, int x, int y, int rx, int ry, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillEllipse(x, y, rx, ry, color);
#else
    (void)index; (void)x; (void)y; (void)rx; (void)ry; (void)color;
#endif
}

void m5u_display_draw_arc_at(int index, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawArc(x, y, r0, r1, angle0, angle1, color);
#else
    (void)index; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_display_fill_arc_at(int index, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillArc(x, y, r0, r1, angle0, angle1, color);
#else
    (void)index; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
#endif
}

void m5u_display_draw_triangle_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).drawTriangle(x0, y0, x1, y1, x2, y2, color);
#else
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_display_fill_triangle_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillTriangle(x0, y0, x1, y1, x2, y2, color);
#else
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
#endif
}

void m5u_display_progress_bar_at(int index, int x, int y, int w, int h, uint8_t value) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).progressBar(x, y, w, h, value);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)value;
#endif
}

bool m5u_display_push_image_rgb565_at(int index, int x, int y, int w, int h, const uint16_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!data || w <= 0 || h <= 0 || len < static_cast<size_t>(w) * static_cast<size_t>(h)) { return false; }
    M5.Displays(index).pushImage(x, y, w, h, data);
    return true;
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)data; (void)len;
    return false;
#endif
}

void m5u_display_write_pixel_at(int index, int x, int y, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).writePixel(x, y, color);
#else
    (void)index; (void)x; (void)y; (void)color;
#endif
}

void m5u_display_write_fast_vline_at(int index, int x, int y, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).writeFastVLine(x, y, h, color);
#else
    (void)index; (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_display_set_addr_window_at(int index, int x, int y, int w, int h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setAddrWindow(x, y, w, h);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_display_set_window_at(int index, int xs, int ys, int xe, int ye) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setWindow((uint_fast16_t)xs, (uint_fast16_t)ys, (uint_fast16_t)xe, (uint_fast16_t)ye);
#else
    (void)index; (void)xs; (void)ys; (void)xe; (void)ye;
#endif
}

void m5u_display_set_clip_rect_at(int index, int x, int y, int w, int h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setClipRect(x, y, w, h);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_display_get_clip_rect_at(int index, int* x, int* y, int* w, int* h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).getClipRect(x, y, w, h);
#else
    (void)index;
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 320; }
    if (h) { *h = 240; }
#endif
}

void m5u_display_clear_clip_rect_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).clearClipRect();
#else
    (void)index;
#endif
}

void m5u_display_scroll_at(int index, int dx, int dy) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).scroll(dx, dy);
#else
    (void)index; (void)dx; (void)dy;
#endif
}

void m5u_display_set_text_scroll_at(int index, bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setTextScroll(enable);
#else
    (void)index; (void)enable;
#endif
}

void m5u_display_set_scroll_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).setScrollRect(x, y, w, h, color);
#else
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_get_scroll_rect_at(int index, int* x, int* y, int* w, int* h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).getScrollRect(x, y, w, h);
#else
    (void)index;
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
#endif
}

void m5u_display_clear_scroll_rect_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).clearScrollRect();
#else
    (void)index;
#endif
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
bool m5u_button_was_release_for(int button, uint32_t ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->wasReleaseFor(ms) : false;
#else
    (void)button; (void)ms; return false;
#endif
}
bool m5u_button_pressed_for(int button, uint32_t ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->pressedFor(ms) : false;
#else
    (void)button; (void)ms; return false;
#endif
}
bool m5u_button_released_for(int button, uint32_t ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->releasedFor(ms) : false;
#else
    (void)button; (void)ms; return false;
#endif
}
int m5u_button_get_click_count(int button) {
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->getClickCount() : 0;
}

void m5u_button_set_debounce_thresh(int button, uint32_t msec) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    if (btn) { btn->setDebounceThresh(msec); }
#else
    (void)button; (void)msec;
#endif
}

void m5u_button_set_hold_thresh(int button, uint32_t msec) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    if (btn) { btn->setHoldThresh(msec); }
#else
    (void)button; (void)msec;
#endif
}

void m5u_button_set_raw_state(int button, uint32_t msec, bool press) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    if (btn) { btn->setRawState(msec, press); }
#else
    (void)button; (void)msec; (void)press;
#endif
}

void m5u_button_set_state(int button, uint32_t msec, int state) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    if (btn) {
        btn->setState(msec, static_cast<m5::Button_Class::button_state_t>(state));
    }
#else
    (void)button; (void)msec; (void)state;
#endif
}

int m5u_button_get_state(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? static_cast<int>(btn->getState()) : 0;
#else
    (void)button; return 0;
#endif
}

uint32_t m5u_button_last_change(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->lastChange() : 0;
#else
    (void)button; return 0;
#endif
}

uint32_t m5u_button_get_debounce_thresh(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->getDebounceThresh() : 0;
#else
    (void)button; return 0;
#endif
}

uint32_t m5u_button_get_hold_thresh(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->getHoldThresh() : 0;
#else
    (void)button; return 0;
#endif
}

uint32_t m5u_button_get_update_msec(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = m5u_button_from_int(button);
    return btn ? btn->getUpdateMsec() : 0;
#else
    (void)button; return 0;
#endif
}

bool m5u_mic_is_enabled(void) {
    return M5.Mic.isEnabled();
}

bool m5u_mic_is_recording(void) {
    return M5.Mic.isRecording();
}

void m5u_mic_end(void) {
    M5.Mic.end();
}

bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz) {
    return M5.Mic.record(buffer, samples, sample_rate_hz);
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

bool m5u_mic_get_config(m5u_mic_config_t* out) {
    if (!out) { return false; }
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
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
    out->i2s_port = static_cast<int>(cfg.i2s_port);
    return true;
#else
    out->pin_data_in = -1;
    out->pin_bck = -1;
    out->pin_mck = -1;
    out->pin_ws = -1;
    out->sample_rate = 16000;
    out->left_channel = false;
    out->stereo = false;
    out->over_sampling = 2;
    out->magnification = 16;
    out->noise_filter_level = 0;
    out->use_adc = false;
    out->dma_buf_len = 128;
    out->dma_buf_count = 8;
    out->task_priority = 2;
    out->task_pinned_core = 255;
    out->i2s_port = 0;
    return true;
#endif
}

void m5u_mic_set_config(const m5u_mic_config_t* config) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!config) { return; }
    auto cfg = M5.Mic.config();
    cfg.pin_data_in = config->pin_data_in;
    cfg.pin_bck = config->pin_bck;
    cfg.pin_mck = config->pin_mck;
    cfg.pin_ws = config->pin_ws;
    cfg.sample_rate = config->sample_rate;
    cfg.left_channel = config->left_channel;
    cfg.stereo = config->stereo;
    cfg.over_sampling = config->over_sampling;
    cfg.magnification = config->magnification;
    cfg.noise_filter_level = config->noise_filter_level;
    cfg.use_adc = config->use_adc;
    cfg.dma_buf_len = config->dma_buf_len;
    cfg.dma_buf_count = config->dma_buf_count;
    cfg.task_priority = config->task_priority;
    cfg.task_pinned_core = config->task_pinned_core;
    cfg.i2s_port = static_cast<i2s_port_t>(config->i2s_port);
    M5.Mic.config(cfg);
#else
    (void)config;
#endif
}

bool m5u_speaker_is_enabled(void) {
    return M5.Speaker.isEnabled();
}

bool m5u_speaker_is_running(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.isRunning();
#else
    return false;
#endif
}

void m5u_speaker_end(void) {
    M5.Speaker.end();
}

bool m5u_speaker_get_config(m5u_speaker_config_t* out) {
    if (!out) { return false; }
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    auto cfg = M5.Speaker.config();
    out->pin_data_out = cfg.pin_data_out;
    out->pin_bck = cfg.pin_bck;
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
    out->i2s_port = static_cast<int>(cfg.i2s_port);
    return true;
#else
    out->pin_data_out = -1;
    out->pin_bck = -1;
    out->pin_ws = -1;
    out->sample_rate = 48000;
    out->stereo = false;
    out->buzzer = false;
    out->use_dac = false;
    out->dac_zero_level = 0;
    out->magnification = 16;
    out->dma_buf_len = 256;
    out->dma_buf_count = 8;
    out->task_priority = 2;
    out->task_pinned_core = 255;
    out->i2s_port = 0;
    return true;
#endif
}

void m5u_speaker_set_config(const m5u_speaker_config_t* config) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!config) { return; }
    auto cfg = M5.Speaker.config();
    cfg.pin_data_out = config->pin_data_out;
    cfg.pin_bck = config->pin_bck;
    cfg.pin_ws = config->pin_ws;
    cfg.sample_rate = config->sample_rate;
    cfg.stereo = config->stereo;
    cfg.buzzer = config->buzzer;
    cfg.use_dac = config->use_dac;
    cfg.dac_zero_level = config->dac_zero_level;
    cfg.magnification = config->magnification;
    cfg.dma_buf_len = config->dma_buf_len;
    cfg.dma_buf_count = config->dma_buf_count;
    cfg.task_priority = config->task_priority;
    cfg.task_pinned_core = config->task_pinned_core;
    cfg.i2s_port = static_cast<i2s_port_t>(config->i2s_port);
    M5.Speaker.config(cfg);
#else
    (void)config;
#endif
}

uint8_t m5u_speaker_get_volume(void) {
    return M5.Speaker.getVolume();
}

bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel) {
    return M5.Speaker.tone(frequency_hz, duration_ms, channel);
}

bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz) {
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, false, 1, 0);
}

bool m5u_speaker_play_wav(const uint8_t* data, size_t len) {
    return M5.Speaker.playWav(data, len);
}

bool m5u_speaker_play_i16_ex(const int16_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, stereo, repeat, channel, stop_current_sound);
#else
    (void)samples; (void)len; (void)sample_rate_hz; (void)stereo; (void)repeat; (void)channel; (void)stop_current_sound; return false;
#endif
}

bool m5u_speaker_play_u8_ex(const uint8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, stereo, repeat, channel, stop_current_sound);
#else
    (void)samples; (void)len; (void)sample_rate_hz; (void)stereo; (void)repeat; (void)channel; (void)stop_current_sound; return false;
#endif
}

bool m5u_speaker_play_wav_ex(const uint8_t* data, size_t len, uint32_t repeat, int channel, bool stop_current_sound) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playWav(data, len, repeat, channel, stop_current_sound);
#else
    (void)data; (void)len; (void)repeat; (void)channel; (void)stop_current_sound; return false;
#endif
}

bool m5u_speaker_is_playing(int channel) {
    return channel < 0 ? M5.Speaker.isPlaying() : M5.Speaker.isPlaying(channel);
}

size_t m5u_speaker_get_playing_channels(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.getPlayingChannels();
#else
    return 0;
#endif
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

bool m5u_imu_sleep(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.sleep();
#else
    return false;
#endif
}

bool m5u_imu_load_offset_from_nvs(void) {
    return M5.Imu.loadOffsetFromNVS();
}

bool m5u_imu_save_offset_to_nvs(void) {
    return M5.Imu.saveOffsetToNVS();
}

void m5u_imu_clear_offset_data(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Imu.clearOffsetData();
#endif
}

int32_t m5u_imu_get_offset_data(size_t index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getOffsetData(index);
#else
    (void)index; return 0;
#endif
}

void m5u_imu_set_offset_data(size_t index, int32_t value) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Imu.setOffsetData(index, value);
#else
    (void)index; (void)value;
#endif
}

int16_t m5u_imu_get_raw_data(size_t index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getRawData(index);
#else
    (void)index; return 0;
#endif
}

bool m5u_imu_set_int_pin_active_logic(bool level) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.setINTPinActiveLogic(level);
#else
    (void)level; return false;
#endif
}

void m5u_imu_set_calibration(uint8_t accel_strength, uint8_t gyro_strength, uint8_t mag_strength) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Imu.setCalibration(accel_strength, gyro_strength, mag_strength);
#else
    (void)accel_strength; (void)gyro_strength; (void)mag_strength;
#endif
}

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
    if (!out || index < 0 || index >= M5.Touch.getCount()) { return false; }
    auto d = M5.Touch.getDetail(index);
    out->x = d.x;
    out->y = d.y;
    out->size = d.size;
    out->id = d.id;
    out->prev_x = d.prev_x;
    out->prev_y = d.prev_y;
    out->base_x = d.base_x;
    out->base_y = d.base_y;
    out->base_msec = d.base_msec;
    out->state = static_cast<int>(d.state);
    out->is_pressed = d.isPressed();
    out->was_pressed = d.wasPressed();
    out->is_released = d.isReleased();
    out->was_released = d.wasReleased();
    out->was_clicked = d.wasClicked();
    out->was_hold = d.wasHold();
    out->is_holding = d.isHolding();
    out->was_flick_start = d.wasFlickStart();
    out->is_flicking = d.isFlicking();
    out->was_flicked = d.wasFlicked();
    out->was_drag_start = d.wasDragStart();
    out->is_dragging = d.isDragging();
    out->was_dragged = d.wasDragged();
    out->click_count = d.getClickCount();
    return true;
}

void m5u_set_touch_button_height(uint16_t pixel) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.setTouchButtonHeight(pixel);
#else
    (void)pixel;
#endif
}

void m5u_set_touch_button_height_by_ratio(uint8_t ratio) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.setTouchButtonHeightByRatio(ratio);
#else
    (void)ratio;
#endif
}

uint16_t m5u_get_touch_button_height(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.getTouchButtonHeight();
#else
    return 0;
#endif
}

bool m5u_rtc_is_enabled(void) {
    return M5.Rtc.isEnabled();
}

bool m5u_power_axp2101_disable_irq(uint64_t mask) { (void)mask; return false; }
bool m5u_power_axp2101_enable_irq(uint64_t mask) { (void)mask; return false; }
bool m5u_power_axp2101_clear_irq_statuses(void) { return false; }
uint64_t m5u_power_axp2101_get_irq_statuses(void) { return 0; }
bool m5u_power_axp2101_is_bat_charger_under_temperature_irq(void) { return false; }
bool m5u_power_axp2101_is_bat_charger_over_temperature_irq(void) { return false; }
bool m5u_power_axp2101_is_vbus_insert_irq(void) { return false; }
bool m5u_power_axp2101_is_vbus_remove_irq(void) { return false; }

void m5u_log_print(const char* text) {
    M5.Log.print(text);
}

void m5u_log_level(int level, const char* text) {
    M5.Log((esp_log_level_t)level, "%s", text);
}

void m5u_log_set_level(int target, int level) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Log.setLogLevel(m5u_log_target_from_int(target), (esp_log_level_t)level);
#else
    (void)target; (void)level;
#endif
}

int m5u_log_get_level(int target) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Log.getLogLevel(m5u_log_target_from_int(target));
#else
    (void)target;
    return 3;
#endif
}

void m5u_log_set_enable_color(int target, bool enable) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Log.setEnableColor(m5u_log_target_from_int(target), enable);
#else
    (void)target; (void)enable;
#endif
}

bool m5u_log_get_enable_color(int target) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Log.getEnableColor(m5u_log_target_from_int(target));
#else
    (void)target;
    return false;
#endif
}

void m5u_log_set_suffix(int target, const char* suffix) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Log.setSuffix(m5u_log_target_from_int(target), suffix);
#else
    (void)target; (void)suffix;
#endif
}

void m5u_set_log_display_index(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (index >= 0) {
        M5.setLogDisplayIndex((size_t)index);
    }
#else
    (void)index;
#endif
}

bool m5u_cardputer_begin(bool enable_keyboard) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.begin(m5u_config_from_c(nullptr), enable_keyboard);
    return true;
#else
    (void)enable_keyboard;
    return false;
#endif
}

bool m5u_cardputer_begin_with_config(const m5u_config_t* config, bool enable_keyboard) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.begin(m5u_config_from_c(config), enable_keyboard);
    return true;
#else
    (void)config;
    (void)enable_keyboard;
    return false;
#endif
}

void m5u_cardputer_update(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.update();
#endif
}

void m5u_cardputer_keyboard_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.Keyboard.begin();
#endif
}

bool m5u_cardputer_keyboard_is_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return M5Cardputer.Keyboard.isPressed() != 0;
#else
    return false;
#endif
}

uint8_t m5u_cardputer_keyboard_pressed_count(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return M5Cardputer.Keyboard.isPressed();
#else
    return 0;
#endif
}

bool m5u_cardputer_keyboard_is_change(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return M5Cardputer.Keyboard.isChange();
#else
    return false;
#endif
}

bool m5u_cardputer_keyboard_is_key_pressed(uint8_t key) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return M5Cardputer.Keyboard.isKeyPressed((char)key);
#else
    (void)key;
    return false;
#endif
}

uint8_t m5u_cardputer_keyboard_get_key(uint8_t x, uint8_t y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    Point2D_t key_coor;
    key_coor.x = x;
    key_coor.y = y;
    return M5Cardputer.Keyboard.getKey(key_coor);
#else
    (void)x; (void)y; return 0;
#endif
}

bool m5u_cardputer_keyboard_get_key_value(uint8_t x, uint8_t y, m5u_cardputer_key_value_t* out) {
    if (!out) { return false; }
    memset(out, 0, sizeof(*out));
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    if (x >= 14 || y >= 4) { return false; }
    Point2D_t key_coor;
    key_coor.x = x;
    key_coor.y = y;
    auto value = M5Cardputer.Keyboard.getKeyValue(key_coor);
    out->first = static_cast<uint8_t>(value.value_first);
    out->second = static_cast<uint8_t>(value.value_second);
    return true;
#else
    (void)x; (void)y; return false;
#endif
}

bool m5u_cardputer_keyboard_get_state(m5u_cardputer_keyboard_state_t* out) {
    if (!out) { return false; }
    memset(out, 0, sizeof(*out));

#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.Keyboard.updateKeysState();
    auto& state = M5Cardputer.Keyboard.keysState();
    out->tab = state.tab;
    out->fn_key = state.fn;
    out->shift = state.shift;
    out->ctrl = state.ctrl;
    out->opt = state.opt;
    out->alt = state.alt;
    out->del = state.del;
    out->enter = state.enter;
    out->space = state.space;
    out->modifiers = state.modifiers;

    out->word_len = state.word.size();
    if (out->word_len > M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY) {
        out->word_len = M5U_CARDPUTER_KEYBOARD_WORD_CAPACITY;
    }
    for (size_t i = 0; i < out->word_len; ++i) {
        out->word[i] = (uint8_t)state.word[i];
    }

    out->hid_len = state.hid_keys.size();
    if (out->hid_len > M5U_CARDPUTER_KEYBOARD_HID_CAPACITY) {
        out->hid_len = M5U_CARDPUTER_KEYBOARD_HID_CAPACITY;
    }
    for (size_t i = 0; i < out->hid_len; ++i) {
        out->hid_keys[i] = (uint8_t)state.hid_keys[i];
    }

    out->modifier_len = state.modifier_keys.size();
    if (out->modifier_len > M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY) {
        out->modifier_len = M5U_CARDPUTER_KEYBOARD_MODIFIER_CAPACITY;
    }
    for (size_t i = 0; i < out->modifier_len; ++i) {
        out->modifier_keys[i] = (uint8_t)state.modifier_keys[i];
    }
#endif

    return true;
}

bool m5u_cardputer_keyboard_capslocked(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    return M5Cardputer.Keyboard.capslocked();
#else
    return false;
#endif
}

void m5u_cardputer_keyboard_set_capslocked(bool locked) {
#ifdef M5UNIFIED_RS_USE_REAL_M5CARDPUTER
    M5Cardputer.Keyboard.setCapsLocked(locked);
#else
    (void)locked;
#endif
}

bool m5u_cardputer_sd_begin(int sck, int miso, int mosi, int cs, uint32_t frequency_hz) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    SPI.begin(sck, miso, mosi, cs);
    return SD.begin(cs, SPI, frequency_hz);
#else
    (void)sck; (void)miso; (void)mosi; (void)cs; (void)frequency_hz;
    return false;
#endif
}

void m5u_cardputer_sd_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    SD.end();
#endif
}

int m5u_cardputer_sd_card_type(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    switch (SD.cardType()) {
    case CARD_NONE: return 0;
    case CARD_MMC: return 1;
    case CARD_SD: return 2;
    case CARD_SDHC: return 3;
    default: return 4;
    }
#else
    return 0;
#endif
}

uint64_t m5u_cardputer_sd_card_size_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.cardSize();
#else
    return 0;
#endif
}

uint64_t m5u_cardputer_sd_total_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.totalBytes();
#else
    return 0;
#endif
}

uint64_t m5u_cardputer_sd_used_bytes(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    return SD.usedBytes();
#else
    return 0;
#endif
}

bool m5u_cardputer_sd_exists(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.exists(path);
#else
    (void)path;
    return false;
#endif
}

uint64_t m5u_cardputer_sd_file_size(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return 0;
    }

    File file = SD.open(path, FILE_READ);
    if (!file || file.isDirectory()) {
        return 0;
    }

    return file.size();
#else
    (void)path;
    return 0;
#endif
}

bool m5u_cardputer_sd_is_directory(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }

    File file = SD.open(path, FILE_READ);
    if (!file) {
        return false;
    }

    return file.isDirectory();
#else
    (void)path;
    return false;
#endif
}

size_t m5u_cardputer_sd_list_dir(const char* path, m5u_cardputer_sd_dir_entry_t* entries, size_t capacity) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || entries == nullptr || capacity == 0) {
        return 0;
    }

    File root = SD.open(path, FILE_READ);
    if (!root || !root.isDirectory()) {
        return 0;
    }

    size_t count = 0;
    while (count < capacity) {
        File file = root.openNextFile();
        if (!file) {
            break;
        }

        memset(&entries[count], 0, sizeof(entries[count]));
        const char* name = file.name();
        if (name != nullptr) {
            strncpy(entries[count].name, name, M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY - 1);
            entries[count].name[M5U_CARDPUTER_SD_DIR_ENTRY_NAME_CAPACITY - 1] = '\0';
        }
        entries[count].is_directory = file.isDirectory();
        entries[count].size = entries[count].is_directory ? 0 : file.size();
        ++count;
    }

    return count;
#else
    (void)path; (void)entries; (void)capacity;
    return 0;
#endif
}

size_t m5u_cardputer_sd_read_file(const char* path, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || data == nullptr || len == 0) {
        return 0;
    }

    File file = SD.open(path, FILE_READ);
    if (!file) {
        return 0;
    }

    return file.read(data, len);
#else
    (void)path; (void)data; (void)len;
    return 0;
#endif
}

size_t m5u_cardputer_sd_write_file(const char* path, const uint8_t* data, size_t len, bool append) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr || data == nullptr || len == 0) {
        return 0;
    }

    File file = SD.open(path, append ? FILE_APPEND : FILE_WRITE);
    if (!file) {
        return 0;
    }

    return file.write(data, len);
#else
    (void)path; (void)data; (void)len; (void)append;
    return 0;
#endif
}

bool m5u_cardputer_sd_remove(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.remove(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_cardputer_sd_mkdir(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.mkdir(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_cardputer_sd_rmdir(const char* path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (path == nullptr) {
        return false;
    }
    return SD.rmdir(path);
#else
    (void)path;
    return false;
#endif
}

bool m5u_cardputer_sd_rename(const char* from_path, const char* to_path) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SD
    if (from_path == nullptr || to_path == nullptr) {
        return false;
    }
    return SD.rename(from_path, to_path);
#else
    (void)from_path; (void)to_path;
    return false;
#endif
}

bool m5u_cardputer_ir_begin(int pin) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_IRREMOTE
    IrSender.begin(DISABLE_LED_FEEDBACK);
    IrSender.setSendPin(pin);
    return true;
#else
    (void)pin;
    return false;
#endif
}

bool m5u_cardputer_ir_send_nec(uint16_t address, uint8_t command, uint8_t repeats) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_IRREMOTE
    IrSender.sendNEC(address, command, repeats);
    return true;
#else
    (void)address; (void)command; (void)repeats;
    return false;
#endif
}

bool m5u_cardputer_grove_i2c_begin(int sda, int scl, uint32_t frequency_hz) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    return Wire.begin(sda, scl, frequency_hz);
#else
    (void)sda; (void)scl; (void)frequency_hz;
    return false;
#endif
}

void m5u_cardputer_grove_i2c_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.end();
#endif
}

bool m5u_cardputer_grove_i2c_probe(uint8_t address) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    return Wire.endTransmission() == 0;
#else
    (void)address;
    return false;
#endif
}

bool m5u_cardputer_grove_i2c_write(uint8_t address, const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    if (data && len) {
        Wire.write(data, len);
    }
    return Wire.endTransmission() == 0;
#else
    (void)address; (void)data; (void)len;
    return false;
#endif
}

size_t m5u_cardputer_grove_i2c_read(uint8_t address, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    if (!data || !len) { return 0; }
    size_t requested = Wire.requestFrom((int)address, (int)len);
    size_t read_len = 0;
    while (Wire.available() && read_len < requested && read_len < len) {
        data[read_len++] = (uint8_t)Wire.read();
    }
    return read_len;
#else
    (void)address; (void)data; (void)len;
    return 0;
#endif
}

bool m5u_cardputer_grove_i2c_write_reg(uint8_t address, uint8_t reg, const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    Wire.beginTransmission(address);
    Wire.write(reg);
    if (data && len) {
        Wire.write(data, len);
    }
    return Wire.endTransmission() == 0;
#else
    (void)address; (void)reg; (void)data; (void)len;
    return false;
#endif
}

size_t m5u_cardputer_grove_i2c_read_reg(uint8_t address, uint8_t reg, uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_WIRE
    if (!data || !len) { return 0; }
    Wire.beginTransmission(address);
    Wire.write(reg);
    if (Wire.endTransmission(false) != 0) { return 0; }

    size_t requested = Wire.requestFrom((int)address, (int)len);
    size_t read_len = 0;
    while (Wire.available() && read_len < requested && read_len < len) {
        data[read_len++] = (uint8_t)Wire.read();
    }
    return read_len;
#else
    (void)address; (void)reg; (void)data; (void)len;
    return 0;
#endif
}

bool m5u_cardputer_grove_gpio_pin_mode(int pin, int mode) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    switch (mode) {
    case 0: pinMode(pin, INPUT); return true;
    case 1: pinMode(pin, OUTPUT); return true;
    case 2: pinMode(pin, INPUT_PULLUP); return true;
    case 3: pinMode(pin, INPUT_PULLDOWN); return true;
    default: return false;
    }
#else
    (void)pin; (void)mode;
    return false;
#endif
}

bool m5u_cardputer_grove_gpio_write(int pin, bool high) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    digitalWrite(pin, high ? HIGH : LOW);
    return true;
#else
    (void)pin; (void)high;
    return false;
#endif
}

int m5u_cardputer_grove_gpio_read(int pin) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_GPIO
    return digitalRead(pin) == HIGH ? 1 : 0;
#else
    (void)pin;
    return -1;
#endif
}

bool m5u_cardputer_grove_uart_begin(int rx, int tx, uint32_t baud) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial1.begin(baud, SERIAL_8N1, rx, tx);
    return true;
#else
    (void)rx; (void)tx; (void)baud;
    return false;
#endif
}

void m5u_cardputer_grove_uart_end(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial1.end();
#endif
}

size_t m5u_cardputer_grove_uart_available(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    int available = Serial1.available();
    return available > 0 ? (size_t)available : 0;
#else
    return 0;
#endif
}

size_t m5u_cardputer_grove_uart_read(uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    if (!data || !len) { return 0; }
    size_t read_len = 0;
    while (read_len < len && Serial1.available() > 0) {
        int value = Serial1.read();
        if (value < 0) { break; }
        data[read_len++] = (uint8_t)value;
    }
    return read_len;
#else
    (void)data; (void)len;
    return 0;
#endif
}

size_t m5u_cardputer_grove_uart_write(const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    if (!data || !len) { return 0; }
    return Serial1.write(data, len);
#else
    (void)data; (void)len;
    return 0;
#endif
}

void m5u_cardputer_grove_uart_flush(void) {
#ifdef M5UNIFIED_RS_USE_ARDUINO_SERIAL
    Serial1.flush();
#endif
}

} // extern "C"
