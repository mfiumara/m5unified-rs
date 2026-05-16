#include "m5u_shim.h"

// The real implementation is intentionally guarded until the ESP-IDF component
// integration is wired in build.rs. This file documents the exact C ABI the Rust
// side expects. Once M5Unified/M5GFX are available as ESP-IDF components, define
// M5UNIFIED_RS_USE_REAL_M5UNIFIED and compile this shim as C++.
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
#include <M5Unified.h>
#endif

extern "C" {

bool m5u_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    auto cfg = M5.config();
    M5.begin(cfg);
    return true;
#else
    return false;
#endif
}

void m5u_update(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.update();
#endif
}

void m5u_delay_ms(uint32_t ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.delay(ms);
#else
    (void)ms;
#endif
}

int m5u_display_width(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.width();
#else
    return 0;
#endif
}

int m5u_display_height(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.height();
#else
    return 0;
#endif
}

void m5u_display_fill_screen(uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillScreen(color);
#else
    (void)color;
#endif
}

void m5u_display_set_cursor(int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setCursor(x, y);
#else
    (void)x; (void)y;
#endif
}

void m5u_display_set_text_size(int size) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextSize(size);
#else
    (void)size;
#endif
}

void m5u_display_set_text_color(uint16_t fg, uint16_t bg) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextColor(fg, bg);
#else
    (void)fg; (void)bg;
#endif
}

void m5u_display_print(const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.print(text);
#else
    (void)text;
#endif
}

void m5u_display_println(const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.println(text);
#else
    (void)text;
#endif
}

void m5u_display_draw_line(int x0, int y0, int x1, int y1, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawLine(x0, y0, x1, y1, color);
#else
    (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
#endif
}

void m5u_display_draw_rect(int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawRect(x, y, w, h, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_fill_rect(int x, int y, int w, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillRect(x, y, w, h, color);
#else
    (void)x; (void)y; (void)w; (void)h; (void)color;
#endif
}

void m5u_display_draw_circle(int x, int y, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.drawCircle(x, y, r, color);
#else
    (void)x; (void)y; (void)r; (void)color;
#endif
}

void m5u_display_fill_circle(int x, int y, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.fillCircle(x, y, r, color);
#else
    (void)x; (void)y; (void)r; (void)color;
#endif
}

void m5u_display_set_rotation(int rotation) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setRotation(rotation);
#else
    (void)rotation;
#endif
}

bool m5u_btn_a_is_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnA.isPressed();
#else
    return false;
#endif
}

bool m5u_btn_a_was_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnA.wasPressed();
#else
    return false;
#endif
}

bool m5u_btn_a_was_released(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnA.wasReleased();
#else
    return false;
#endif
}

bool m5u_btn_b_is_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnB.isPressed();
#else
    return false;
#endif
}

bool m5u_btn_b_was_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnB.wasPressed();
#else
    return false;
#endif
}

bool m5u_btn_b_was_released(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnB.wasReleased();
#else
    return false;
#endif
}

bool m5u_btn_c_is_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnC.isPressed();
#else
    return false;
#endif
}

bool m5u_btn_c_was_pressed(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnC.wasPressed();
#else
    return false;
#endif
}

bool m5u_btn_c_was_released(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.BtnC.wasReleased();
#else
    return false;
#endif
}

bool m5u_mic_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.begin();
#else
    return false;
#endif
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.record(buffer, samples);
#else
    (void)buffer; (void)samples; return false;
#endif
}

bool m5u_speaker_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.begin();
#else
    return false;
#endif
}

void m5u_speaker_set_volume(uint8_t volume) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Speaker.setVolume(volume);
#else
    (void)volume;
#endif
}

bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.tone(frequency_hz, duration_ms);
#else
    (void)frequency_hz; (void)duration_ms; return false;
#endif
}

bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, false, 1, 0);
#else
    (void)samples; (void)len; (void)sample_rate_hz; return false;
#endif
}

bool m5u_imu_begin(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.begin();
#else
    return false;
#endif
}

bool m5u_imu_get_accel(float* x, float* y, float* z) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getAccel(x, y, z);
#else
    (void)x; (void)y; (void)z; return false;
#endif
}

bool m5u_imu_get_gyro(float* x, float* y, float* z) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getGyro(x, y, z);
#else
    (void)x; (void)y; (void)z; return false;
#endif
}

bool m5u_imu_get_temp_c(float* temp) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getTemp(temp);
#else
    (void)temp; return false;
#endif
}

int m5u_touch_count(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Touch.getCount();
#else
    return 0;
#endif
}

bool m5u_touch_get(int index, int* x, int* y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    auto detail = M5.Touch.getDetail(index);
    if (x) { *x = detail.x; }
    if (y) { *y = detail.y; }
    return detail.isPressed();
#else
    (void)index; (void)x; (void)y; return false;
#endif
}

bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_datetime_t dt;
    if (!M5.Rtc.getDateTime(&dt)) { return false; }
    if (year) { *year = dt.date.year; }
    if (month) { *month = dt.date.month; }
    if (day) { *day = dt.date.date; }
    if (hour) { *hour = dt.time.hours; }
    if (minute) { *minute = dt.time.minutes; }
    if (second) { *second = dt.time.seconds; }
    return true;
#else
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
#endif
}

bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::rtc_datetime_t dt;
    dt.date.year = year;
    dt.date.month = month;
    dt.date.date = day;
    dt.time.hours = hour;
    dt.time.minutes = minute;
    dt.time.seconds = second;
    M5.Rtc.setDateTime(&dt);
    return true;
#else
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
#endif
}

int m5u_battery_level(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getBatteryLevel();
#else
    return -1;
#endif
}

int m5u_battery_voltage_mv(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.getBatteryVoltage();
#else
    return -1;
#endif
}

bool m5u_power_is_charging(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Power.isCharging();
#else
    return false;
#endif
}

void m5u_log_println(const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5_LOGI("%s", text);
#else
    (void)text;
#endif
}

bool m5u_sd_begin(void) {
    // SD support needs an explicit ESP-IDF/Arduino SD component wiring step.
    // Keep the shim target-buildable for display/button firmware until that
    // component is added instead of referencing Arduino globals (SD, SPI) here.
    return false;
}

} // extern "C"
