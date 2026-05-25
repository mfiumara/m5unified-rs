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

uint32_t m5u_millis(void) {
    return 0;
}

uint32_t m5u_micros(void) {
    return 0;
}

uint32_t m5u_get_update_msec(void) {
    return 0;
}

size_t m5u_heap_get_free_size(uint32_t caps) {
    (void)caps; return 0;
}

size_t m5u_heap_get_largest_free_block(uint32_t caps) {
    (void)caps; return 0;
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

bool m5u_set_primary_display_types(const int* kinds, size_t len) {
    (void)kinds; (void)len; return false;
}

void m5u_set_log_display_index(size_t index) {
    (void)index;
}

void m5u_set_log_display_type(int kind) {
    (void)kind;
}

void m5u_set_log_display_types(const int* kinds, size_t len) {
    (void)kinds; (void)len;
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

bool m5u_io_expander_available(size_t index) { (void)index; return false; }
bool m5u_io_expander_set_direction(size_t index, uint8_t pin, bool output) { (void)index; (void)pin; (void)output; return false; }
bool m5u_io_expander_enable_pull(size_t index, uint8_t pin, bool enable) { (void)index; (void)pin; (void)enable; return false; }
bool m5u_io_expander_set_pull_mode(size_t index, uint8_t pin, bool pull_up) { (void)index; (void)pin; (void)pull_up; return false; }
bool m5u_io_expander_set_high_impedance(size_t index, uint8_t pin, bool enable) { (void)index; (void)pin; (void)enable; return false; }
bool m5u_io_expander_get_write_value(size_t index, uint8_t pin) { (void)index; (void)pin; return false; }
bool m5u_io_expander_digital_write(size_t index, uint8_t pin, bool level) { (void)index; (void)pin; (void)level; return false; }
bool m5u_io_expander_digital_read(size_t index, uint8_t pin) { (void)index; (void)pin; return false; }
bool m5u_io_expander_reset_irq(size_t index) { (void)index; return false; }
bool m5u_io_expander_disable_irq(size_t index) { (void)index; return false; }
bool m5u_io_expander_enable_irq(size_t index) { (void)index; return false; }
bool m5u_pi4ioe5v6408_begin(void) { return false; }
bool m5u_pi4ioe5v6408_set_direction(uint8_t pin, bool output) { (void)pin; (void)output; return false; }
bool m5u_pi4ioe5v6408_enable_pull(uint8_t pin, bool enable) { (void)pin; (void)enable; return false; }
bool m5u_pi4ioe5v6408_set_pull_mode(uint8_t pin, bool pull_up) { (void)pin; (void)pull_up; return false; }
bool m5u_pi4ioe5v6408_set_high_impedance(uint8_t pin, bool enable) { (void)pin; (void)enable; return false; }
bool m5u_pi4ioe5v6408_get_write_value(uint8_t pin) { (void)pin; return false; }
bool m5u_pi4ioe5v6408_digital_write(uint8_t pin, bool level) { (void)pin; (void)level; return false; }
bool m5u_pi4ioe5v6408_digital_read(uint8_t pin) { (void)pin; return false; }
void m5u_pi4ioe5v6408_reset_irq(void) {}
void m5u_pi4ioe5v6408_disable_irq(void) {}
void m5u_pi4ioe5v6408_enable_irq(void) {}

void m5u_i2c_set_port(int bus, int port_num, int pin_sda, int pin_scl) {
    (void)bus; (void)port_num; (void)pin_sda; (void)pin_scl;
}

bool m5u_i2c_begin(int bus) {
    (void)bus; return false;
}

bool m5u_i2c_begin_with_port(int bus, int port_num, int pin_sda, int pin_scl) {
    (void)bus; (void)port_num; (void)pin_sda; (void)pin_scl; return false;
}

bool m5u_i2c_release(int bus) {
    (void)bus; return false;
}

bool m5u_i2c_is_enabled(int bus) {
    (void)bus; return false;
}

int m5u_i2c_get_port(int bus) {
    (void)bus; return -1;
}

int m5u_i2c_get_sda(int bus) {
    (void)bus; return -1;
}

int m5u_i2c_get_scl(int bus) {
    (void)bus; return -1;
}

bool m5u_i2c_start(int bus, uint8_t address, bool read, uint32_t freq) {
    (void)bus; (void)address; (void)read; (void)freq; return false;
}

bool m5u_i2c_restart(int bus, uint8_t address, bool read, uint32_t freq) {
    (void)bus; (void)address; (void)read; (void)freq; return false;
}

bool m5u_i2c_stop(int bus) {
    (void)bus; return false;
}

bool m5u_i2c_write_byte(int bus, uint8_t data) {
    (void)bus; (void)data; return false;
}

bool m5u_i2c_write(int bus, const uint8_t* data, size_t length) {
    (void)bus; (void)data; (void)length; return false;
}

bool m5u_i2c_read(int bus, uint8_t* result, size_t length, bool last_nack) {
    (void)bus; (void)result; (void)length; (void)last_nack; return false;
}

bool m5u_i2c_write_register(int bus, uint8_t address, uint8_t reg, const uint8_t* data, size_t length, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)data; (void)length; (void)freq; return false;
}

bool m5u_i2c_read_register(int bus, uint8_t address, uint8_t reg, uint8_t* result, size_t length, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)result; (void)length; (void)freq; return false;
}

bool m5u_i2c_write_register8(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)data; (void)freq; return false;
}

uint8_t m5u_i2c_read_register8(int bus, uint8_t address, uint8_t reg, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)freq; return 0;
}

bool m5u_i2c_bit_on(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)data; (void)freq; return false;
}

bool m5u_i2c_bit_off(int bus, uint8_t address, uint8_t reg, uint8_t data, uint32_t freq) {
    (void)bus; (void)address; (void)reg; (void)data; (void)freq; return false;
}

void m5u_i2c_scan(int bus, bool* result, uint32_t freq) {
    (void)bus; (void)freq;
    if (result) {
        for (size_t i = 0; i < 120; ++i) {
            result[i] = false;
        }
    }
}

bool m5u_i2c_scan_address(int bus, uint8_t address, uint32_t freq) {
    (void)bus; (void)address; (void)freq; return false;
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

bool m5u_mic_is_running(void) {
    return false;
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    (void)buffer; (void)samples; return false;
}

bool m5u_mic_record_u8(uint8_t* buffer, size_t samples) {
    (void)buffer; (void)samples; return false;
}

bool m5u_speaker_begin(void) {
    return false;
}

bool m5u_speaker_is_running(void) {
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

bool m5u_imu_begin_for_board(int board) {
    (void)board; return false;
}

bool m5u_imu_get_accel(float* x, float* y, float* z) {
    (void)x; (void)y; (void)z; return false;
}

bool m5u_imu_get_gyro(float* x, float* y, float* z) {
    (void)x; (void)y; (void)z; return false;
}

bool m5u_imu_get_mag(float* x, float* y, float* z) {
    (void)x; (void)y; (void)z; return false;
}

bool m5u_imu_get_data(m5u_imu_data_t* out) {
    (void)out; return false;
}

bool m5u_imu_get_temp_c(float* temp) {
    (void)temp; return false;
}

int m5u_touch_count(void) {
    return 0;
}

bool m5u_touch_is_enabled(void) {
    return false;
}

void m5u_touch_set_hold_thresh(uint16_t ms) {
    (void)ms;
}

void m5u_touch_set_flick_thresh(uint16_t distance) {
    (void)distance;
}

bool m5u_touch_get(int index, int* x, int* y) {
    (void)index; (void)x; (void)y; return false;
}

bool m5u_touch_get_raw(int index, int* x, int* y) {
    (void)index; (void)x; (void)y; return false;
}

bool m5u_rtc_begin(void) {
    return false;
}

bool m5u_rtc_begin_for_board(int board) {
    (void)board; return false;
}

bool m5u_rtc_get_datetime(int* year, int* month, int* day, int* hour, int* minute, int* second) {
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
}

bool m5u_rtc_get_datetime_detail(m5u_rtc_datetime_t* out) {
    (void)out; return false;
}

bool m5u_rtc_get_date_detail(m5u_rtc_datetime_t* out) {
    (void)out; return false;
}

bool m5u_rtc_get_time_detail(m5u_rtc_datetime_t* out) {
    (void)out; return false;
}

bool m5u_rtc_set_datetime(int year, int month, int day, int hour, int minute, int second) {
    (void)year; (void)month; (void)day; (void)hour; (void)minute; (void)second; return false;
}

bool m5u_rtc_set_datetime_detail(const m5u_rtc_datetime_t* datetime) {
    (void)datetime; return false;
}

bool m5u_rtc_set_date_detail(const m5u_rtc_datetime_t* date) {
    (void)date; return false;
}

bool m5u_rtc_set_time_detail(const m5u_rtc_datetime_t* time) {
    (void)time; return false;
}

void m5u_rtc_set_system_time_from_rtc(void) {
}

bool m5u_rtc_get_volt_low(void) {
    return false;
}

uint32_t m5u_rtc_set_timer_irq(uint32_t timer_msec) {
    (void)timer_msec; return 0;
}

int m5u_rtc_set_alarm_irq_after_seconds(int after_seconds) {
    (void)after_seconds; return -1;
}

int m5u_rtc_set_alarm_irq_datetime(const m5u_rtc_datetime_t* datetime) {
    (void)datetime; return -1;
}

int m5u_rtc_set_alarm_irq_time(const m5u_rtc_datetime_t* time) {
    (void)time; return -1;
}

bool m5u_rtc_get_irq_status(void) {
    return false;
}

void m5u_rtc_clear_irq(void) {
}

void m5u_rtc_disable_irq(void) {
}

bool m5u_rtc_device_begin(int kind) {
    (void)kind; return false;
}

bool m5u_rtc_device_get_datetime_detail(int kind, m5u_rtc_datetime_t* out) {
    (void)kind; (void)out; return false;
}

bool m5u_rtc_device_get_date_detail(int kind, m5u_rtc_datetime_t* out) {
    (void)kind; (void)out; return false;
}

bool m5u_rtc_device_get_time_detail(int kind, m5u_rtc_datetime_t* out) {
    (void)kind; (void)out; return false;
}

bool m5u_rtc_device_set_datetime_detail(int kind, const m5u_rtc_datetime_t* datetime) {
    (void)kind; (void)datetime; return false;
}

bool m5u_rtc_device_set_date_detail(int kind, const m5u_rtc_datetime_t* date) {
    (void)kind; (void)date; return false;
}

bool m5u_rtc_device_set_time_detail(int kind, const m5u_rtc_datetime_t* time) {
    (void)kind; (void)time; return false;
}

bool m5u_rtc_device_get_volt_low(int kind) {
    (void)kind; return false;
}

uint32_t m5u_rtc_device_set_timer_irq(int kind, uint32_t timer_msec) {
    (void)kind; (void)timer_msec; return 0;
}

int m5u_rtc_device_set_alarm_irq_datetime(int kind, const m5u_rtc_datetime_t* datetime) {
    (void)kind; (void)datetime; return -1;
}

int m5u_rtc_device_set_alarm_irq_time(int kind, const m5u_rtc_datetime_t* time) {
    (void)kind; (void)time; return -1;
}

bool m5u_rtc_device_get_irq_status(int kind) {
    (void)kind; return false;
}

void m5u_rtc_device_clear_irq(int kind) {
    (void)kind;
}

void m5u_rtc_device_disable_irq(int kind) {
    (void)kind;
}

int m5u_battery_level(void) {
    return -1;
}

int m5u_battery_voltage_mv(void) {
    return -1;
}

bool m5u_power_begin(void) {
    return false;
}

int m5u_power_get_type(void) {
    return 0;
}

int m5u_power_get_charge_state(void) {
    return 2;
}

bool m5u_power_is_charging(void) {
    return false;
}

void m5u_power_set_led(uint8_t brightness) {
    (void)brightness;
}

void m5u_power_set_ext_output(bool enable, uint16_t port_mask) {
    (void)enable; (void)port_mask;
}

bool m5u_power_get_ext_output(void) {
    return false;
}

void m5u_power_set_usb_output(bool enable) {
    (void)enable;
}

bool m5u_power_get_usb_output(void) {
    return false;
}

void m5u_power_set_battery_charge(bool enable) {
    (void)enable;
}

void m5u_power_set_charge_current(uint16_t max_ma) {
    (void)max_ma;
}

void m5u_power_set_charge_voltage(uint16_t max_mv) {
    (void)max_mv;
}

int m5u_power_get_vbus_voltage_mv(void) {
    return -1;
}

int m5u_power_get_battery_current_ma(void) {
    return 0;
}

float m5u_power_get_ext_voltage_mv(uint16_t port_mask) {
    (void)port_mask; return 0.0f;
}

float m5u_power_get_ext_current_ma(uint16_t port_mask) {
    (void)port_mask; return 0.0f;
}

uint8_t m5u_power_get_key_state(void) {
    return 0;
}

void m5u_power_set_ext_port_bus_config(const m5u_power_ext_port_bus_t* config) {
    (void)config;
}

void m5u_power_set_vibration(uint8_t level) {
    (void)level;
}

void m5u_power_power_off(void) {
}

void m5u_power_timer_sleep_seconds(int seconds) {
    (void)seconds;
}

void m5u_power_timer_sleep_time(const m5u_rtc_datetime_t* time) {
    (void)time;
}

void m5u_power_timer_sleep_date_time(const m5u_rtc_datetime_t* date, const m5u_rtc_datetime_t* time) {
    (void)date; (void)time;
}

void m5u_power_deep_sleep_us(uint64_t micro_seconds, bool touch_wakeup) {
    (void)micro_seconds; (void)touch_wakeup;
}

void m5u_power_light_sleep_us(uint64_t micro_seconds, bool touch_wakeup) {
    (void)micro_seconds; (void)touch_wakeup;
}

void m5u_log_println(const char* text) {
    (void)text;
}

void m5u_log_println_empty(void) {
}

bool m5u_sd_begin(void) {
    return false;
}

bool m5u_sd_begin_spi(const m5u_sd_spi_config_t* config) {
    (void)config;
    return false;
}

bool m5u_sd_is_mounted(void) {
    return false;
}

void m5u_sd_end(void) {
}


static bool m5u_button_state(int button, int query) {
    (void)button;
    return query == 10;
}

int m5u_display_get_rotation(void) {
    return 0;
}

void m5u_display_set_brightness(uint8_t brightness) {
    (void)brightness;
}

uint8_t m5u_display_get_brightness(void) {
    return 0;
}

void m5u_display_sleep(void) {
}

void m5u_display_wakeup(void) {
}

void m5u_display_power_save(bool enable) {
    (void)enable;
}

void m5u_display_invert_display(bool invert) {
    (void)invert;
}

bool m5u_display_get_invert(void) {
    return false;
}

void m5u_display_set_swap_bytes(bool swap) {
    (void)swap;
}

bool m5u_display_get_swap_bytes(void) {
    return false;
}

void m5u_display_set_color_depth(int depth) {
    (void)depth;
}

int m5u_display_get_color_depth(void) {
    return 16;
}

void m5u_display_set_addr_window(int x, int y, int w, int h) {
    (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_set_window(int xs, int ys, int xe, int ye) {
    (void)xs; (void)ys; (void)xe; (void)ye;
}

void m5u_display_begin_transaction(void) {
}

void m5u_display_end_transaction(void) {
}

uint32_t m5u_display_get_start_count(void) {
    return 0;
}

int m5u_display_get_scan_line(void) {
    return 0;
}

void m5u_display_set_raw_color(uint32_t color) {
    (void)color;
}

uint32_t m5u_display_get_raw_color(void) {
    return 0;
}

void m5u_display_write_color(uint16_t color, uint32_t length) {
    (void)color; (void)length;
}

void m5u_display_draw_pixel_current(int x, int y) {
    (void)x; (void)y;
}

void m5u_display_write_pixel_current(int x, int y) {
    (void)x; (void)y;
}

void m5u_display_write_fill_rect(int x, int y, int w, int h, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_write_fill_rect_preclipped(int x, int y, int w, int h, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_push_block(uint16_t color, uint32_t length) {
    (void)color; (void)length;
}

void m5u_display_progress_bar(int x, int y, int w, int h, uint8_t value) {
    (void)x; (void)y; (void)w; (void)h; (void)value;
}

void m5u_display_push_state(void) {
}

void m5u_display_pop_state(void) {
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

void m5u_display_display_region(int x, int y, int w, int h) {
    (void)x; (void)y; (void)w; (void)h;
}

bool m5u_display_display_busy(void) {
    return false;
}

void m5u_display_wait_display(void) {
}

bool m5u_display_has_palette(void) {
    return false;
}

uint32_t m5u_display_get_palette_count(void) {
    return 0;
}

bool m5u_display_is_readable(void) {
    return false;
}

bool m5u_display_is_epd(void) {
    return false;
}

bool m5u_display_is_bus_shared(void) {
    return false;
}

void m5u_display_set_auto_display(bool enable) {
    (void)enable;
}

void m5u_display_init_dma(void) {
}

void m5u_display_wait_dma(void) {
}

bool m5u_display_dma_busy(void) {
    return false;
}

int m5u_display_get_cursor_x(void) {
    return 0;
}

int m5u_display_get_cursor_y(void) {
    return 0;
}

int m5u_display_font_height(void) {
    return 16;
}

uint32_t m5u_display_get_base_color(void) {
    return 0;
}

void m5u_display_set_base_color(uint32_t color) {
    (void)color;
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

int m5u_display_draw_center_string(const char* text, int x, int y) {
    (void)text; (void)x; (void)y; return 0;
}

int m5u_display_draw_right_string(const char* text, int x, int y) {
    (void)text; (void)x; (void)y; return 0;
}

int m5u_display_draw_number(int value, int x, int y) {
    (void)value; (void)x; (void)y; return 0;
}

int m5u_display_draw_float(float value, uint8_t decimals, int x, int y) {
    (void)value; (void)decimals; (void)x; (void)y; return 0;
}

int m5u_display_draw_char(uint16_t codepoint, int x, int y) {
    (void)codepoint; (void)x; (void)y; return 0;
}

void m5u_display_draw_pixel(int x, int y, uint16_t color) {
    (void)x; (void)y; (void)color;
}

void m5u_display_write_pixel(int x, int y, uint16_t color) {
    (void)x; (void)y; (void)color;
}

void m5u_display_draw_fast_hline(int x, int y, int w, uint16_t color) {
    (void)x; (void)y; (void)w; (void)color;
}

void m5u_display_write_fast_hline(int x, int y, int w, uint16_t color) {
    (void)x; (void)y; (void)w; (void)color;
}

void m5u_display_draw_fast_vline(int x, int y, int h, uint16_t color) {
    (void)x; (void)y; (void)h; (void)color;
}

void m5u_display_write_fast_vline(int x, int y, int h, uint16_t color) {
    (void)x; (void)y; (void)h; (void)color;
}

void m5u_display_draw_round_rect(int x, int y, int w, int h, int r, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_fill_round_rect(int x, int y, int w, int h, int r, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_draw_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_fill_triangle(int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_draw_ellipse(int x, int y, int rx, int ry, uint16_t color) {
    (void)x; (void)y; (void)rx; (void)ry; (void)color;
}

void m5u_display_fill_ellipse(int x, int y, int rx, int ry, uint16_t color) {
    (void)x; (void)y; (void)rx; (void)ry; (void)color;
}

void m5u_display_draw_arc(int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
    (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_fill_arc(int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
    (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_draw_ellipse_arc(int x, int y, int r0x, int r1x, int r0y, int r1y, float angle0, float angle1, uint16_t color) {
    (void)x; (void)y; (void)r0x; (void)r1x; (void)r0y; (void)r1y; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_fill_ellipse_arc(int x, int y, int r0x, int r1x, int r0y, int r1y, float angle0, float angle1, uint16_t color) {
    (void)x; (void)y; (void)r0x; (void)r1x; (void)r0y; (void)r1y; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_draw_bezier3(int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_draw_bezier4(int x0, int y0, int x1, int y1, int x2, int y2, int x3, int y3, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)x3; (void)y3; (void)color;
}

void m5u_display_draw_smooth_line(int x0, int y0, int x1, int y1, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
}

void m5u_display_draw_wide_line(int x0, int y0, int x1, int y1, float radius, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)radius; (void)color;
}

void m5u_display_draw_wedge_line(int x0, int y0, int x1, int y1, float r0, float r1, uint16_t color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)r0; (void)r1; (void)color;
}

void m5u_display_draw_gradient_line(int x0, int y0, int x1, int y1, uint16_t start_color, uint16_t end_color) {
    (void)x0; (void)y0; (void)x1; (void)y1; (void)start_color; (void)end_color;
}

void m5u_display_draw_spot(int x, int y, float radius, uint16_t color) {
    (void)x; (void)y; (void)radius; (void)color;
}

void m5u_display_fill_smooth_circle(int x, int y, int r, uint16_t color) {
    (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_fill_smooth_round_rect(int x, int y, int w, int h, int r, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_fill_gradient_rect(int x, int y, int w, int h, uint16_t start_color, uint16_t end_color, int style) {
    (void)x; (void)y; (void)w; (void)h; (void)start_color; (void)end_color; (void)style;
}

void m5u_display_flood_fill(int x, int y, uint16_t color) {
    (void)x; (void)y; (void)color;
}

void m5u_display_set_scroll_rect(int x, int y, int w, int h) {
    (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_set_scroll_rect_color(int x, int y, int w, int h, uint16_t color) {
    (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_get_scroll_rect(int* x, int* y, int* w, int* h) {
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_display_clear_scroll_rect(void) {
}

void m5u_display_scroll(int dx, int dy) {
    (void)dx; (void)dy;
}

int m5u_display_text_width(const char* text) {
    (void)text; return 0;
}

int m5u_display_text_length(const char* text, int width) {
    (void)text; (void)width; return 0;
}

int m5u_display_get_text_datum(void) {
    return 0;
}

int m5u_display_font_width(void) {
    return 6;
}

void m5u_display_set_text_padding(uint32_t padding) {
    (void)padding;
}

uint32_t m5u_display_get_text_padding(void) {
    return 0;
}

float m5u_display_get_text_size_x(void) {
    return 1.0f;
}

float m5u_display_get_text_size_y(void) {
    return 1.0f;
}

void m5u_display_set_clip_rect(int x, int y, int w, int h) {
    (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_get_clip_rect(int* x, int* y, int* w, int* h) {
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_display_clear_clip_rect(void) {
}

uint32_t m5u_display_color888(uint8_t r, uint8_t g, uint8_t b) {
    return ((uint32_t)r << 16) | ((uint32_t)g << 8) | b;
}

void m5u_display_set_pivot(float x, float y) {
    (void)x; (void)y;
}

float m5u_display_get_pivot_x(void) {
    return 0.0f;
}

float m5u_display_get_pivot_y(void) {
    return 0.0f;
}

bool m5u_display_push_image_rgb565(int x, int y, int w, int h, const uint16_t* data) {
    (void)x; (void)y; return data && w > 0 && h > 0;
}

bool m5u_display_push_image_rgb565_transparent(int x, int y, int w, int h, const uint16_t* data, uint16_t transparent) {
    (void)x; (void)y; (void)transparent; return data && w > 0 && h > 0;
}

uint16_t m5u_display_read_pixel(int x, int y) {
    (void)x; (void)y; return 0;
}

bool m5u_display_read_rect_rgb565(int x, int y, int w, int h, uint16_t* data) {
    (void)x; (void)y;
    if (!data || w <= 0 || h <= 0) {
        return false;
    }
    for (int i = 0; i < w * h; ++i) {
        data[i] = 0;
    }
    return true;
}

void m5u_display_copy_rect(int dst_x, int dst_y, int w, int h, int src_x, int src_y) {
    (void)dst_x; (void)dst_y; (void)w; (void)h; (void)src_x; (void)src_y;
}

bool m5u_display_draw_image(int format, const uint8_t* data, size_t len, const m5u_image_options_t* options) {
    (void)options;
    return format >= 0 && format <= 3 && data && len > 0;
}

bool m5u_display_draw_image_file(int format, const char* path, const m5u_image_options_t* options) {
    (void)options;
    return format >= 0 && format <= 3 && path && *path;
}

void m5u_display_qrcode(const char* text, int x, int y, int width, uint8_t version, bool margin) {
    (void)text; (void)x; (void)y; (void)width; (void)version; (void)margin;
}

int m5u_display_count(void) {
    return 1;
}

int m5u_display_index_for_kind(int kind) {
    (void)kind; return -1;
}

int m5u_display_index_for_kinds(const int* kinds, size_t len) {
    (void)kinds; (void)len; return -1;
}

int m5u_display_width_at(int index) {
    (void)index; return 320;
}

int m5u_display_height_at(int index) {
    (void)index; return 240;
}

void m5u_display_fill_screen_at(int index, uint16_t color) {
    (void)index; (void)color;
}

void m5u_display_set_cursor_at(int index, int x, int y) {
    (void)index; (void)x; (void)y;
}

void m5u_display_set_text_size_at(int index, int size) {
    (void)index; (void)size;
}

void m5u_display_set_text_color_at(int index, uint16_t fg, uint16_t bg) {
    (void)index; (void)fg; (void)bg;
}

int m5u_display_get_rotation_at(int index) {
    (void)index; return 0;
}

void m5u_display_set_rotation_at(int index, int rotation) {
    (void)index; (void)rotation;
}

void m5u_display_set_brightness_at(int index, uint8_t brightness) {
    (void)index; (void)brightness;
}

uint8_t m5u_display_get_brightness_at(int index) {
    (void)index; return 0;
}

void m5u_display_sleep_at(int index) {
    (void)index;
}

void m5u_display_wakeup_at(int index) {
    (void)index;
}

void m5u_display_power_save_at(int index, bool enable) {
    (void)index; (void)enable;
}

void m5u_display_invert_display_at(int index, bool invert) {
    (void)index; (void)invert;
}

bool m5u_display_get_invert_at(int index) {
    (void)index; return false;
}

void m5u_display_set_swap_bytes_at(int index, bool swap) {
    (void)index; (void)swap;
}

bool m5u_display_get_swap_bytes_at(int index) {
    (void)index; return false;
}

void m5u_display_set_color_depth_at(int index, int depth) {
    (void)index; (void)depth;
}

int m5u_display_get_color_depth_at(int index) {
    (void)index; return 16;
}

void m5u_display_set_addr_window_at(int index, int x, int y, int w, int h) {
    (void)index; (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_set_window_at(int index, int xs, int ys, int xe, int ye) {
    (void)index; (void)xs; (void)ys; (void)xe; (void)ye;
}

void m5u_display_begin_transaction_at(int index) {
    (void)index;
}

void m5u_display_end_transaction_at(int index) {
    (void)index;
}

uint32_t m5u_display_get_start_count_at(int index) {
    (void)index; return 0;
}

int m5u_display_get_scan_line_at(int index) {
    (void)index; return 0;
}

void m5u_display_set_raw_color_at(int index, uint32_t color) {
    (void)index; (void)color;
}

uint32_t m5u_display_get_raw_color_at(int index) {
    (void)index; return 0;
}

void m5u_display_write_color_at(int index, uint16_t color, uint32_t length) {
    (void)index; (void)color; (void)length;
}

void m5u_display_draw_pixel_current_at(int index, int x, int y) {
    (void)index; (void)x; (void)y;
}

void m5u_display_write_pixel_current_at(int index, int x, int y) {
    (void)index; (void)x; (void)y;
}

void m5u_display_write_fill_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_write_fill_rect_preclipped_at(int index, int x, int y, int w, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_push_block_at(int index, uint16_t color, uint32_t length) {
    (void)index; (void)color; (void)length;
}

void m5u_display_progress_bar_at(int index, int x, int y, int w, int h, uint8_t value) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)value;
}

void m5u_display_push_state_at(int index) {
    (void)index;
}

void m5u_display_pop_state_at(int index) {
    (void)index;
}

void m5u_display_set_color_at(int index, uint16_t color) {
    (void)index; (void)color;
}

uint32_t m5u_display_get_base_color_at(int index) {
    (void)index; return 0;
}

void m5u_display_set_base_color_at(int index, uint32_t color) {
    (void)index; (void)color;
}

int m5u_display_get_cursor_x_at(int index) {
    (void)index; return 0;
}

int m5u_display_get_cursor_y_at(int index) {
    (void)index; return 0;
}

void m5u_display_start_write_at(int index) {
    (void)index;
}

void m5u_display_end_write_at(int index) {
    (void)index;
}

void m5u_display_display_at(int index) {
    (void)index;
}

void m5u_display_display_region_at(int index, int x, int y, int w, int h) {
    (void)index; (void)x; (void)y; (void)w; (void)h;
}

bool m5u_display_display_busy_at(int index) {
    (void)index; return false;
}

void m5u_display_wait_display_at(int index) {
    (void)index;
}

bool m5u_display_has_palette_at(int index) {
    (void)index; return false;
}

uint32_t m5u_display_get_palette_count_at(int index) {
    (void)index; return 0;
}

bool m5u_display_is_readable_at(int index) {
    (void)index; return false;
}

bool m5u_display_is_epd_at(int index) {
    (void)index; return false;
}

bool m5u_display_is_bus_shared_at(int index) {
    (void)index; return false;
}

void m5u_display_set_auto_display_at(int index, bool enable) {
    (void)index; (void)enable;
}

void m5u_display_init_dma_at(int index) {
    (void)index;
}

void m5u_display_wait_dma_at(int index) {
    (void)index;
}

bool m5u_display_dma_busy_at(int index) {
    (void)index; return false;
}

void m5u_display_print_at(int index, const char* text) {
    (void)index; (void)text;
}

void m5u_display_println_at(int index, const char* text) {
    (void)index; (void)text;
}

int m5u_display_draw_string_at(int index, const char* text, int x, int y) {
    (void)index; (void)text; (void)x; (void)y; return 0;
}

int m5u_display_draw_center_string_at(int index, const char* text, int x, int y) {
    (void)index; (void)text; (void)x; (void)y; return 0;
}

int m5u_display_draw_right_string_at(int index, const char* text, int x, int y) {
    (void)index; (void)text; (void)x; (void)y; return 0;
}

int m5u_display_draw_number_at(int index, int value, int x, int y) {
    (void)index; (void)value; (void)x; (void)y; return 0;
}

int m5u_display_draw_float_at(int index, float value, uint8_t decimals, int x, int y) {
    (void)index; (void)value; (void)decimals; (void)x; (void)y; return 0;
}

int m5u_display_draw_char_at(int index, uint16_t codepoint, int x, int y) {
    (void)index; (void)codepoint; (void)x; (void)y; return 0;
}

void m5u_display_draw_line_at(int index, int x0, int y0, int x1, int y1, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
}

void m5u_display_draw_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_fill_rect_at(int index, int x, int y, int w, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_draw_circle_at(int index, int x, int y, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_write_pixel_at(int index, int x, int y, uint16_t color) {
    (void)index; (void)x; (void)y; (void)color;
}

void m5u_display_draw_pixel_at(int index, int x, int y, uint16_t color) {
    (void)index; (void)x; (void)y; (void)color;
}

void m5u_display_draw_fast_hline_at(int index, int x, int y, int w, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)color;
}

void m5u_display_write_fast_hline_at(int index, int x, int y, int w, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)color;
}

void m5u_display_draw_fast_vline_at(int index, int x, int y, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)h; (void)color;
}

void m5u_display_write_fast_vline_at(int index, int x, int y, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)h; (void)color;
}

void m5u_display_draw_round_rect_at(int index, int x, int y, int w, int h, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_fill_round_rect_at(int index, int x, int y, int w, int h, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_draw_triangle_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_fill_triangle_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_draw_ellipse_at(int index, int x, int y, int rx, int ry, uint16_t color) {
    (void)index; (void)x; (void)y; (void)rx; (void)ry; (void)color;
}

void m5u_display_fill_ellipse_at(int index, int x, int y, int rx, int ry, uint16_t color) {
    (void)index; (void)x; (void)y; (void)rx; (void)ry; (void)color;
}

void m5u_display_draw_arc_at(int index, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_fill_arc_at(int index, int x, int y, int r0, int r1, float angle0, float angle1, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r0; (void)r1; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_draw_ellipse_arc_at(int index, int x, int y, int r0x, int r1x, int r0y, int r1y, float angle0, float angle1, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r0x; (void)r1x; (void)r0y; (void)r1y; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_fill_ellipse_arc_at(int index, int x, int y, int r0x, int r1x, int r0y, int r1y, float angle0, float angle1, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r0x; (void)r1x; (void)r0y; (void)r1y; (void)angle0; (void)angle1; (void)color;
}

void m5u_display_draw_bezier3_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)color;
}

void m5u_display_draw_bezier4_at(int index, int x0, int y0, int x1, int y1, int x2, int y2, int x3, int y3, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)x2; (void)y2; (void)x3; (void)y3; (void)color;
}

void m5u_display_draw_smooth_line_at(int index, int x0, int y0, int x1, int y1, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)color;
}

void m5u_display_draw_wide_line_at(int index, int x0, int y0, int x1, int y1, float radius, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)radius; (void)color;
}

void m5u_display_draw_wedge_line_at(int index, int x0, int y0, int x1, int y1, float r0, float r1, uint16_t color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)r0; (void)r1; (void)color;
}

void m5u_display_draw_gradient_line_at(int index, int x0, int y0, int x1, int y1, uint16_t start_color, uint16_t end_color) {
    (void)index; (void)x0; (void)y0; (void)x1; (void)y1; (void)start_color; (void)end_color;
}

void m5u_display_draw_spot_at(int index, int x, int y, float radius, uint16_t color) {
    (void)index; (void)x; (void)y; (void)radius; (void)color;
}

void m5u_display_fill_smooth_circle_at(int index, int x, int y, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)r; (void)color;
}

void m5u_display_fill_smooth_round_rect_at(int index, int x, int y, int w, int h, int r, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)r; (void)color;
}

void m5u_display_fill_gradient_rect_at(int index, int x, int y, int w, int h, uint16_t start_color, uint16_t end_color, int style) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)start_color; (void)end_color; (void)style;
}

void m5u_display_flood_fill_at(int index, int x, int y, uint16_t color) {
    (void)index; (void)x; (void)y; (void)color;
}

void m5u_display_set_scroll_rect_at(int index, int x, int y, int w, int h) {
    (void)index; (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_set_scroll_rect_color_at(int index, int x, int y, int w, int h, uint16_t color) {
    (void)index; (void)x; (void)y; (void)w; (void)h; (void)color;
}

void m5u_display_get_scroll_rect_at(int index, int* x, int* y, int* w, int* h) {
    (void)index;
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_display_clear_scroll_rect_at(int index) {
    (void)index;
}

void m5u_display_scroll_at(int index, int dx, int dy) {
    (void)index; (void)dx; (void)dy;
}

int m5u_display_text_width_at(int index, const char* text) {
    (void)index; (void)text; return 0;
}

int m5u_display_text_length_at(int index, const char* text, int width) {
    (void)index; (void)text; (void)width; return 0;
}

int m5u_display_get_text_datum_at(int index) {
    (void)index; return 0;
}

int m5u_display_font_width_at(int index) {
    (void)index; return 6;
}

void m5u_display_set_text_padding_at(int index, uint32_t padding) {
    (void)index; (void)padding;
}

uint32_t m5u_display_get_text_padding_at(int index) {
    (void)index; return 0;
}

float m5u_display_get_text_size_x_at(int index) {
    (void)index; return 1.0f;
}

float m5u_display_get_text_size_y_at(int index) {
    (void)index; return 1.0f;
}

void m5u_display_set_clip_rect_at(int index, int x, int y, int w, int h) {
    (void)index; (void)x; (void)y; (void)w; (void)h;
}

void m5u_display_get_clip_rect_at(int index, int* x, int* y, int* w, int* h) {
    (void)index;
    if (x) { *x = 0; }
    if (y) { *y = 0; }
    if (w) { *w = 0; }
    if (h) { *h = 0; }
}

void m5u_display_clear_clip_rect_at(int index) {
    (void)index;
}

void m5u_display_set_pivot_at(int index, float x, float y) {
    (void)index; (void)x; (void)y;
}

float m5u_display_get_pivot_x_at(int index) {
    (void)index; return 0.0f;
}

float m5u_display_get_pivot_y_at(int index) {
    (void)index; return 0.0f;
}

bool m5u_display_push_image_rgb565_at(int index, int x, int y, int w, int h, const uint16_t* data) {
    (void)index; (void)x; (void)y; return data && w > 0 && h > 0;
}

bool m5u_display_push_image_rgb565_transparent_at(int index, int x, int y, int w, int h, const uint16_t* data, uint16_t transparent) {
    (void)index; (void)x; (void)y; (void)transparent; return data && w > 0 && h > 0;
}

uint16_t m5u_display_read_pixel_at(int index, int x, int y) {
    (void)index; (void)x; (void)y; return 0;
}

bool m5u_display_read_rect_rgb565_at(int index, int x, int y, int w, int h, uint16_t* data) {
    (void)index; (void)x; (void)y;
    if (!data || w <= 0 || h <= 0) {
        return false;
    }
    for (int i = 0; i < w * h; ++i) {
        data[i] = 0;
    }
    return true;
}

void m5u_display_copy_rect_at(int index, int dst_x, int dst_y, int w, int h, int src_x, int src_y) {
    (void)index; (void)dst_x; (void)dst_y; (void)w; (void)h; (void)src_x; (void)src_y;
}

bool m5u_display_draw_image_at(int index, int format, const uint8_t* data, size_t len, const m5u_image_options_t* options) {
    (void)index; (void)options;
    return format >= 0 && format <= 3 && data && len > 0;
}

bool m5u_display_draw_image_file_at(int index, int format, const char* path, const m5u_image_options_t* options) {
    (void)index; (void)options;
    return format >= 0 && format <= 3 && path && *path;
}

void m5u_display_qrcode_at(int index, const char* text, int x, int y, int width, uint8_t version, bool margin) {
    (void)index; (void)text; (void)x; (void)y; (void)width; (void)version; (void)margin;
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
    (void)button; return 0;
}
bool m5u_button_was_release_for(int button, uint32_t ms) {
    (void)button; (void)ms; return false;
}
bool m5u_button_pressed_for(int button, uint32_t ms) {
    (void)button; (void)ms; return false;
}
bool m5u_button_released_for(int button, uint32_t ms) {
    (void)button; (void)ms; return true;
}
void m5u_button_set_debounce_thresh(int button, uint32_t ms) {
    (void)button; (void)ms;
}
void m5u_button_set_hold_thresh(int button, uint32_t ms) {
    (void)button; (void)ms;
}
void m5u_button_set_raw_state(int button, uint32_t msec, bool press) {
    (void)button; (void)msec; (void)press;
}
void m5u_button_set_state(int button, uint32_t msec, uint8_t state) {
    (void)button; (void)msec; (void)state;
}
uint8_t m5u_button_get_state(int button) {
    (void)button; return 0;
}
uint32_t m5u_button_last_change(int button) {
    (void)button; return 0;
}
uint32_t m5u_button_get_debounce_thresh(int button) {
    (void)button; return 10;
}
uint32_t m5u_button_get_hold_thresh(int button) {
    (void)button; return 500;
}
uint32_t m5u_button_get_update_msec(int button) {
    (void)button; return 0;
}

bool m5u_mic_is_enabled(void) {
    return false;
}

bool m5u_mic_is_recording(void) {
    return false;
}

size_t m5u_mic_recording_state(void) {
    return 0;
}

void m5u_mic_end(void) {
}

bool m5u_mic_record_i16_at(int16_t* buffer, size_t samples, uint32_t sample_rate_hz) {
    (void)buffer; (void)samples; (void)sample_rate_hz; return false;
}

bool m5u_mic_record_i16_ex(int16_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo) {
    (void)buffer; (void)samples; (void)sample_rate_hz; (void)stereo; return false;
}

bool m5u_mic_record_u8_ex(uint8_t* buffer, size_t samples, uint32_t sample_rate_hz, bool stereo) {
    (void)buffer; (void)samples; (void)sample_rate_hz; (void)stereo; return false;
}

bool m5u_audio_capture_begin(uint32_t sample_rate_hz, size_t dma_frame_num, size_t dma_desc_num, uint8_t* out_channels) {
    (void)sample_rate_hz; (void)dma_frame_num; (void)dma_desc_num;
    if (out_channels) { *out_channels = 0; }
    return false;
}

size_t m5u_audio_capture_read_i16(int16_t* buffer, size_t samples, uint32_t timeout_ms) {
    (void)buffer; (void)samples; (void)timeout_ms; return 0;
}

void m5u_audio_capture_end(void) {
}

void m5u_mic_set_sample_rate(uint32_t sample_rate_hz) {
    (void)sample_rate_hz;
}

bool m5u_mic_get_config(m5u_mic_config_t* out) {
    if (!out) {
        return false;
    }
    out->pin_data_in = -1;
    out->pin_bck = -1;
    out->pin_mck = -1;
    out->pin_ws = -1;
    out->sample_rate = 16000;
    out->left_channel = 0;
    out->stereo = 0;
    out->over_sampling = 2;
    out->magnification = 16;
    out->noise_filter_level = 0;
    out->use_adc = 0;
    out->dma_buf_len = 128;
    out->dma_buf_count = 8;
    out->task_priority = 2;
    out->task_pinned_core = 255;
    out->i2s_port = 0;
    return true;
}

bool m5u_mic_set_config(const m5u_mic_config_t* config) {
    return config != nullptr;
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

bool m5u_speaker_get_config(m5u_speaker_config_t* out) {
    if (!out) {
        return false;
    }
    out->pin_data_out = -1;
    out->pin_bck = -1;
    out->pin_mck = -1;
    out->pin_ws = -1;
    out->sample_rate = 48000;
    out->stereo = 0;
    out->buzzer = 0;
    out->use_dac = 0;
    out->dac_zero_level = 0;
    out->magnification = 16;
    out->dma_buf_len = 256;
    out->dma_buf_count = 8;
    out->task_priority = 2;
    out->task_pinned_core = 255;
    out->i2s_port = 0;
    return true;
}

bool m5u_speaker_set_config(const m5u_speaker_config_t* config) {
    return config != nullptr;
}

bool m5u_speaker_tone_ex(float frequency_hz, uint32_t duration_ms, int channel) {
    (void)frequency_hz; (void)duration_ms; (void)channel; return false;
}

bool m5u_speaker_tone_options(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound) {
    (void)frequency_hz; (void)duration_ms; (void)channel; (void)stop_current_sound; return false;
}

bool m5u_speaker_tone_full(float frequency_hz, uint32_t duration_ms, int channel, bool stop_current_sound, const uint8_t* raw_data, size_t len, bool stereo) {
    (void)frequency_hz; (void)duration_ms; (void)channel; (void)stop_current_sound; (void)raw_data; (void)len; (void)stereo; return false;
}

bool m5u_speaker_play_u8(const uint8_t* samples, size_t len, uint32_t sample_rate_hz) {
    (void)samples; (void)len; (void)sample_rate_hz; return false;
}

bool m5u_speaker_play_u8_ex(const uint8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    (void)samples; (void)len; (void)sample_rate_hz; (void)stereo; (void)repeat; (void)channel; (void)stop_current_sound; return false;
}

bool m5u_speaker_play_i8_ex(const int8_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    (void)samples; (void)len; (void)sample_rate_hz; (void)stereo; (void)repeat; (void)channel; (void)stop_current_sound; return false;
}

bool m5u_speaker_play_i16_ex(const int16_t* samples, size_t len, uint32_t sample_rate_hz, bool stereo, uint32_t repeat, int channel, bool stop_current_sound) {
    (void)samples; (void)len; (void)sample_rate_hz; (void)stereo; (void)repeat; (void)channel; (void)stop_current_sound; return false;
}

bool m5u_speaker_play_wav(const uint8_t* data, size_t len) {
    (void)data; (void)len; return false;
}

bool m5u_speaker_play_wav_ex(const uint8_t* data, size_t len, uint32_t repeat, int channel, bool stop_current_sound) {
    (void)data; (void)len; (void)repeat; (void)channel; (void)stop_current_sound; return false;
}

bool m5u_speaker_is_playing(int channel) {
    (void)channel; return false;
}

size_t m5u_speaker_playing_channels(void) {
    return 0;
}

size_t m5u_speaker_channel_playing_state(int channel) {
    (void)channel; return 0;
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

int m5u_imu_update_mask(void) {
    return 0;
}

bool m5u_imu_sleep(void) {
    return true;
}

void m5u_imu_set_clock(uint32_t freq) {
    (void)freq;
}

bool m5u_imu_set_axis_order(int axis0, int axis1, int axis2) {
    (void)axis0; (void)axis1; (void)axis2; return true;
}

bool m5u_imu_set_axis_order_right_handed(int axis0, int axis1) {
    (void)axis0; (void)axis1; return true;
}

bool m5u_imu_set_axis_order_left_handed(int axis0, int axis1) {
    (void)axis0; (void)axis1; return true;
}

bool m5u_imu_set_int_pin_active_logic(bool level) {
    (void)level; return true;
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

void m5u_imu_set_calibration_strength(uint8_t accel, uint8_t gyro, uint8_t mag) {
    (void)accel; (void)gyro; (void)mag;
}

void m5u_imu_clear_offset_data(void) {
}

void m5u_imu_set_offset_data(size_t index, int32_t value) {
    (void)index; (void)value;
}

int32_t m5u_imu_get_offset_data_i32(size_t index) {
    (void)index; return 0;
}

int16_t m5u_imu_get_raw_data(size_t index) {
    (void)index; return 0;
}

int m5u_imu_device_begin(int kind) {
    (void)kind; return 0;
}

bool m5u_imu_device_get_raw_data(int kind, m5u_imu_raw_data_t* out) {
    (void)kind; (void)out; return false;
}

bool m5u_imu_device_get_convert_param(int kind, m5u_imu_convert_param_t* out) {
    (void)kind;
    if (!out) {
        return false;
    }
    out->accel_res = 8.0f / 32768.0f;
    out->gyro_res = 2000.0f / 32768.0f;
    out->mag_res = 10.0f * 4912.0f / 32768.0f;
    out->temp_res = 1.0f;
    out->temp_offset = 0.0f;
    return true;
}

bool m5u_imu_device_get_temp_adc(int kind, int16_t* adc) {
    (void)kind; (void)adc; return false;
}

bool m5u_imu_device_sleep(int kind) {
    (void)kind; return false;
}

bool m5u_imu_device_set_int_pin_active_logic(int kind, bool level) {
    (void)kind; (void)level; return false;
}

int m5u_imu_device_who_am_i(int kind) {
    (void)kind; return -1;
}

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
    (void)index;
    if (out) {
        *out = {};
    }
    return false;
}

bool m5u_rtc_is_enabled(void) {
    return false;
}

bool m5u_power_axp192_begin(void) { return false; }
int m5u_power_axp192_get_battery_level(void) { return -1; }
bool m5u_power_axp192_set_battery_charge(bool enable) { (void)enable; return false; }
bool m5u_power_axp192_set_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_axp192_set_charge_voltage(uint16_t max_mv) { (void)max_mv; return false; }
bool m5u_power_axp192_is_charging(void) { return false; }
bool m5u_power_axp192_set_dcdc(uint8_t channel, int voltage_mv) { (void)channel; (void)voltage_mv; return false; }
bool m5u_power_axp192_set_ldo(uint8_t channel, int voltage_mv) { (void)channel; (void)voltage_mv; return false; }
bool m5u_power_axp192_set_gpio(uint8_t gpio_num, bool state) { (void)gpio_num; (void)state; return false; }
bool m5u_power_axp192_power_off(void) { return false; }
bool m5u_power_axp192_set_adc_state(bool enable) { (void)enable; return false; }
bool m5u_power_axp192_set_adc_rate(uint8_t rate) { (void)rate; return false; }
bool m5u_power_axp192_set_exten(bool enable) { (void)enable; return false; }
bool m5u_power_axp192_set_backup(bool enable) { (void)enable; return false; }
bool m5u_power_axp192_is_acin(void) { return false; }
bool m5u_power_axp192_is_vbus(void) { return false; }
bool m5u_power_axp192_get_bat_state(void) { return false; }
bool m5u_power_axp192_get_exten(void) { return false; }
float m5u_power_axp192_get_battery_voltage_v(void) { return 0.0f; }
float m5u_power_axp192_get_battery_discharge_current_ma(void) { return 0.0f; }
float m5u_power_axp192_get_battery_charge_current_ma(void) { return 0.0f; }
float m5u_power_axp192_get_battery_power_mw(void) { return 0.0f; }
float m5u_power_axp192_get_acin_voltage_v(void) { return 0.0f; }
float m5u_power_axp192_get_acin_current_ma(void) { return 0.0f; }
float m5u_power_axp192_get_vbus_voltage_v(void) { return 0.0f; }
float m5u_power_axp192_get_vbus_current_ma(void) { return 0.0f; }
float m5u_power_axp192_get_aps_voltage_v(void) { return 0.0f; }
float m5u_power_axp192_get_internal_temperature_c(void) { return 0.0f; }
uint8_t m5u_power_axp192_get_pek_press(void) { return 0; }
bool m5u_power_aw32001_begin(void) { return false; }
bool m5u_power_aw32001_set_battery_charge(bool enable) { (void)enable; return false; }
bool m5u_power_aw32001_set_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_aw32001_set_charge_voltage(uint16_t max_mv) { (void)max_mv; return false; }
bool m5u_power_aw32001_is_charging(void) { return false; }
uint16_t m5u_power_aw32001_get_charge_current(void) { return 0; }
uint16_t m5u_power_aw32001_get_charge_voltage(void) { return 0; }
int m5u_power_aw32001_get_charge_status(void) { return -1; }
bool m5u_power_bq27220_begin(void) { return false; }
int16_t m5u_power_bq27220_get_current_ma(void) { return 0; }
int16_t m5u_power_bq27220_get_voltage_mv(void) { return 0; }
float m5u_power_bq27220_get_current_a(void) { return 0.0f; }
float m5u_power_bq27220_get_voltage_v(void) { return 0.0f; }
bool m5u_power_ina226_begin(void) { return false; }
bool m5u_power_ina226_config(const m5u_power_ina226_config_t* config) { (void)config; return false; }
float m5u_power_ina226_get_bus_voltage_v(void) { return 0.0f; }
float m5u_power_ina226_get_shunt_voltage_v(void) { return 0.0f; }
float m5u_power_ina226_get_shunt_current_a(void) { return 0.0f; }
float m5u_power_ina226_get_power_w(void) { return 0.0f; }
bool m5u_power_ina3221_begin(size_t index) { (void)index; return false; }
float m5u_power_ina3221_get_bus_voltage_v(size_t index, uint8_t channel) { (void)index; (void)channel; return 0.0f; }
float m5u_power_ina3221_get_shunt_voltage_v(size_t index, uint8_t channel) { (void)index; (void)channel; return 0.0f; }
float m5u_power_ina3221_get_current_a(size_t index, uint8_t channel) { (void)index; (void)channel; return 0.0f; }
int32_t m5u_power_ina3221_get_bus_voltage_mv(size_t index, uint8_t channel) { (void)index; (void)channel; return 0; }
int32_t m5u_power_ina3221_get_shunt_voltage_mv(size_t index, uint8_t channel) { (void)index; (void)channel; return 0; }
bool m5u_power_ina3221_set_shunt_res(size_t index, uint8_t channel, uint32_t res) { (void)index; (void)channel; (void)res; return false; }
bool m5u_power_ip5306_begin(void) { return false; }
int m5u_power_ip5306_get_battery_level(void) { return -1; }
bool m5u_power_ip5306_set_battery_charge(bool enable) { (void)enable; return false; }
bool m5u_power_ip5306_set_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_ip5306_set_charge_voltage(uint16_t max_mv) { (void)max_mv; return false; }
bool m5u_power_ip5306_is_charging(void) { return false; }
bool m5u_power_ip5306_set_power_boost_keep_on(bool enable) { (void)enable; return false; }
bool m5u_power_py32pmic_begin(void) { return false; }
bool m5u_power_py32pmic_set_ext_output(bool enable) { (void)enable; return false; }
bool m5u_power_py32pmic_set_battery_charge(bool enable) { (void)enable; return false; }
bool m5u_power_py32pmic_set_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_py32pmic_set_charge_voltage(uint16_t max_mv) { (void)max_mv; return false; }
bool m5u_power_py32pmic_is_charging(void) { return false; }
uint16_t m5u_power_py32pmic_get_charge_current(void) { return 0; }
uint16_t m5u_power_py32pmic_get_charge_voltage(void) { return 0; }
uint8_t m5u_power_py32pmic_get_pek_press(void) { return 0; }
bool m5u_power_py32pmic_power_off(void) { return false; }
bool m5u_power_axp2101_begin(void) { return false; }
int m5u_power_axp2101_get_battery_level(void) { return -1; }
bool m5u_power_axp2101_set_battery_charge(bool enable) { (void)enable; return false; }
bool m5u_power_axp2101_set_pre_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_axp2101_set_charge_current(uint16_t max_ma) { (void)max_ma; return false; }
bool m5u_power_axp2101_set_charge_voltage(uint16_t max_mv) { (void)max_mv; return false; }
int m5u_power_axp2101_get_charge_status(void) { return -2; }
bool m5u_power_axp2101_is_charging(void) { return false; }
bool m5u_power_axp2101_set_ldo(int kind, int channel, int voltage_mv) { (void)kind; (void)channel; (void)voltage_mv; return false; }
bool m5u_power_axp2101_get_ldo_enabled(int kind, int channel) { (void)kind; (void)channel; return false; }
bool m5u_power_axp2101_power_off(void) { return false; }
bool m5u_power_axp2101_set_adc_state(bool enable) { (void)enable; return false; }
bool m5u_power_axp2101_set_adc_rate(uint8_t rate) { (void)rate; return false; }
bool m5u_power_axp2101_set_backup(bool enable) { (void)enable; return false; }
bool m5u_power_axp2101_is_acin(void) { return false; }
bool m5u_power_axp2101_is_vbus(void) { return false; }
bool m5u_power_axp2101_get_bat_state(void) { return false; }
float m5u_power_axp2101_get_battery_voltage_v(void) { return 0.0f; }
float m5u_power_axp2101_get_battery_discharge_current_ma(void) { return 0.0f; }
float m5u_power_axp2101_get_battery_charge_current_ma(void) { return 0.0f; }
float m5u_power_axp2101_get_battery_power_mw(void) { return 0.0f; }
float m5u_power_axp2101_get_acin_voltage_v(void) { return 0.0f; }
float m5u_power_axp2101_get_acin_current_ma(void) { return 0.0f; }
float m5u_power_axp2101_get_vbus_voltage_v(void) { return 0.0f; }
float m5u_power_axp2101_get_vbus_current_ma(void) { return 0.0f; }
float m5u_power_axp2101_get_ts_voltage_v(void) { return 0.0f; }
float m5u_power_axp2101_get_aps_voltage_v(void) { return 0.0f; }
float m5u_power_axp2101_get_internal_temperature_c(void) { return 0.0f; }
uint8_t m5u_power_axp2101_get_pek_press(void) { return 0; }
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

void m5u_led_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
    (void)colors; (void)index; (void)length;
}

int m5u_led_get_type(size_t index) {
    (void)index; return 0;
}

bool m5u_led_is_enabled(void) {
    return false;
}

bool m5u_led_power_hub_begin(void) {
    return false;
}

size_t m5u_led_power_hub_count(void) {
    return 0;
}

void m5u_led_power_hub_set_brightness(uint8_t brightness) {
    (void)brightness;
}

void m5u_led_power_hub_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
    (void)index; (void)r; (void)g; (void)b;
}

void m5u_led_power_hub_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
    (void)colors; (void)index; (void)length;
}

void m5u_led_power_hub_display(void) {
}

int m5u_led_power_hub_get_type(size_t index) {
    (void)index; return 0;
}

bool m5u_led_strip_set_config(const m5u_led_strip_config_t* config) {
    (void)config; return false;
}

bool m5u_led_strip_set_rmt_bus_config(const m5u_led_strip_rmt_config_t* config) {
    (void)config; return false;
}

bool m5u_led_strip_begin(void) {
    return false;
}

size_t m5u_led_strip_count(void) {
    return 0;
}

void m5u_led_strip_set_brightness(uint8_t brightness) {
    (void)brightness;
}

void m5u_led_strip_set_color_rgb(size_t index, uint8_t r, uint8_t g, uint8_t b) {
    (void)index; (void)r; (void)g; (void)b;
}

void m5u_led_strip_set_colors_rgb(const m5u_led_color_t* colors, size_t index, size_t length) {
    (void)colors; (void)index; (void)length;
}

void m5u_led_strip_display(void) {
}

int m5u_led_strip_get_type(size_t index) {
    (void)index; return 0;
}

void m5u_log_print(const char* text) {
    (void)text;
}

void m5u_log_level(int level, const char* text) {
    (void)level; (void)text;
}

void m5u_log_dump(const void* addr, uint32_t len, int level) {
    (void)addr; (void)len; (void)level;
}

const char* m5u_log_path_to_file_name(const char* path) {
    if (!path) {
        return nullptr;
    }
    const char* file = path;
    for (const char* p = path; *p; ++p) {
        if (*p == '/' || *p == '\\') {
            file = p + 1;
        }
    }
    return file;
}

bool m5u_log_set_callback(m5u_log_callback_t callback, void* user_data) {
    (void)callback; (void)user_data; return true;
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

bool m5u_canvas_create(int /*w*/, int /*h*/) { return false; }
void m5u_canvas_push(int /*x*/, int /*y*/) {}
void m5u_canvas_delete(void) {}
void m5u_canvas_fill_screen(uint16_t /*c*/) {}
void m5u_canvas_fill_smooth_circle(int /*x*/, int /*y*/, int /*r*/, uint16_t /*c*/) {}
void m5u_canvas_draw_line(int /*x0*/, int /*y0*/, int /*x1*/, int /*y1*/, uint16_t /*c*/) {}
void m5u_canvas_draw_circle(int /*x*/, int /*y*/, int /*r*/, uint16_t /*c*/) {}
void m5u_canvas_fill_circle(int /*x*/, int /*y*/, int /*r*/, uint16_t /*c*/) {}
void m5u_canvas_fill_rect(int /*x*/, int /*y*/, int /*w*/, int /*h*/, uint16_t /*c*/) {}
void m5u_canvas_fill_triangle(int /*x0*/, int /*y0*/, int /*x1*/, int /*y1*/, int /*x2*/, int /*y2*/, uint16_t /*c*/) {}
void m5u_canvas_fill_smooth_round_rect(int /*x*/, int /*y*/, int /*w*/, int /*h*/, int /*r*/, uint16_t /*c*/) {}
void m5u_canvas_fill_arc(int /*x*/, int /*y*/, int /*r0*/, int /*r1*/, float /*a0*/, float /*a1*/, uint16_t /*c*/) {}
void m5u_canvas_fill_ellipse(int /*x*/, int /*y*/, int /*rx*/, int /*ry*/, uint16_t /*c*/) {}
void m5u_canvas_draw_ellipse(int /*x*/, int /*y*/, int /*rx*/, int /*ry*/, uint16_t /*c*/) {}
void m5u_canvas_set_text_size(int /*size*/) {}
void m5u_canvas_set_text_color(uint16_t /*fg*/, uint16_t /*bg*/) {}
void m5u_canvas_set_text_datum(int /*datum*/) {}
int m5u_canvas_text_width(const char* /*text*/) { return 0; }
int m5u_canvas_draw_string(const char* /*text*/, int /*x*/, int /*y*/) { return 0; }

bool m5u_cardputer_begin(bool /*enable_keyboard*/) { return false; }
bool m5u_cardputer_begin_with_config(const m5u_config_t* /*config*/, bool /*enable_keyboard*/) { return false; }
void m5u_cardputer_update(void) {}
void m5u_cardputer_keyboard_begin(void) {}
bool m5u_cardputer_keyboard_is_pressed(void) { return false; }
uint8_t m5u_cardputer_keyboard_pressed_count(void) { return 0; }
bool m5u_cardputer_keyboard_is_change(void) { return false; }
bool m5u_cardputer_keyboard_is_key_pressed(uint8_t /*key*/) { return false; }
uint8_t m5u_cardputer_keyboard_get_key(uint8_t /*x*/, uint8_t /*y*/) { return 0; }
bool m5u_cardputer_keyboard_get_key_value(uint8_t /*x*/, uint8_t /*y*/, m5u_cardputer_key_value_t* out) {
    if (!out) { return false; }
    out->first = 0;
    out->second = 0;
    return false;
}
bool m5u_cardputer_keyboard_get_state(m5u_cardputer_keyboard_state_t* out) {
    if (!out) { return false; }
    *out = {};
    return true;
}
bool m5u_cardputer_keyboard_capslocked(void) { return false; }
void m5u_cardputer_keyboard_set_capslocked(bool /*locked*/) {}
bool m5u_cardputer_ir_begin(int /*pin*/) { return false; }
bool m5u_cardputer_ir_send_nec(uint16_t /*address*/, uint8_t /*command*/, uint8_t /*repeats*/) { return false; }
bool m5u_cardputer_grove_i2c_begin(int /*sda*/, int /*scl*/, uint32_t /*frequency_hz*/) { return false; }
void m5u_cardputer_grove_i2c_end(void) {}
bool m5u_cardputer_grove_i2c_probe(uint8_t /*address*/) { return false; }
bool m5u_cardputer_grove_i2c_write(uint8_t /*address*/, const uint8_t* /*data*/, size_t /*len*/) { return false; }
size_t m5u_cardputer_grove_i2c_read(uint8_t /*address*/, uint8_t* /*data*/, size_t /*len*/) { return 0; }
bool m5u_cardputer_grove_gpio_pin_mode(int /*pin*/, int /*mode*/) { return false; }
bool m5u_cardputer_grove_gpio_write(int /*pin*/, bool /*high*/) { return false; }
int m5u_cardputer_grove_gpio_read(int /*pin*/) { return -1; }
bool m5u_cardputer_grove_uart_begin(int /*rx*/, int /*tx*/, uint32_t /*baud*/) { return false; }
void m5u_cardputer_grove_uart_end(void) {}
size_t m5u_cardputer_grove_uart_available(void) { return 0; }
size_t m5u_cardputer_grove_uart_read(uint8_t* /*data*/, size_t /*len*/) { return 0; }
size_t m5u_cardputer_grove_uart_write(const uint8_t* /*data*/, size_t /*len*/) { return 0; }
void m5u_cardputer_grove_uart_flush(void) {}

bool m5u_servo_init(int /*tx_pin*/, int /*rx_pin*/, int /*baud_rate*/) { return false; }
bool m5u_servo_write_raw_pos(uint8_t /*id*/, uint16_t /*raw_pos*/, uint16_t /*time_ms*/, uint16_t /*speed*/) { return false; }
int  m5u_servo_read_raw_pos(uint8_t /*id*/) { return -1; }
bool m5u_servo_enable_torque(uint8_t /*id*/, bool /*enable*/) { return false; }
void m5u_servo_deinit(void) {}

bool m5u_nvs_read_i32(const char* /*ns*/, const char* /*key*/, int32_t* /*out_val*/) { return false; }
bool m5u_nvs_write_i32(const char* /*ns*/, const char* /*key*/, int32_t /*val*/) { return false; }

} // extern "C"
