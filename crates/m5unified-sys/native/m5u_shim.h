#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    uint32_t serial_baudrate;
    uint8_t external_speaker_value;
    uint16_t external_display_value;
    uint8_t clear_display;
    uint8_t output_power;
    uint8_t pmic_button;
    uint8_t internal_imu;
    uint8_t internal_rtc;
    uint8_t internal_mic;
    uint8_t internal_spk;
    uint8_t external_imu;
    uint8_t external_rtc;
    uint8_t disable_rtc_irq;
    uint8_t led_brightness;
    int fallback_board;
} m5u_config_t;

bool m5u_begin(void);
bool m5u_begin_with_config(const m5u_config_t* config);
void m5u_update(void);
void m5u_delay_ms(uint32_t ms);
uint32_t m5u_millis(void);
uint32_t m5u_micros(void);
uint32_t m5u_get_update_msec(void);
int m5u_get_board(void);
int m5u_get_pin(int name);
bool m5u_set_primary_display_index(size_t index);
bool m5u_set_primary_display_type(int kind);
void m5u_set_log_display_index(size_t index);
void m5u_set_log_display_type(int kind);
void m5u_set_touch_button_height(uint16_t pixel);
void m5u_set_touch_button_height_by_ratio(uint8_t ratio);
uint16_t m5u_get_touch_button_height(void);

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
void m5u_display_set_epd_mode(int mode);
void m5u_display_set_text_scroll(bool scroll);
bool m5u_display_set_font(int font);
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
void m5u_display_set_text_size_at(int index, int size);
void m5u_display_start_write_at(int index);
void m5u_display_end_write_at(int index);
void m5u_display_print_at(int index, const char* text);
void m5u_display_println_at(int index, const char* text);
int m5u_display_draw_string_at(int index, const char* text, int x, int y);
void m5u_display_fill_rect_at(int index, int x, int y, int w, int h, uint16_t color);
void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color);
void m5u_display_write_pixel_at(int index, int x, int y, uint16_t color);
void m5u_display_draw_pixel_at(int index, int x, int y, uint16_t color);

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
bool m5u_button_was_single_clicked(int button);
bool m5u_button_was_double_clicked(int button);
bool m5u_button_was_change_pressed(int button);
bool m5u_button_is_released(int button);
bool m5u_button_was_released_after_hold(int button);
bool m5u_button_was_release_for(int button, uint32_t ms);
bool m5u_button_pressed_for(int button, uint32_t ms);
bool m5u_button_released_for(int button, uint32_t ms);
void m5u_button_set_debounce_thresh(int button, uint32_t ms);
void m5u_button_set_hold_thresh(int button, uint32_t ms);
uint8_t m5u_button_get_state(int button);
uint32_t m5u_button_last_change(int button);
uint32_t m5u_button_get_debounce_thresh(int button);
uint32_t m5u_button_get_hold_thresh(int button);
uint32_t m5u_button_get_update_msec(int button);

typedef struct {
    int pin_data_in;
    int pin_bck;
    int pin_mck;
    int pin_ws;
    uint32_t sample_rate;
    uint8_t left_channel;
    uint8_t stereo;
    uint8_t over_sampling;
    uint8_t magnification;
    uint8_t noise_filter_level;
    uint8_t use_adc;
    size_t dma_buf_len;
    size_t dma_buf_count;
    uint8_t task_priority;
    uint8_t task_pinned_core;
    int i2s_port;
} m5u_mic_config_t;

typedef struct {
    int pin_data_out;
    int pin_bck;
    int pin_mck;
    int pin_ws;
    uint32_t sample_rate;
    uint8_t stereo;
    uint8_t buzzer;
    uint8_t use_dac;
    uint8_t dac_zero_level;
    uint8_t magnification;
    size_t dma_buf_len;
    size_t dma_buf_count;
    uint8_t task_priority;
    uint8_t task_pinned_core;
    int i2s_port;
} m5u_speaker_config_t;

bool m5u_mic_begin(void);
bool m5u_mic_is_running(void);
bool m5u_mic_is_enabled(void);
bool m5u_mic_is_recording(void);
size_t m5u_mic_recording_state(void);
void m5u_mic_end(void);
bool m5u_mic_record_i16(int16_t* buffer, size_t samples);
bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz);
bool m5u_mic_record_i16_ex(int16_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo);
bool m5u_mic_record_u8(uint8_t* buffer, size_t samples);
bool m5u_mic_record_u8_ex(uint8_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo);
void m5u_mic_set_sample_rate(uint32_t sample_rate_hz);
bool m5u_mic_get_config(m5u_mic_config_t* out);
bool m5u_mic_set_config(const m5u_mic_config_t* config);
int m5u_mic_get_noise_filter_level(void);
bool m5u_mic_set_noise_filter_level(int level);
bool m5u_speaker_begin(void);
bool m5u_speaker_is_running(void);
bool m5u_speaker_is_enabled(void);
void m5u_speaker_end(void);
void m5u_speaker_set_volume(uint8_t volume);
uint8_t m5u_speaker_get_volume(void);
bool m5u_speaker_get_config(m5u_speaker_config_t* out);
bool m5u_speaker_set_config(const m5u_speaker_config_t* config);
bool m5u_speaker_tone(uint32_t frequency_hz, uint32_t duration_ms);
bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel);
bool m5u_speaker_tone_options(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound);
bool m5u_speaker_tone_full(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound, const uint8_t* raw_data, size_t len, bool stereo);
bool m5u_speaker_play_i16(const int16_t* samples, size_t len, uint32_t sample_rate_hz);
bool m5u_speaker_play_i16_ex(const int16_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound);
bool m5u_speaker_play_i8_ex(const int8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound);
bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz);
bool m5u_speaker_play_u8_ex(const uint8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound);
bool m5u_speaker_play_wav(const uint8_t* data, size_t len);
bool m5u_speaker_play_wav_ex(const uint8_t* data, size_t len, uint32_t repeat, int channel, bool stop_current_sound);
bool m5u_speaker_is_playing(int channel);
size_t m5u_speaker_playing_channels(void);
size_t m5u_speaker_channel_playing_state(int channel);
void m5u_speaker_stop(int channel);
uint8_t m5u_speaker_get_channel_volume(int channel);
void m5u_speaker_set_channel_volume(int channel, uint8_t volume);
void m5u_speaker_set_all_channel_volume(uint8_t volume);

typedef struct {
    uint32_t usec;
    float accel_x;
    float accel_y;
    float accel_z;
    float gyro_x;
    float gyro_y;
    float gyro_z;
    float mag_x;
    float mag_y;
    float mag_z;
} m5u_imu_data_t;

bool m5u_imu_begin(void);
bool m5u_imu_is_enabled(void);
int m5u_imu_get_type(void);
bool m5u_imu_update(void);
int m5u_imu_update_mask(void);
bool m5u_imu_sleep(void);
void m5u_imu_set_clock(uint32_t freq);
bool m5u_imu_set_axis_order(int axis0, int axis1, int axis2);
bool m5u_imu_set_axis_order_right_handed(int axis0, int axis1);
bool m5u_imu_set_axis_order_left_handed(int axis0, int axis1);
bool m5u_imu_set_int_pin_active_logic(bool level);
bool m5u_imu_get_accel(float* x, float* y, float* z);
bool m5u_imu_get_gyro(float* x, float* y, float* z);
bool m5u_imu_get_mag(float* x, float* y, float* z);
bool m5u_imu_get_data(m5u_imu_data_t* out);
bool m5u_imu_get_temp_c(float* temp);
bool m5u_imu_load_offset_from_nvs(void);
bool m5u_imu_save_offset_to_nvs(void);
float m5u_imu_get_offset_data(int index);
void m5u_imu_set_calibration(float x, float y, float z);
void m5u_imu_set_calibration_strength(uint8_t accel, uint8_t gyro, uint8_t mag);
void m5u_imu_clear_offset_data(void);
void m5u_imu_set_offset_data(size_t index, int32_t value);
int32_t m5u_imu_get_offset_data_i32(size_t index);
int16_t m5u_imu_get_raw_data(size_t index);

typedef struct {
    int x;
    int y;
    int prev_x;
    int prev_y;
    int base_x;
    int base_y;
    uint32_t base_msec;
    uint8_t state;
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
bool m5u_touch_is_enabled(void);
void m5u_touch_set_hold_thresh(uint16_t ms);
void m5u_touch_set_flick_thresh(uint16_t distance);

typedef struct {
    int year;
    int month;
    int day;
    int weekday;
    int hour;
    int minute;
    int second;
} m5u_rtc_datetime_t;

bool m5u_rtc_is_enabled(void);
bool m5u_rtc_get_volt_low(void);
bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second);
bool m5u_rtc_get_datetime_detail(m5u_rtc_datetime_t* out);
bool m5u_rtc_get_date_detail(m5u_rtc_datetime_t* out);
bool m5u_rtc_get_time_detail(m5u_rtc_datetime_t* out);
bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second);
bool m5u_rtc_set_datetime_detail(const m5u_rtc_datetime_t* datetime);
bool m5u_rtc_set_date_detail(const m5u_rtc_datetime_t* date);
bool m5u_rtc_set_time_detail(const m5u_rtc_datetime_t* time);
void m5u_rtc_set_system_time_from_rtc(void);
uint32_t m5u_rtc_set_timer_irq(uint32_t timer_msec);
int m5u_rtc_set_alarm_irq_after_seconds(int after_seconds);
int m5u_rtc_set_alarm_irq_datetime(const m5u_rtc_datetime_t* datetime);
int m5u_rtc_set_alarm_irq_time(const m5u_rtc_datetime_t* time);
bool m5u_rtc_get_irq_status(void);
void m5u_rtc_clear_irq(void);
void m5u_rtc_disable_irq(void);

int m5u_battery_level(void);
int m5u_battery_voltage_mv(void);
typedef struct {
    uint16_t voltage_mv;
    uint8_t current_limit_ma;
    bool enable;
    bool direction_output;
} m5u_power_ext_port_bus_t;
bool m5u_power_begin(void);
int m5u_power_get_type(void);
int m5u_power_get_charge_state(void);
bool m5u_power_is_charging(void);
void m5u_power_set_led(uint8_t brightness);
void m5u_power_set_ext_output(bool enable, uint16_t port_mask);
bool m5u_power_get_ext_output(void);
void m5u_power_set_usb_output(bool enable);
bool m5u_power_get_usb_output(void);
void m5u_power_set_battery_charge(bool enable);
void m5u_power_set_charge_current(uint16_t max_ma);
void m5u_power_set_charge_voltage(uint16_t max_mv);
int m5u_power_get_vbus_voltage_mv(void);
int m5u_power_get_battery_current_ma(void);
float m5u_power_get_ext_voltage_mv(uint16_t port_mask);
float m5u_power_get_ext_current_ma(uint16_t port_mask);
uint8_t m5u_power_get_key_state(void);
void m5u_power_set_ext_port_bus_config(const m5u_power_ext_port_bus_t* config);
void m5u_power_set_vibration(uint8_t level);
void m5u_power_power_off(void);
void m5u_power_timer_sleep_seconds(int seconds);
void m5u_power_deep_sleep_us(uint64_t micro_seconds, bool touch_wakeup);
void m5u_power_light_sleep_us(uint64_t micro_seconds, bool touch_wakeup);
bool m5u_power_axp2101_disable_irq(uint64_t mask);
bool m5u_power_axp2101_enable_irq(uint64_t mask);
bool m5u_power_axp2101_clear_irq_statuses(void);
uint64_t m5u_power_axp2101_get_irq_statuses(void);
bool m5u_power_axp2101_is_bat_charger_under_temperature_irq(void);
bool m5u_power_axp2101_is_bat_charger_over_temperature_irq(void);
bool m5u_power_axp2101_is_vbus_insert_irq(void);
bool m5u_power_axp2101_is_vbus_remove_irq(void);

typedef struct {
    uint8_t r;
    uint8_t g;
    uint8_t b;
} m5u_led_color_t;
bool m5u_led_begin(void);
void m5u_led_display(void);
void m5u_led_set_auto_display(bool enable);
size_t m5u_led_count(void);
void m5u_led_set_brightness(uint8_t brightness);
void m5u_led_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b);
void m5u_led_set_all_color_rgb(uint8_t r, uint8_t g, uint8_t b);
void m5u_led_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length);
int m5u_led_get_type(size_t index);
bool m5u_led_is_enabled(void);

void m5u_log_print(const char* text);
void m5u_log_println(const char* text);
void m5u_log_println_empty(void);
void m5u_log_level(int level, const char* text);
void m5u_log_dump(const void* addr, uint32_t len, int level);
const char* m5u_log_path_to_file_name(const char* path);
typedef void (*m5u_log_callback_t)(int level, bool use_color, const char* text, void* user_data);
bool m5u_log_set_callback(m5u_log_callback_t callback, void* user_data);
bool m5u_log_set_enable_color(int target, bool enable);
bool m5u_log_get_enable_color(int target);
bool m5u_log_set_level(int target, int level);
int m5u_log_get_level(int target);
bool m5u_log_set_suffix(int target, const char* suffix);

typedef struct {
    int pin_sclk;
    int pin_mosi;
    int pin_miso;
    int pin_cs;
    int host_id;
    uint32_t frequency_khz;
    int max_files;
    uint8_t format_if_mount_failed;
} m5u_sd_spi_config_t;

bool m5u_sd_begin(void);
bool m5u_sd_begin_spi(const m5u_sd_spi_config_t* config);
bool m5u_sd_is_mounted(void);
void m5u_sd_end(void);

#ifdef __cplusplus
}
#endif
