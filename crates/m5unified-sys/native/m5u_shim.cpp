#include "m5u_shim.h"

#if defined(__has_include)
#if __has_include(<M5Unified.h>)
#define M5U_HAS_REAL_M5UNIFIED 1
#include <M5Unified.h>
#endif
#endif

#if defined(M5U_REQUIRE_REAL_M5UNIFIED) && !defined(M5U_HAS_REAL_M5UNIFIED)
#error "M5UNIFIED_SYS_REQUIRE_REAL is set, but <M5Unified.h> was not found."
#endif

extern "C" {

bool m5u_begin(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    auto cfg = M5.config();
    M5.begin(cfg);
    return true;
#else
    return false;
#endif
}

void m5u_update(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    M5.update();
#endif
}

void m5u_display_fill_screen(uint16_t color) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    M5.Display.fillScreen(color);
#else
    (void)color;
#endif
}

void m5u_display_set_cursor(int x, int y) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    M5.Display.setCursor(x, y);
#else
    (void)x;
    (void)y;
#endif
}

void m5u_display_print(const char* text) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    M5.Display.print(text);
#else
    (void)text;
#endif
}

bool m5u_btn_a_is_pressed(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    return M5.BtnA.isPressed();
#else
    return false;
#endif
}

bool m5u_btn_b_was_pressed(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    return M5.BtnB.wasPressed();
#else
    return false;
#endif
}

bool m5u_mic_begin(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    return M5.Mic.begin();
#else
    return false;
#endif
}

bool m5u_mic_record_i16(int16_t* buffer, size_t samples) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    return M5.Mic.record(buffer, samples);
#else
    (void)buffer;
    (void)samples;
    return false;
#endif
}

int m5u_battery_level(void) {
#if defined(M5U_HAS_REAL_M5UNIFIED)
    return M5.Power.getBatteryLevel();
#else
    return -1;
#endif
}

}
