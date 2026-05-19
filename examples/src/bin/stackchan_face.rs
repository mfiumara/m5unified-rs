//! StackChan face — animated display + head-bobbing servo demo.
//!
//! All drawing happens on an off-screen LGFX_Sprite canvas and is pushed to
//! the display in one DMA transfer per frame, eliminating flicker.
//!
//! Servo wiring (M5Stack Core S3 default):
//!   TX = GPIO 6   RX = GPIO 7   baud = 1 000 000
//!   Servo ID 1 = yaw (left/right)   ID 2 = pitch (up/down)

use core::f32::consts::PI;

use m5unified::{Canvas, M5Unified, Point, Rect, StackChanServos};
use m5unified_examples::ExampleResult;

#[cfg(target_os = "espidf")]
fn link_patches() { esp_idf_sys::link_patches(); }
#[cfg(not(target_os = "espidf"))]
fn link_patches() {}

// ── Colour palette (RGB565, computed at compile time) ───────────────────────

const fn rgb(r: u8, g: u8, b: u8) -> u16 {
    (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3)
}

const BG: u16        = rgb(22,  33,  62);   // deep navy
const SHADOW: u16    = rgb(10,  15,  30);   // shadow behind face
const FACE: u16      = rgb(255, 218, 185);  // peach skin
const EYE_W: u16     = rgb(240, 248, 255);  // eye white
const IRIS: u16      = rgb(65,  130, 220);  // blue iris
const PUPIL: u16     = rgb(15,  15,  40);   // near-black pupil
const SHINE: u16     = rgb(255, 255, 255);  // specular highlight
const CHEEK: u16     = rgb(255, 170, 180);  // rose blush
const MOUTH: u16     = rgb(45,  35,  50);   // dark plum
const OUTLINE: u16   = rgb(200, 150, 130);  // warm face outline

// ── Layout (320 × 240) ──────────────────────────────────────────────────────

const EYE_OUTER_R: i32 = 40;
const IRIS_R: i32       = 28;
const PUPIL_R: i32      = 17;
const SHINE_R: i32      = 6;
const CHEEK_R: i32      = 20;

// ── Eye helper ───────────────────────────────────────────────────────────────

fn draw_eye(cv: &mut Canvas, cx: i32, cy: i32, blink: bool, dx: i32, dy: i32) {
    cv.fill_smooth_circle(Point { x: cx, y: cy }, EYE_OUTER_R, EYE_W);

    if blink {
        // Flatten the eye to a horizontal sliver
        let stripe = Rect { x: cx - EYE_OUTER_R, y: cy - 4, w: EYE_OUTER_R * 2, h: 8 };
        cv.fill_smooth_round_rect(stripe, 4, EYE_W);
        cv.fill_rect(cx - EYE_OUTER_R - 1, cy - EYE_OUTER_R, EYE_OUTER_R * 2 + 2, EYE_OUTER_R - 4, FACE);
        cv.fill_rect(cx - EYE_OUTER_R - 1, cy + 5, EYE_OUTER_R * 2 + 2, EYE_OUTER_R - 4, FACE);
        return;
    }

    let px = cx + dx.clamp(-12, 12);
    let py = cy + dy.clamp(-8, 8);
    cv.fill_smooth_circle(Point { x: px, y: py }, IRIS_R, IRIS);
    cv.fill_smooth_circle(Point { x: px, y: py }, PUPIL_R, PUPIL);
    cv.fill_smooth_circle(Point { x: px + 9, y: py - 9 }, SHINE_R, SHINE);
}

// ── Full face render (draws to canvas, then push() flips to display) ─────────

fn draw_face(cv: &mut Canvas, w: i32, h: i32, frame: u32, yaw: f32, pitch: f32, surprised: bool) {
    let cx = w / 2;
    let cy = h / 2;

    // Background
    cv.fill_screen(BG);

    // Drop shadow
    cv.fill_smooth_circle(Point { x: cx + 4, y: cy + 7 }, 100, SHADOW);

    // Face
    cv.fill_smooth_circle(Point { x: cx, y: cy }, 100, FACE);
    cv.draw_circle(cx, cy, 100, OUTLINE);

    // Eyes
    let eye_y = cy - 18;
    let blink  = (frame % 200) < 4;
    let pdx    = (yaw   * 0.25) as i32;  // pupils track head direction
    let pdy    = (-pitch * 0.20) as i32;

    draw_eye(cv, cx - 48, eye_y, blink, pdx, pdy);
    draw_eye(cv, cx + 48, eye_y, blink, pdx, pdy);

    // Cheeks
    cv.fill_smooth_circle(Point { x: cx - 68, y: cy + 28 }, CHEEK_R, CHEEK);
    cv.fill_smooth_circle(Point { x: cx + 68, y: cy + 28 }, CHEEK_R, CHEEK);

    // Mouth
    let mc = Point { x: cx, y: cy + 46 };
    if surprised {
        cv.fill_ellipse(mc.x, mc.y, 16, 22, MOUTH);
        cv.draw_ellipse(mc.x, mc.y, 16, 22, OUTLINE);
    } else {
        cv.fill_arc(mc, 26, 34, 218.0, 322.0, MOUTH);
    }

    // Nose hint
    cv.fill_smooth_circle(Point { x: cx, y: cy + 12 }, 4, OUTLINE);
}

// ── Servo bobbing ────────────────────────────────────────────────────────────

struct BobState {
    servos: Option<StackChanServos>,
}

impl BobState {
    fn new() -> Self {
        let servos = match StackChanServos::new() {
            Ok(mut s) => {
                s.set_torque(true);
                // Home both axes before starting the animation
                s.move_both(0, 10, 400, 0);
                Some(s)
            }
            Err(_) => None, // display-only mode if no servos
        };
        Self { servos }
    }

    fn update(&mut self, yaw_deg: i32, pitch_deg: i32) {
        if let Some(ref mut s) = self.servos {
            // time_ms=60 keeps motion smooth at 20 fps (50 ms period)
            s.move_both(yaw_deg, pitch_deg, 60, 0);
        }
    }
}

// ── Main ─────────────────────────────────────────────────────────────────────

fn main() -> ExampleResult {
    link_patches();

    let mut m5 = M5Unified::begin()?;
    m5.display.set_brightness(200);

    let w = m5.display.width();
    let h = m5.display.height();

    // Allocate the off-screen sprite (320×240 × 2 bytes ≈ 150 KB in PSRAM)
    let mut canvas = Canvas::create(w, h).expect("canvas alloc failed");

    let mut bob = BobState::new();

    // Give servos time to home before the animation starts
    m5.delay_ms(500);

    let mut frame: u32 = 0;

    loop {
        m5.update();

        // Use frame counter for time — avoids millis() starting mid-cycle at boot
        let t = frame as f32 * (50.0 / 1000.0); // seconds at 20 fps

        // Yaw: ±22 ° at 3.2 s period
        let yaw_f   = 22.0_f32 * (t * 2.0 * PI / 3.2).sin();
        // Pitch: 10° ± 8° (stays 2°–18°, never hits limit)  at 2.0 s period
        let pitch_f = 10.0_f32 + 8.0_f32 * (t * 2.0 * PI / 2.0).sin();

        let yaw_deg   = yaw_f   as i32;
        let pitch_deg = pitch_f as i32;

        // Surprised expression when the head is accelerating fast
        let surprised = yaw_f.abs() > 18.0;

        // Draw frame off-screen …
        draw_face(&mut canvas, w, h, frame, yaw_f, pitch_f, surprised);
        // … then flip to display in one shot
        canvas.push(0, 0);

        // Move servos
        bob.update(yaw_deg, pitch_deg);

        frame = frame.wrapping_add(1);
        m5.delay_ms(50);
    }
}
