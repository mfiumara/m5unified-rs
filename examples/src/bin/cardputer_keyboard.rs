use m5unified::{colors, Cardputer, DisplayFont, Point, Rect, Size};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer keyboard")?;

    if let Some(mut canvas) = cardputer.canvas() {
        canvas.set_text_size(0.5);
        canvas.set_text_color(colors::GREEN, colors::BLACK);
        canvas.set_text_scroll(true);
        canvas.set_text_datum(m5unified::TextDatum::MiddleCenter);
        let _ = canvas.try_set_font(DisplayFont::FreeSerifBoldItalic18pt7b);
        let sprite = Rect {
            x: 0,
            y: 0,
            w: cardputer.display.width() - 8,
            h: 36,
        };
        let _ = canvas.try_create_sprite(Size {
            w: sprite.w,
            h: sprite.h,
        });
        canvas.fill_round_rect(sprite, 4, colors::DARK_GREY);
        canvas.draw_round_rect(sprite, 4, colors::GREEN);
        canvas.progress_bar(
            Rect {
                x: 8,
                y: sprite.h - 8,
                w: sprite.w - 16,
                h: 3,
            },
            35,
        );
        canvas.draw_center_string("typed keys appear here", sprite.w / 2, 18)?;
        canvas.fill_circle(Point { x: 8, y: 8 }, 3, colors::GREEN);
        canvas.fill_triangle(
            Point { x: 14, y: 10 },
            Point { x: 20, y: 6 },
            Point { x: 20, y: 14 },
            colors::GREEN,
        );
        let badge = [colors::GREEN, colors::BLACK, colors::BLACK, colors::GREEN];
        let _ = canvas.try_push_image_rgb565(
            Rect {
                x: sprite.w - 14,
                y: 4,
                w: 2,
                h: 2,
            },
            &badge,
        );
        canvas.draw_number(cardputer.keyboard.pressed_count() as i32, sprite.w - 28, 18);
        canvas.draw_pixel(
            Point {
                x: sprite.w - 2,
                y: 2,
            },
            colors::WHITE,
        );
        canvas.push_sprite(Point { x: 4, y: 32 });
    }

    let state = cardputer.keyboard.try_state().unwrap_or_default();
    cardputer.display.set_cursor(4, 72);
    cardputer
        .display
        .println(&format!("pressed: {}", cardputer.keyboard.is_pressed()))?;
    cardputer.display.set_cursor(4, 96);
    cardputer
        .display
        .println(&format!("changed: {}", cardputer.keyboard.is_change()))?;
    cardputer.display.set_cursor(4, 120);
    cardputer.display.println(&format!(
        "word: {}",
        cardputer.keyboard.try_word_lossy().unwrap_or_default()
    ))?;
    cardputer.display.set_cursor(4, 144);
    cardputer
        .display
        .println(&format!("first: {:?}", state.first_word_char()))?;
    cardputer.display.set_cursor(4, 168);
    cardputer
        .display
        .println(&format!("empty: {}", state.is_empty()))?;

    Ok(())
}
