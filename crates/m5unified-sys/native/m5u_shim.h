#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

bool m5u_begin(void);
void m5u_update(void);
void m5u_delay_ms(uint32_t ms);

int m5u_display_width(void);
int m5u_display_height(void);
void m5u_display_fill_screen(uint16_t color);
void m5u_display_set_cursor(int x, int y);
void m5u_display_set_text_size(int size);
void m5u_display_set_text_color(uint16_t fg, uint16_t bg);
void m5u_display_print(const char* text);
void m5u_display_println(const char* text);
void m5u_display_draw_line(int x0, int y0, int x1, int y1, uint16_t color);
void m5u_display_draw_rect(int x, int y, int w, int h, uint16_t color);
void m5u_display_fill_rect(int x, int y, int w, int h, uint16_t color);
void m5u_display_draw_circle(int x, int y, int r, uint16_t color);
void m5u_display_fill_circle(int x, int y, int r, uint16_t color);
void m5u_display_set_rotation(int rotation);
int m5u_display_get_rotation(void);
void m5u_display_set_brightness(uint8_t brightness);
void m5u_display_set_epd_fastest(void);
void m5u_display_start_write(void);
void m5u_display_end_write(void);
void m5u_display_display(void);
bool m5u_display_display_busy(void);
void m5u_display_wait_display(void);
int m5u_display_get_cursor_y(void);
int m5u_display_font_height(void);
uint16_t m5u_display_get_base_color(void);
void m5u_display_set_color(uint16_t color);
void m5u_display_set_text_wrap(bool wrap_x, bool wrap_y);
void m5u_display_set_text_datum(int datum);
int m5u_display_draw_string(const char* text, int x, int y);
void m5u_display_write_pixel(int x, int y, uint16_t color);
void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color);
void m5u_display_set_clip_rect(int x, int y, int w, int h);
void m5u_display_clear_clip_rect(void);
uint16_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b);
int m5u_display_count(void);
int m5u_display_index_for_kind(int kind);
int m5u_display_width_at(int index);
int m5u_display_height_at(int index);
void m5u_display_print_at(int index, const char* text);
void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color);

bool m5u_btn_a_is_pressed(void);
bool m5u_btn_a_was_pressed(void);
bool m5u_btn_a_was_released(void);
bool m5u_btn_b_is_pressed(void);
bool m5u_btn_b_was_pressed(void);
bool m5u_btn_b_was_released(void);
bool m5u_btn_c_is_pressed(void);
bool m5u_btn_c_was_pressed(void);
bool m5u_btn_c_was_released(void);
bool m5u_button_is_pressed(int button);
bool m5u_button_was_pressed(int button);
bool m5u_button_was_released(int button);
bool m5u_button_was_clicked(int button);
bool m5u_button_was_hold(int button);
bool m5u_button_is_holding(int button);
bool m5u_button_was_decide_click_count(int button);
int m5u_button_get_click_count(int button);

bool m5u_mic_begin(void);
bool m5u_mic_is_enabled(void);
bool m5u_mic_is_recording(void);
void m5u_mic_end(void);
bool m5u_mic_record_i16(int16_t* buffer, size_t samples);
bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz);
int m5u_mic_get_noise_filter_level(void);
bool m5u_mic_set_noise_filter_level(int level);
bool m5u_speaker_begin(void);
bool m5u_speaker_is_enabled(void);
void m5u_speaker_end(void);
void m5u_speaker_set_volume(uint8_t volume);
uint8_t m5u_speaker_get_volume(void);
bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms);
bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel);
bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz);
bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz);
bool m5u_speaker_play_wav(const uint8_t* data, size_t len);
bool m5u_speaker_is_playing(int channel);
void m5u_speaker_stop(int channel);
uint8_t m5u_speaker_get_channel_volume(int channel);
void m5u_speaker_set_channel_volume(int channel, uint8_t volume);
void m5u_speaker_set_all_channel_volume(uint8_t volume);

bool m5u_imu_begin(void);
bool m5u_imu_is_enabled(void);
int m5u_imu_get_type(void);
bool m5u_imu_update(void);
bool m5u_imu_get_accel(float* x, float* y, float* z);
bool m5u_imu_get_gyro(float* x, float* y, float* z);
bool m5u_imu_get_temp_c(float* temp);
bool m5u_imu_load_offset_from_nvs(void);
bool m5u_imu_save_offset_to_nvs(void);
float m5u_imu_get_offset_data(int index);
void m5u_imu_set_calibration(float x, float y, float z);

typedef struct {
    int x;
    int y;
    int prev_x;
    int prev_y;
    bool is_pressed;
    bool was_pressed;
    bool was_released;
    bool was_clicked;
    bool was_hold;
    bool is_holding;
    int click_count;
} m5u_touch_detail_t;

int m5u_touch_count(void);
bool m5u_touch_get(int index, int* x, int* y);
bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out);

bool m5u_rtc_is_enabled(void);
bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second);
bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second);

int m5u_battery_level(void);
int m5u_battery_voltage_mv(void);
bool m5u_power_is_charging(void);
bool m5u_power_axp2101_disable_irq(uint64_t mask);
bool m5u_power_axp2101_enable_irq(uint64_t mask);
bool m5u_power_axp2101_clear_irq_statuses(void);
uint64_t m5u_power_axp2101_get_irq_statuses(void);
bool m5u_power_axp2101_is_bat_charger_under_temperature_irq(void);
bool m5u_power_axp2101_is_bat_charger_over_temperature_irq(void);
bool m5u_power_axp2101_is_vbus_insert_irq(void);
bool m5u_power_axp2101_is_vbus_remove_irq(void);

bool m5u_led_begin(void);
void m5u_led_display(void);
void m5u_led_set_auto_display(bool enable);
size_t m5u_led_count(void);
void m5u_led_set_brightness(uint8_t brightness);
void m5u_led_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b);
void m5u_led_set_all_color_rgb(uint8_t r, uint8_t g, uint8_t b);
bool m5u_led_is_enabled(void);

void m5u_log_print(const char* text);
void m5u_log_println(const char* text);
void m5u_log_level(int level, const char* text);
bool m5u_sd_begin(void);

#ifdef __cplusplus
}
#endif
