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

bool m5u_btn_a_is_pressed(void);
bool m5u_btn_a_was_pressed(void);
bool m5u_btn_a_was_released(void);
bool m5u_btn_b_is_pressed(void);
bool m5u_btn_b_was_pressed(void);
bool m5u_btn_b_was_released(void);
bool m5u_btn_c_is_pressed(void);
bool m5u_btn_c_was_pressed(void);
bool m5u_btn_c_was_released(void);

bool m5u_mic_begin(void);
bool m5u_mic_record_i16(int16_t* buffer, size_t samples);
bool m5u_speaker_begin(void);
void m5u_speaker_set_volume(uint8_t volume);
bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms);
bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz);

bool m5u_imu_begin(void);
bool m5u_imu_get_accel(float* x, float* y, float* z);
bool m5u_imu_get_gyro(float* x, float* y, float* z);
bool m5u_imu_get_temp_c(float* temp);

int m5u_touch_count(void);
bool m5u_touch_get(int index, int* x, int* y);

bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second);
bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second);

int m5u_battery_level(void);
int m5u_battery_voltage_mv(void);
bool m5u_power_is_charging(void);

void m5u_log_println(const char* text);
bool m5u_sd_begin(void);

#ifdef __cplusplus
}
#endif
