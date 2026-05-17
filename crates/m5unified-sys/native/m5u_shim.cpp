#include "m5u_shim.h"

#include <M5Unified.h>
#include <string>

static std::string s_m5u_log_suffixes[3];

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

void m5u_set_log_display_index(size_t index) {
    M5.setLogDisplayIndex(index);
}

void m5u_set_log_display_type(int kind) {
    M5.setLogDisplayType((m5gfx::board_t)kind);
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

bool m5u_touch_get(int index, int* x, int* y) {
    auto detail = M5.Touch.getDetail(index);
    if (x) { *x = detail.x; }
    if (y) { *y = detail.y; }
    return detail.isPressed();
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
    M5.Rtc.setSystemTimeFromRtc();
}

int m5u_battery_level(void) {
    return M5.Power.getBatteryLevel();
}

int m5u_battery_voltage_mv(void) {
    return M5.Power.getBatteryVoltage();
}

bool m5u_power_is_charging(void) {
    return M5.Power.isCharging();
}

void m5u_log_println(const char* text) {
    M5_LOGI("%s", text);
}

bool m5u_sd_begin(void) {
    // SD support needs an explicit ESP-IDF/Arduino SD component wiring step.
    // Keep the shim target-buildable for display/button firmware until that
    // component is added instead of referencing Arduino globals (SD, SPI) here.
    return false;
}


static bool m5u_button_state(int button, int query) {
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

int m5u_display_width_at(int index) {
    return M5.Displays(index).width();
}

int m5u_display_height_at(int index) {
    return M5.Displays(index).height();
}

void m5u_display_print_at(int index, const char* text) {
    M5.Displays(index).print(text);
}

void m5u_display_fill_circle_at(int index, int x, int y, int r, uint16_t color) {
    M5.Displays(index).fillCircle(x, y, r, color);
}

bool m5u_button_is_pressed(int button) { return m5u_button_state(button, 0); }
bool m5u_button_was_pressed(int button) { return m5u_button_state(button, 1); }
bool m5u_button_was_released(int button) { return m5u_button_state(button, 2); }
bool m5u_button_was_clicked(int button) { return m5u_button_state(button, 3); }
bool m5u_button_was_hold(int button) { return m5u_button_state(button, 4); }
bool m5u_button_is_holding(int button) { return m5u_button_state(button, 5); }
bool m5u_button_was_decide_click_count(int button) { return m5u_button_state(button, 6); }
int m5u_button_get_click_count(int button) {
    switch (button) {
    case 0: return M5.BtnA.getClickCount();
    case 1: return M5.BtnB.getClickCount();
    case 2: return M5.BtnC.getClickCount();
    case 3: return M5.BtnPWR.getClickCount();
    case 4: return M5.BtnEXT.getClickCount();
    default: return 0;
    }
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

bool m5u_speaker_is_enabled(void) {
    return M5.Speaker.isEnabled();
}

void m5u_speaker_end(void) {
    M5.Speaker.end();
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

bool m5u_speaker_is_playing(int channel) {
    return channel < 0 ? M5.Speaker.isPlaying() : M5.Speaker.isPlaying(channel);
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

bool m5u_touch_get_detail(int index, m5u_touch_detail_t* out) {
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

bool m5u_led_is_enabled(void) {
    return M5.Led.isEnabled();
}

void m5u_log_print(const char* text) {
    M5.Log.print(text);
}

void m5u_log_level(int level, const char* text) {
    M5.Log((esp_log_level_t)level, "%s", text);
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
