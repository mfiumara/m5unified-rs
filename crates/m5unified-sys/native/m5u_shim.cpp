#include "m5u_shim.h"

// TODO: include M5Unified once ESP-IDF component build integration is wired up.
// #include <M5Unified.h>

extern "C" {

bool m5u_begin(void) {
    // auto cfg = M5.config();
    // M5.begin(cfg);
    // return true;
    return false;
}

void m5u_update(void) {
    // M5.update();
}

void m5u_display_fill_screen(uint16_t color) {
    (void)color;
    // M5.Display.fillScreen(color);
}

void m5u_display_set_cursor(int x, int y) {
    (void)x;
    (void)y;
    // M5.Display.setCursor(x, y);
}

void m5u_display_print(const char* text) {
    (void)text;
    // M5.Display.print(text);
}

bool m5u_btn_a_is_pressed(void) {
    // return M5.BtnA.isPressed();
    return false;
}

bool m5u_btn_b_was_pressed(void) {
    // return M5.BtnB.wasPressed();
    return false;
}

bool m5u_mic_begin(void) {
    // return M5.Mic.begin();
    return false;
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
    (void)buffer;
    (void)samples;
    // return M5.Mic.record(buffer, samples);
    return false;
}

int m5u_battery_level(void) {
    // return M5.Power.getBatteryLevel();
    return -1;
}

}
