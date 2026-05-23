//! Rust-native renderer for the original M5Stack-Avatar face.
//!
//! The default geometry and expression behavior intentionally mirror
//! `stack-chan/m5stack-avatar`'s default face model: white face parts on a
//! black background, circular eyes, rectangular mouth, breath motion, blink,
//! and the six original expressions.

use core::f32::consts::PI;

use m5unified::{Canvas, Point, TextDatum};

const ORIGINAL_WIDTH: f32 = 320.0;
const ORIGINAL_HEIGHT: f32 = 240.0;
const PRIMARY_WHITE: u16 = 0xffff;
const BACKGROUND_BLACK: u16 = 0x0000;
const SPEECH_TEXT_LIMIT: usize = 40;

/// Compile-time RGB888 to RGB565 conversion.
pub const fn rgb565(r: u8, g: u8, b: u8) -> u16 {
    (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3)
}

/// Original M5Stack-Avatar expression set.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum Expression {
    Happy,
    Angry,
    Sad,
    Doubt,
    Sleepy,
    #[default]
    Neutral,
}

/// Colors used by [`Avatar`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Palette {
    pub background: u16,
    pub shadow: u16,
    pub face: u16,
    pub outline: u16,
    pub eye_white: u16,
    pub iris: u16,
    pub pupil: u16,
    pub highlight: u16,
    pub cheek: u16,
    pub mouth: u16,
    pub accent: u16,
    pub balloon_foreground: u16,
    pub balloon_background: u16,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            background: BACKGROUND_BLACK,
            shadow: BACKGROUND_BLACK,
            face: BACKGROUND_BLACK,
            outline: BACKGROUND_BLACK,
            eye_white: PRIMARY_WHITE,
            iris: PRIMARY_WHITE,
            pupil: PRIMARY_WHITE,
            highlight: PRIMARY_WHITE,
            cheek: PRIMARY_WHITE,
            mouth: PRIMARY_WHITE,
            accent: PRIMARY_WHITE,
            balloon_foreground: BACKGROUND_BLACK,
            balloon_background: PRIMARY_WHITE,
        }
    }
}

/// Tunable geometry. Defaults match the original 320x240 avatar coordinates.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AvatarStyle {
    pub eye_radius: f32,
    pub mouth_min_width: f32,
    pub mouth_max_width: f32,
    pub mouth_min_height: f32,
    pub mouth_max_height: f32,
    pub breath_pixels: f32,
}

impl Default for AvatarStyle {
    fn default() -> Self {
        Self {
            eye_radius: 8.0,
            mouth_min_width: 50.0,
            mouth_max_width: 90.0,
            mouth_min_height: 4.0,
            mouth_max_height: 60.0,
            breath_pixels: 3.0,
        }
    }
}

/// Mutable avatar state.
#[derive(Debug, Clone)]
pub struct Avatar {
    width: i32,
    height: i32,
    palette: Palette,
    style: AvatarStyle,
    expression: Expression,
    gaze_x: f32,
    gaze_y: f32,
    mouth_open: f32,
    speech_text: String,
    elapsed_ms: u32,
}

impl Avatar {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width: width.max(1),
            height: height.max(1),
            palette: Palette::default(),
            style: AvatarStyle::default(),
            expression: Expression::Neutral,
            gaze_x: 0.0,
            gaze_y: 0.0,
            mouth_open: 0.0,
            speech_text: String::new(),
            elapsed_ms: 0,
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn expression(&self) -> Expression {
        self.expression
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }

    pub fn style(&self) -> AvatarStyle {
        self.style
    }

    pub fn set_size(&mut self, width: i32, height: i32) {
        self.width = width.max(1);
        self.height = height.max(1);
    }

    pub fn set_palette(&mut self, palette: Palette) {
        self.palette = palette;
    }

    pub fn set_style(&mut self, style: AvatarStyle) {
        self.style = style;
    }

    pub fn set_expression(&mut self, expression: Expression) {
        self.expression = expression;
    }

    /// Set gaze offset in normalized `[-1.0, 1.0]` units.
    ///
    /// The original renderer moves each eye by at most three pixels on a
    /// 320x240 display. This value is scaled with the active display size.
    pub fn set_gaze(&mut self, x: f32, y: f32) {
        self.gaze_x = clamp_unit(x);
        self.gaze_y = clamp_unit(y);
    }

    /// Set mouth openness in normalized `[0.0, 1.0]` units.
    pub fn set_mouth_open(&mut self, level: f32) {
        self.mouth_open = level.clamp(0.0, 1.0);
    }

    pub fn set_speech_text(&mut self, text: &str) {
        self.speech_text = speech_excerpt(text);
    }

    pub fn clear_speech_text(&mut self) {
        self.speech_text.clear();
    }

    pub fn speech_text(&self) -> &str {
        &self.speech_text
    }

    pub fn update(&mut self, delta_ms: u32) {
        self.elapsed_ms = self.elapsed_ms.wrapping_add(delta_ms);
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        let layout = Layout::new(self.width, self.height, self.style);
        let breath = self.breath();
        let eye_open = self.auto_eye_open();
        let gaze = self.gaze();

        canvas.fill_screen(self.palette.background);
        draw_eye(
            canvas,
            EyeSpec {
                center: layout.right_eye,
                radius: layout.eye_radius,
                is_left: false,
                expression: self.expression,
                open_ratio: eye_open,
                gaze,
                palette: self.palette,
            },
        );
        draw_eye(
            canvas,
            EyeSpec {
                center: layout.left_eye,
                radius: layout.eye_radius,
                is_left: true,
                expression: self.expression,
                open_ratio: eye_open,
                gaze,
                palette: self.palette,
            },
        );
        draw_mouth(canvas, &layout, breath, self.mouth_open, self.palette);
        draw_balloon(canvas, &layout, &self.speech_text, self.palette);
    }

    fn breath(&self) -> f32 {
        (self.elapsed_ms as f32 * 2.0 * PI / 3300.0).sin().min(1.0)
    }

    fn auto_eye_open(&self) -> f32 {
        let phase = self.elapsed_ms % 4200;
        if (3600..3900).contains(&phase) {
            0.0
        } else {
            1.0
        }
    }

    fn gaze(&self) -> (f32, f32) {
        (self.gaze_x, self.gaze_y)
    }
}

fn draw_balloon(canvas: &mut Canvas, layout: &Layout, text: &str, palette: Palette) {
    if text.trim().is_empty() {
        return;
    }

    let cx = sx(240.0, layout.display_scale_x);
    let cy = sy(220.0, layout.display_scale_y);
    let text_height = sy_len(16.0, layout.display_scale_y).max(8);
    canvas.set_text_size(2);
    canvas.set_text_color(palette.balloon_foreground, palette.balloon_background);
    canvas.set_text_datum(TextDatum::MiddleCenter);
    let text_width = canvas
        .text_width(text)
        .ok()
        .filter(|width| *width > 0)
        .unwrap_or_else(|| sx_len(text.chars().count() as f32 * 12.0, layout.display_scale_x));

    canvas.fill_ellipse(
        cx - sx_len(20.0, layout.display_scale_x),
        cy,
        text_width + 2,
        text_height * 2 + 2,
        palette.balloon_foreground,
    );
    canvas.fill_triangle(
        Point {
            x: cx - sx_len(62.0, layout.display_scale_x),
            y: cy - sy_len(42.0, layout.display_scale_y),
        },
        Point {
            x: cx - sx_len(8.0, layout.display_scale_x),
            y: cy - sy_len(10.0, layout.display_scale_y),
        },
        Point {
            x: cx - sx_len(41.0, layout.display_scale_x),
            y: cy - sy_len(8.0, layout.display_scale_y),
        },
        palette.balloon_foreground,
    );
    canvas.fill_ellipse(
        cx - sx_len(20.0, layout.display_scale_x),
        cy,
        text_width,
        text_height * 2,
        palette.balloon_background,
    );
    canvas.fill_triangle(
        Point {
            x: cx - sx_len(60.0, layout.display_scale_x),
            y: cy - sy_len(40.0, layout.display_scale_y),
        },
        Point {
            x: cx - sx_len(10.0, layout.display_scale_x),
            y: cy - sy_len(10.0, layout.display_scale_y),
        },
        Point {
            x: cx - sx_len(40.0, layout.display_scale_x),
            y: cy - sy_len(10.0, layout.display_scale_y),
        },
        palette.balloon_background,
    );

    let x = cx - text_width / 6 - sx_len(15.0, layout.display_scale_x);
    let _ = canvas.draw_string(text, x, cy);
}

#[derive(Debug, Copy, Clone)]
struct Layout {
    right_eye: Point,
    left_eye: Point,
    mouth: Point,
    eye_radius: i32,
    mouth_min_width: i32,
    mouth_max_width: i32,
    mouth_min_height: i32,
    mouth_max_height: i32,
    breath_pixels: i32,
    display_scale_x: f32,
    display_scale_y: f32,
}

impl Layout {
    fn new(width: i32, height: i32, style: AvatarStyle) -> Self {
        let scale_x = width as f32 / ORIGINAL_WIDTH;
        let scale_y = height as f32 / ORIGINAL_HEIGHT;
        let scale = scale_x.min(scale_y);

        Self {
            right_eye: Point {
                x: sx(90.0, scale_x),
                y: sy(93.0, scale_y),
            },
            left_eye: Point {
                x: sx(230.0, scale_x),
                y: sy(96.0, scale_y),
            },
            mouth: Point {
                x: sx(163.0, scale_x),
                y: sy(148.0, scale_y),
            },
            eye_radius: ss(style.eye_radius, scale).max(1),
            mouth_min_width: sx_len(style.mouth_min_width, scale_x).max(1),
            mouth_max_width: sx_len(style.mouth_max_width, scale_x).max(1),
            mouth_min_height: sy_len(style.mouth_min_height, scale_y).max(1),
            mouth_max_height: sy_len(style.mouth_max_height, scale_y).max(1),
            breath_pixels: ss(style.breath_pixels, scale),
            display_scale_x: scale_x,
            display_scale_y: scale_y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct EyeSpec {
    center: Point,
    radius: i32,
    is_left: bool,
    expression: Expression,
    open_ratio: f32,
    gaze: (f32, f32),
    palette: Palette,
}

fn draw_eye(canvas: &mut Canvas, spec: EyeSpec) {
    let offset_x = (spec.gaze.0 * 3.0).round() as i32;
    let offset_y = (spec.gaze.1 * 3.0).round() as i32;
    let x = spec.center.x + offset_x;
    let y = spec.center.y + offset_y;

    if spec.open_ratio > 0.0 {
        canvas.fill_circle(x, y, spec.radius, spec.palette.eye_white);
        match spec.expression {
            Expression::Angry | Expression::Sad => {
                let x0 = x - spec.radius;
                let y0 = y - spec.radius;
                let x1 = x0 + spec.radius * 2;
                let y1 = y0;
                let x2 = if clipped_to_left(spec.is_left, spec.expression) {
                    x0
                } else {
                    x1
                };
                let y2 = y0 + spec.radius;
                canvas.fill_triangle(
                    Point { x: x0, y: y0 },
                    Point { x: x1, y: y1 },
                    Point { x: x2, y: y2 },
                    spec.palette.background,
                );
            }
            Expression::Happy | Expression::Sleepy => {
                let x0 = x - spec.radius;
                let mut y0 = y - spec.radius;
                let w = spec.radius * 2 + 4;
                let h = spec.radius + 2;
                if spec.expression == Expression::Happy {
                    y0 += spec.radius;
                    canvas.fill_circle(x, y, (spec.radius * 2 / 3).max(1), spec.palette.background);
                }
                canvas.fill_rect(x0, y0, w, h, spec.palette.background);
            }
            Expression::Doubt | Expression::Neutral => {}
        }
    } else {
        canvas.fill_rect(
            x - spec.radius,
            y - 2,
            spec.radius * 2,
            4,
            spec.palette.eye_white,
        );
    }
}

fn clipped_to_left(is_left: bool, expression: Expression) -> bool {
    match expression {
        Expression::Angry => is_left,
        Expression::Sad => !is_left,
        _ => false,
    }
}

fn draw_mouth(
    canvas: &mut Canvas,
    layout: &Layout,
    breath: f32,
    mouth_open: f32,
    palette: Palette,
) {
    let open = mouth_open.clamp(0.0, 1.0);
    let h = layout.mouth_min_height
        + ((layout.mouth_max_height - layout.mouth_min_height) as f32 * open) as i32;
    let w = layout.mouth_min_width
        + ((layout.mouth_max_width - layout.mouth_min_width) as f32 * (1.0 - open)) as i32;
    let breath_y = (breath * layout.breath_pixels as f32 * 5.0 / 3.0).round() as i32;
    let x = layout.mouth.x - w / 2;
    let y = layout.mouth.y - h / 2 + breath_y;
    canvas.fill_rect(x, y, w, h, palette.mouth);
}

fn sx(value: f32, scale_x: f32) -> i32 {
    (value * scale_x).round() as i32
}

fn sy(value: f32, scale_y: f32) -> i32 {
    (value * scale_y).round() as i32
}

fn sx_len(value: f32, scale_x: f32) -> i32 {
    (value * scale_x).round() as i32
}

fn sy_len(value: f32, scale_y: f32) -> i32 {
    (value * scale_y).round() as i32
}

fn ss(value: f32, scale: f32) -> i32 {
    (value * scale).round() as i32
}

fn speech_excerpt(text: &str) -> String {
    let mut output = String::new();
    for ch in text.trim().chars().take(SPEECH_TEXT_LIMIT) {
        if ch == '\0' || ch == '\n' || ch == '\r' || ch == '\t' {
            output.push(' ');
        } else {
            output.push(ch);
        }
    }
    if text.trim().chars().count() > SPEECH_TEXT_LIMIT {
        output.push_str("...");
    }
    output
}

fn clamp_unit(value: f32) -> f32 {
    value.clamp(-1.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb565_converts_known_values() {
        assert_eq!(rgb565(0, 0, 0), 0x0000);
        assert_eq!(rgb565(255, 255, 255), 0xffff);
        assert_eq!(rgb565(255, 0, 0), 0xf800);
    }

    #[test]
    fn original_layout_matches_m5stack_avatar_coordinates() {
        let layout = Layout::new(320, 240, AvatarStyle::default());
        assert_eq!(layout.right_eye, Point { x: 90, y: 93 });
        assert_eq!(layout.left_eye, Point { x: 230, y: 96 });
        assert_eq!(layout.mouth, Point { x: 163, y: 148 });
        assert_eq!(layout.eye_radius, 8);
        assert_eq!(layout.mouth_min_width, 50);
        assert_eq!(layout.mouth_max_width, 90);
        assert_eq!(layout.mouth_min_height, 4);
        assert_eq!(layout.mouth_max_height, 60);
    }

    #[test]
    fn gaze_and_mouth_inputs_are_clamped() {
        let mut avatar = Avatar::new(320, 240);
        avatar.set_gaze(5.0, -5.0);
        avatar.set_mouth_open(2.0);

        assert_eq!(avatar.gaze_x, 1.0);
        assert_eq!(avatar.gaze_y, -1.0);
        assert_eq!(avatar.mouth_open, 1.0);
    }

    #[test]
    fn angry_and_sad_eye_clips_match_original_sides() {
        assert!(clipped_to_left(true, Expression::Angry));
        assert!(!clipped_to_left(false, Expression::Angry));
        assert!(!clipped_to_left(true, Expression::Sad));
        assert!(clipped_to_left(false, Expression::Sad));
    }

    #[test]
    fn update_advances_animation_clock() {
        let mut avatar = Avatar::new(320, 240);
        avatar.update(16);
        avatar.update(34);
        assert_eq!(avatar.elapsed_ms, 50);
    }

    #[test]
    fn speech_text_is_sanitized_and_limited() {
        let mut avatar = Avatar::new(320, 240);
        avatar
            .set_speech_text("hello\nstackchan\0this text is intentionally longer than the bubble");
        assert!(!avatar.speech_text().contains('\n'));
        assert!(!avatar.speech_text().contains('\0'));
        assert!(avatar.speech_text().ends_with("..."));
    }
}
