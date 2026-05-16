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


static bool m5u_button_state(int button, int query) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    m5::Button_Class* btn = nullptr;
    switch (button) {
    case 0: btn = &M5.BtnA; break;
    case 1: btn = &M5.BtnB; break;
    case 2: btn = &M5.BtnC; break;
    case 3: btn = &M5.BtnPWR; break;
    case 4: btn = &M5.BtnEXT; break;
    default: return false;
    }
    switch (query) {
    case 0: return btn->isPressed();
    case 1: return btn->wasPressed();
    case 2: return btn->wasReleased();
    case 3: return btn->wasClicked();
    case 4: return btn->wasHold();
    case 5: return btn->isHolding();
    case 6: return btn->wasDecideClickCount();
    default: return false;
    }
#else
    (void)button; (void)query; return false;
#endif
}

int m5u_display_get_rotation(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getRotation();
#else
    return 0;
#endif
}

void m5u_display_set_brightness(uint8_t brightness) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setBrightness(brightness);
#else
    (void)brightness;
#endif
}

void m5u_display_set_epd_fastest(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setEpdMode(m5gfx::epd_fastest);
#endif
}

void m5u_display_start_write(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.startWrite();
#endif
}

void m5u_display_end_write(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.endWrite();
#endif
}

void m5u_display_display(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.display();
#endif
}

bool m5u_display_display_busy(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.displayBusy();
#else
    return false;
#endif
}

void m5u_display_wait_display(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.waitDisplay();
#endif
}

int m5u_display_get_cursor_y(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getCursorY();
#else
    return 0;
#endif
}

int m5u_display_font_height(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.fontHeight();
#else
    return 16;
#endif
}

uint16_t m5u_display_get_base_color(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.getBaseColor();
#else
    return 0;
#endif
}

void m5u_display_set_color(uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setColor(color);
#else
    (void)color;
#endif
}

void m5u_display_set_text_wrap(bool wrap_x, bool wrap_y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextWrap(wrap_x, wrap_y);
#else
    (void)wrap_x; (void)wrap_y;
#endif
}

void m5u_display_set_text_datum(int datum) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setTextDatum((textdatum_t)datum);
#else
    (void)datum;
#endif
}

int m5u_display_draw_string(const char* text, int x, int y) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.drawString(text, x, y);
#else
    (void)text; (void)x; (void)y; return 0;
#endif
}

void m5u_display_write_pixel(int x, int y, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.writePixel(x, y, color);
#else
    (void)x; (void)y; (void)color;
#endif
}

void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.writeFastVLine(x, y, h, color);
#else
    (void)x; (void)y; (void)h; (void)color;
#endif
}

void m5u_display_set_clip_rect(int x, int y, int w, int h) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.setClipRect(x, y, w, h);
#else
    (void)x; (void)y; (void)w; (void)h;
#endif
}

void m5u_display_clear_clip_rect(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Display.clearClipRect();
#endif
}

uint16_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Display.color888(r, g, b);
#else
    return (uint16_t)((r & 0xF8) << 8 | (g & 0xFC) << 3 | (b >> 3));
#endif
}

int m5u_display_count(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.getDisplayCount();
#else
    return 1;
#endif
}

int m5u_display_index_for_kind(int kind) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.getDisplayIndex((m5::board_t)kind);
#else
    (void)kind; return -1;
#endif
}

int m5u_display_width_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).width();
#else
    (void)index; return 320;
#endif
}

int m5u_display_height_at(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Displays(index).height();
#else
    (void)index; return 240;
#endif
}

void m5u_display_print_at(int index, const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).print(text);
#else
    (void)index; (void)text;
#endif
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Displays(index).fillCircle(x, y, r, color);
#else
    (void)index; (void)x; (void)y; (void)r; (void)color;
#endif
}

bool m5u_button_is_pressed(int button) { return m5u_button_state(button, 0); }
bool m5u_button_was_pressed(int button) { return m5u_button_state(button, 1); }
bool m5u_button_was_released(int button) { return m5u_button_state(button, 2); }
bool m5u_button_was_clicked(int button) { return m5u_button_state(button, 3); }
bool m5u_button_was_hold(int button) { return m5u_button_state(button, 4); }
bool m5u_button_is_holding(int button) { return m5u_button_state(button, 5); }
bool m5u_button_was_decide_click_count(int button) { return m5u_button_state(button, 6); }
int m5u_button_get_click_count(int button) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    switch (button) {
    case 0: return M5.BtnA.getClickCount();
    case 1: return M5.BtnB.getClickCount();
    case 2: return M5.BtnC.getClickCount();
    case 3: return M5.BtnPWR.getClickCount();
    case 4: return M5.BtnEXT.getClickCount();
    default: return 0;
    }
#else
    (void)button; return 0;
#endif
}

bool m5u_mic_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.isEnabled();
#else
    return false;
#endif
}

bool m5u_mic_is_recording(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.isRecording();
#else
    return false;
#endif
}

void m5u_mic_end(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Mic.end();
#endif
}

bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.record(buffer, samples, sample_rate_hz);
#else
    (void)buffer; (void)samples; (void)sample_rate_hz; return false;
#endif
}

int m5u_mic_get_noise_filter_level(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Mic.config().noise_filter_level;
#else
    return 0;
#endif
}

bool m5u_mic_set_noise_filter_level(int level) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    auto cfg = M5.Mic.config();
    cfg.noise_filter_level = level;
    M5.Mic.config(cfg);
    return true;
#else
    (void)level; return false;
#endif
}

bool m5u_speaker_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.isEnabled();
#else
    return false;
#endif
}

void m5u_speaker_end(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Speaker.end();
#endif
}

uint8_t m5u_speaker_get_volume(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.getVolume();
#else
    return 0;
#endif
}

bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.tone(frequency_hz, duration_ms, channel);
#else
    (void)frequency_hz; (void)duration_ms; (void)channel; return false;
#endif
}

bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playRaw(samples, len, sample_rate_hz, false, 1, 0);
#else
    (void)samples; (void)len; (void)sample_rate_hz; return false;
#endif
}

bool m5u_speaker_play_wav(const uint8_t* data, size_t len) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.playWav(data, len);
#else
    (void)data; (void)len; return false;
#endif
}

bool m5u_speaker_is_playing(int channel) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return channel < 0 ? M5.Speaker.isPlaying() : M5.Speaker.isPlaying(channel);
#else
    (void)channel; return false;
#endif
}

void m5u_speaker_stop(int channel) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (channel < 0) { M5.Speaker.stop(); } else { M5.Speaker.stop(channel); }
#else
    (void)channel;
#endif
}

uint8_t m5u_speaker_get_channel_volume(int channel) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Speaker.getChannelVolume(channel);
#else
    (void)channel; return 0;
#endif
}

void m5u_speaker_set_channel_volume(int channel, uint8_t volume) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Speaker.setChannelVolume(channel, volume);
#else
    (void)channel; (void)volume;
#endif
}

void m5u_speaker_set_all_channel_volume(uint8_t volume) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Speaker.setAllChannelVolume(volume);
#else
    (void)volume;
#endif
}

bool m5u_imu_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.isEnabled();
#else
    return false;
#endif
}

int m5u_imu_get_type(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return (int)M5.Imu.getType();
#else
    return 0;
#endif
}

bool m5u_imu_update(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.update();
#else
    return false;
#endif
}

bool m5u_imu_load_offset_from_nvs(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.loadOffsetFromNVS();
#else
    return false;
#endif
}

bool m5u_imu_save_offset_to_nvs(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.saveOffsetToNVS();
#else
    return false;
#endif
}

float m5u_imu_get_offset_data(int index) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Imu.getOffsetData(index);
#else
    (void)index; return 0.0f;
#endif
}

void m5u_imu_set_calibration(float x, float y, float z) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Imu.setCalibration(x, y, z);
#else
    (void)x; (void)y; (void)z;
#endif
}

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    if (!out) { return false; }
    auto d = M5.Touch.getDetail(index);
    out->x = d.x;
    out->y = d.y;
    out->prev_x = d.prev_x;
    out->prev_y = d.prev_y;
    out->is_pressed = d.isPressed();
    out->was_pressed = d.wasPressed();
    out->was_released = d.wasReleased();
    out->was_clicked = d.wasClicked();
    out->was_hold = d.wasHold();
    out->is_holding = d.isHolding();
    out->click_count = d.getClickCount();
    return true;
#else
    (void)index; (void)out; return false;
#endif
}

bool m5u_rtc_is_enabled(void) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    return M5.Rtc.isEnabled();
#else
    return false;
#endif
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
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Log.print(text);
#else
    (void)text;
#endif
}

void m5u_log_level(int level, const char* text) {
#ifdef M5UNIFIED_RS_USE_REAL_M5UNIFIED
    M5.Log((esp_log_level_t)level, "%s", text);
#else
    (void)level; (void)text;
#endif
}

} // extern "C"
