use m5unified::{colors, Cardputer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cardputer = Cardputer::begin()?;

    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::WHITE, colors::BLACK);
    cardputer.display.set_text_size(2);
    cardputer.display.set_cursor(4, 8);
    cardputer.display.println("Cardputer keyboard")?;

    cardputer
        .display
        .fill_round_rect(4, 32, 232, 36, 4, colors::DARK_GREY);
    cardputer
        .display
        .draw_round_rect(4, 32, 232, 36, 4, colors::GREEN);
    cardputer.display.progress_bar(
        m5unified::Rect {
            x: 12,
            y: 60,
            w: 216,
            h: 3,
        },
        35,
    );
    cardputer
        .display
        .draw_center_string("typed keys appear here", 120, 46)?;
    cardputer
        .display
        .draw_number(cardputer.keyboard.pressed_count() as i32, 208, 46);

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
