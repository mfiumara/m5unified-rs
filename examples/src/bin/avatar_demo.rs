//! Original M5Stack-Avatar-style face demo.
//!
//! Button A cycles the six original expressions. Button B animates the mouth.

use m5unified::{Canvas, M5Unified};
use m5unified_avatar::{Avatar, Expression};
use m5unified_examples::ExampleResult;

#[cfg(target_os = "espidf")]
fn link_patches() {
    esp_idf_sys::link_patches();
}
#[cfg(not(target_os = "espidf"))]
fn link_patches() {}

const EXPRESSIONS: [Expression; 6] = [
    Expression::Happy,
    Expression::Angry,
    Expression::Sad,
    Expression::Doubt,
    Expression::Sleepy,
    Expression::Neutral,
];

fn main() -> ExampleResult {
    link_patches();

    let mut m5 = M5Unified::begin()?;
    m5.display.set_brightness(200);

    let w = m5.display.width();
    let h = m5.display.height();
    let mut canvas = Canvas::create(w, h).expect("canvas alloc failed");
    let mut avatar = Avatar::new(w, h);

    let mut frame = 0_u32;
    let mut expression_index = 0_usize;
    avatar.set_speech_text("hello");

    loop {
        m5.update();

        if m5.buttons.a().was_pressed() {
            expression_index = (expression_index + 1) % EXPRESSIONS.len();
        }

        let expression = EXPRESSIONS[expression_index];
        let t = frame as f32 * 0.05;
        avatar.set_expression(expression);
        avatar.set_gaze((t * 0.9).sin() * 0.35, (t * 0.6).cos() * 0.12);
        if m5.buttons.b().is_pressed() {
            avatar.set_mouth_open((t * 9.0).sin() * 0.5 + 0.5);
            avatar.set_speech_text("speaking");
        } else {
            avatar.set_mouth_open(0.0);
            avatar.set_speech_text("hello");
        }

        avatar.update(50);
        avatar.draw(&mut canvas);
        canvas.push(0, 0);

        frame = frame.wrapping_add(1);
        m5.delay_ms(50);
    }
}
