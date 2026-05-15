#pragma once

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

bool m5u_begin(void);
void m5u_update(void);

void m5u_display_fill_screen(uint16_t color);
void m5u_display_set_cursor(int x, int y);
void m5u_display_print(const char* text);

bool m5u_btn_a_is_pressed(void);
bool m5u_btn_b_was_pressed(void);

bool m5u_mic_begin(void);
bool m5u_mic_record_i16(int16_t* buffer, size_t samples);

int m5u_battery_level(void);

#ifdef __cplusplus
}
#endif
