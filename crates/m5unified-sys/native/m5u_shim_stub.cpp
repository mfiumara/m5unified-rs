#include "m5u_shim.h"

// Optional no-op implementation for host-side C ABI checks. Firmware builds
// should compile m5u_shim.cpp, which calls the real M5Unified C++ API.

extern "C" {

bool m5u_begin(void) {
    return false;
}

bool m5u_begin_with_config(const m5u_config_t* config) {
    (void)config; return false;
}

void m5u_update(void) {
}

void m5u_delay_ms(uint32_t ms) {
    (void)ms;
}

int m5u_get_board(void) {
    return 0;
}

int m5u_get_pin(int name) {
    (void)name; return -1;
}

bool m5u_set_primary_display_index(size_t index) {
    return index == 0;
}

bool m5u_set_primary_display_type(int kind) {
    (void)kind; return false;
}

void m5u_set_log_display_index(size_t index) {
    (void)index;
}

void m5u_set_log_display_type(int kind) {
    (void)kind;
}

void m5u_set_touch_button_height(uint16_t pixel) {
    (void)pixel;
}

void m5u_set_touch_button_height_by_ratio(uint8_t ratio) {
    (void)ratio;
}

uint16_t m5u_get_touch_button_height(void) {
    return 0;
}

int m5u_display_width(void) {
    return 0;
}

int m5u_display_height(void) {
    return 0;
}

void m5u_display_fill_screen(uint16_t color) {
    (void)color;
}

void m5u_display_set_cursor(int x, int y) {
    (void)x; (void)y;
}

void m5u_display_set_text_size(int size) {
    (void)size;
}

void m5u_display_set_text_color(uint16_t fg, uint16_t bg) {
    (void)fg; (void)bg;
}

void m5u_display_print(const char* text) {
    (void)text;
}

void m5u_display_println(const char* text) {
    (void)text;
}

void m5u_display_draw_line(int x0, int y0, int x1, int y1, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
}

void m5u_display_draw_rect(int x, int y, int w, int h, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_fill_rect(int x, int y, int w, int h, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_draw_circle(int x, int y, int r, uint16_t color) {
    (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_fill_circle(int x, int y, int r, uint16_t color) {
    (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_set_rotation(int rotation) {
    (void)rotation;
}

bool m5u_btn_a_is_pressed(void) {
    return false;
}

bool m5u_btn_a_was_pressed(void) {
    return false;
}

bool m5u_btn_a_was_released(void) {
    return false;
}

bool m5u_btn_b_is_pressed(void) {
    return false;
}

bool m5u_btn_b_was_pressed(void) {
    return false;
}

bool m5u_btn_b_was_released(void) {
    return false;
}

bool m5u_btn_c_is_pressed(void) {
    return false;
}

bool m5u_btn_c_was_pressed(void) {
    return false;
}

bool m5u_btn_c_was_released(void) {
    return false;
}

bool m5u_mic_begin(void) {
    return false;
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    (void)buffer; (void)samples; return false;
}

bool m5u_speaker_begin(void) {
    return false;
}

void m5u_speaker_set_volume(uint8_t volume) {
    (void)volume;
}

bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms) {
    (void)frequency_hz; (void)duration_ms; return false;
}

bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz) {
    (void)samples; (void)len; (void)sample_rate_hz; return false;
}

bool m5u_imu_begin(void) {
    return false;
}

bool m5u_imu_get_accel(float* x, float* y, float* z) {
    (void)x; (void)y; (void)z; return false;
}

bool m5u_imu_get_gyro(float* x, float* y, float* z) {
    (void)x; (void)y; (void)z; return false;
}

bool m5u_imu_get_temp_c(float* temp) {
    (void)temp; return false;
}

int m5u_touch_count(void) {
    return 0;
}

bool m5u_touch_get(int index, int* x, int* y) {
    (void)index; (void)x; (void)y; return false;
}

bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second) {
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
}

bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second) {
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
}

void m5u_rtc_set_system_time_from_rtc(void) {
}

int m5u_battery_level(void) {
    return -1;
}

int m5u_battery_voltage_mv(void) {
    return -1;
}

bool m5u_power_is_charging(void) {
    return false;
}

void m5u_log_println(const char* text) {
    (void)text;
}

bool m5u_sd_begin(void) {
    // SD support needs an explicit ESP-IDF/Arduino SD component wiring step.
    // Keep the shim target-buildable for display/button firmware until that
    // component is added instead of referencing Arduino globals (SD, SPI) here.
    return false;
}


static bool m5u_button_state(int button, int query) {
    (void)button; (void)query; return false;
}

int m5u_display_get_rotation(void) {
    return 0;
}

void m5u_display_set_brightness(uint8_t brightness) {
    (void)brightness;
}

void m5u_display_set_epd_fastest(void) {
}

void m5u_display_set_epd_mode(int mode) {
    (void)mode;
}

void m5u_display_set_text_scroll(bool scroll) {
    (void)scroll;
}

bool m5u_display_set_font(int font) {
    return font >= 0 && font <= 3;
}

void m5u_display_start_write(void) {
}

void m5u_display_end_write(void) {
}

void m5u_display_display(void) {
}

bool m5u_display_display_busy(void) {
    return false;
}

void m5u_display_wait_display(void) {
}

int m5u_display_get_cursor_y(void) {
    return 0;
}

int m5u_display_font_height(void) {
    return 16;
}

uint16_t m5u_display_get_base_color(void) {
    return 0;
}

void m5u_display_set_color(uint16_t color) {
    (void)color;
}

void m5u_display_set_text_wrap(bool wrap_x, bool wrap_y) {
    (void)wrap_x; (void)wrap_y;
}

void m5u_display_set_text_datum(int datum) {
    (void)datum;
}

int m5u_display_draw_string(const char* text, int x, int y) {
    (void)text; (void)x; (void)y; return 0;
}

void m5u_display_write_pixel(int x, int y, uint16_t color) {
    (void)x; (void)y; (void)color;
}

void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color) {
    (void)x; (void)y; (void)h; (void)color;
}

void m5u_display_set_clip_rect(int x, int y, int w, int h) {
    (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_clear_clip_rect(void) {
}

uint16_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) {
    return (uint16_t)((r & 0xF8) << 8 | (g & 0xFC) << 3 | (b >> 3));
}

int m5u_display_count(void) {
    return 1;
}

int m5u_display_index_for_kind(int kind) {
    (void)kind; return -1;
}

int m5u_display_width_at(int index) {
    (void)index; return 320;
}

int m5u_display_height_at(int index) {
    (void)index; return 240;
}

void m5u_display_print_at(int index, const char* text) {
    (void)index; (void)text;
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r; (void)color;
}

bool m5u_button_is_pressed(int button) { return m5u_button_state(button, 0); }
bool m5u_button_was_pressed(int button) { return m5u_button_state(button, 1); }
bool m5u_button_was_released(int button) { return m5u_button_state(button, 2); }
bool m5u_button_was_clicked(int button) { return m5u_button_state(button, 3); }
bool m5u_button_was_hold(int button) { return m5u_button_state(button, 4); }
bool m5u_button_is_holding(int button) { return m5u_button_state(button, 5); }
bool m5u_button_was_decide_click_count(int button) { return m5u_button_state(button, 6); }
int m5u_button_get_click_count(int button) {
    (void)button; return 0;
}

bool m5u_mic_is_enabled(void) {
    return false;
}

bool m5u_mic_is_recording(void) {
    return false;
}

void m5u_mic_end(void) {
}

bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz) {
    (void)buffer; (void)samples; (void)sample_rate_hz; return false;
}

int m5u_mic_get_noise_filter_level(void) {
    return 0;
}

bool m5u_mic_set_noise_filter_level(int level) {
    (void)level; return false;
}

bool m5u_speaker_is_enabled(void) {
    return false;
}

void m5u_speaker_end(void) {
}

uint8_t m5u_speaker_get_volume(void) {
    return 0;
}

bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel) {
    (void)frequency_hz; (void)duration_ms; (void)channel; return false;
}

bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz) {
    (void)samples; (void)len; (void)sample_rate_hz; return false;
}

bool m5u_speaker_play_wav(const uint8_t* data, size_t len) {
    (void)data; (void)len; return false;
}

bool m5u_speaker_is_playing(int channel) {
    (void)channel; return false;
}

void m5u_speaker_stop(int channel) {
    (void)channel;
}

uint8_t m5u_speaker_get_channel_volume(int channel) {
    (void)channel; return 0;
}

void m5u_speaker_set_channel_volume(int channel, uint8_t volume) {
    (void)channel; (void)volume;
}

void m5u_speaker_set_all_channel_volume(uint8_t volume) {
    (void)volume;
}

bool m5u_imu_is_enabled(void) {
    return false;
}

int m5u_imu_get_type(void) {
    return 0;
}

bool m5u_imu_update(void) {
    return false;
}

bool m5u_imu_load_offset_from_nvs(void) {
    return false;
}

bool m5u_imu_save_offset_to_nvs(void) {
    return false;
}

float m5u_imu_get_offset_data(int index) {
    (void)index; return 0.0f;
}

void m5u_imu_set_calibration(float x, float y, float z) {
    (void)x; (void)y; (void)z;
}

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
    (void)index; (void)out; return false;
}

bool m5u_rtc_is_enabled(void) {
    return false;
}

bool m5u_power_axp2101_disable_irq(uint64_t mask) { (void)mask; return false; }
bool m5u_power_axp2101_enable_irq(uint64_t mask) { (void)mask; return false; }
bool m5u_power_axp2101_clear_irq_statuses(void) { return false; }
uint64_t m5u_power_axp2101_get_irq_statuses(void) { return 0; }
bool m5u_power_axp2101_is_bat_charger_under_temperature_irq(void) { return false; }
bool m5u_power_axp2101_is_bat_charger_over_temperature_irq(void) { return false; }
bool m5u_power_axp2101_is_vbus_insert_irq(void) { return false; }
bool m5u_power_axp2101_is_vbus_remove_irq(void) { return false; }

bool m5u_led_begin(void) {
    return false;
}

void m5u_led_display(void) {
}

void m5u_led_set_auto_display(bool enable) {
    (void)enable;
}

size_t m5u_led_count(void) {
    return 0;
}

void m5u_led_set_brightness(uint8_t brightness) {
    (void)brightness;
}

void m5u_led_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
    (void)index; (void)r; (void)g; (void)b;
}

void m5u_led_set_all_color_rgb(uint8_t r, uint8_t g, uint8_t b) {
    (void)r; (void)g; (void)b;
}

bool m5u_led_is_enabled(void) {
    return false;
}

void m5u_log_print(const char* text) {
    (void)text;
}

void m5u_log_level(int level, const char* text) {
    (void)level; (void)text;
}

bool m5u_log_set_enable_color(int target, bool enable) {
    (void)enable; return target >= 0 && target <= 2;
}

bool m5u_log_get_enable_color(int target) {
    return target >= 0 && target <= 2;
}

bool m5u_log_set_level(int target, int level) {
    return target >= 0 && target <= 2 && level >= 0 && level <= 5;
}

int m5u_log_get_level(int target) {
    return target >= 0 && target <= 2 ? 3 : -1;
}

bool m5u_log_set_suffix(int target, const char* suffix) {
    return target >= 0 && target <= 2 && suffix != nullptr;
}

} // extern "C"
