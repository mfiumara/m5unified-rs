use m5unified::{colors, M5Unified, Rect};
use m5unified_examples::{banner, ExampleResult};

fn main() -> ExampleResult {
    let mut m5 = M5Unified::begin()?;
    banner(&mut m5, "Basic/Displays")?;
    let primary_selected = m5.try_set_primary_display(0).is_ok();
    let w = m5.display.width();
    let h = m5.display.height();
    m5.display.draw_rect(0, 0, w, h, colors::WHITE);
    m5.display.draw_line(0, 0, w - 1, h - 1, colors::RED);
    m5.display.draw_line(w - 1, 0, 0, h - 1, colors::GREEN);
    m5.display.fill_circle(w / 2, h / 2, 24, colors::BLUE);
    m5.display.set_cursor(8, h / 2 + 36);
    m5.display.println("Shapes through safe Rust API")?;
    m5.display
        .println(&format!("primary display 0: {primary_selected}"))?;
    for index in 0..m5.display_count() {
        if let Some(mut display) = m5.display(index) {
            let width = display.width();
            let height = display.height();
            display.transaction(|display| {
                display.set_rotation(display.rotation());
                display.set_color_depth(display.color_depth());
                display.set_brightness(display.brightness());
                display.set_base_color(colors::BLACK);
                display.set_swap_bytes(false);
                display.set_clip_rect(Rect {
                    x: 0,
                    y: 0,
                    w: width,
                    h: height,
                });
                display.set_text_size((height / 120).max(1));
                let _ = display.try_set_font(m5unified::DisplayFont::Font0);
                display.set_text_color(colors::WHITE, colors::BLACK);
                display.set_text_datum(m5unified::TextDatum::TopCenter);
                display.set_text_padding(width as u32);
                let label = format!("Display {index}");
                let _ = display.draw_center_string(&label, width / 2, 4);
                display.set_cursor(4, 24 + i32::from(display.text_size_y()) * 8);
                let _ = display.println(&format!(
                    "text {} font {}x{}",
                    display.text_width(&label).unwrap_or(0),
                    display.font_width(),
                    display.font_height_for(m5unified::DisplayFont::Font0)
                ));
                let _ = display.try_show_font(0);
                display.set_text_scroll(true);
                display.set_scroll_rect(
                    Rect {
                        x: 4,
                        y: 4,
                        w: (width - 8).max(1),
                        h: (height / 3).max(1),
                    },
                    colors::BLACK,
                );
                display.draw_rect(
                    Rect {
                        x: 0,
                        y: 0,
                        w: width,
                        h: height,
                    },
                    colors::GREEN,
                );
                display.draw_round_rect(
                    Rect {
                        x: 3,
                        y: 3,
                        w: (width - 6).max(1),
                        h: (height - 6).max(1),
                    },
                    6,
                    colors::WHITE,
                );
                display.progress_bar(
                    Rect {
                        x: 8,
                        y: (height - 12).max(0),
                        w: (width - 16).max(1),
                        h: 6,
                    },
                    50,
                );
                let swatch = [colors::RED, colors::GREEN, colors::BLUE, colors::WHITE];
                let marker = display.color888(255, 255, 0);
                display.set_color(marker);
                let _ = display.try_push_image_rgb565(
                    Rect {
                        x: (width - 2).max(0),
                        y: (height - 2).max(0),
                        w: 2,
                        h: 2,
                    },
                    &swatch,
                );
                display.draw_line(0, height - 1, width - 1, 0, colors::YELLOW);
                display.draw_pixel(width / 2, height / 2, colors::RED);
                display.draw_fast_hline(8, height / 2, (width - 16).max(1), colors::BLUE);
                display.draw_fast_vline(width / 2, 8, (height - 16).max(1), colors::RED);
                display.scroll(0, 0);
                display.clear_scroll_rect();
                display.clear_clip_rect();
            });
            display.display();
        }
    }
    Ok(())
}
