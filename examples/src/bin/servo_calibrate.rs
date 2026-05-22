//! Servo calibration wizard for StackChan head servos.
//!
//! Lets you set the neutral (home) position for each servo axis and persist it
//! to NVS so stackchan_face and other examples use the correct zero position.
//!
//! Controls (M5Stack Core S3):
//!   BtnA (left)    — release torque so you can move servos by hand
//!   BtnB (center)  — re-enable torque and move to current calibrated home
//!   BtnC (right)   — sample current raw position → set as new home → save NVS
//!   Hold BtnA 2 s  — reset to factory defaults

use m5unified::{M5Unified, StackChanServos, PITCH_ZERO_DEFAULT, YAW_ZERO_DEFAULT};
use m5unified_examples::ExampleResult;

#[cfg(target_os = "espidf")]
fn link_patches() {
    esp_idf_sys::link_patches();
}
#[cfg(not(target_os = "espidf"))]
fn link_patches() {}

const fn rgb(r: u8, g: u8, b: u8) -> u16 {
    (((r as u16) >> 3) << 11) | (((g as u16) >> 2) << 5) | ((b as u16) >> 3)
}

const BG: u16 = rgb(20, 20, 40);
const FG: u16 = rgb(220, 220, 220);
const GREEN: u16 = rgb(80, 220, 100);
const AMBER: u16 = rgb(255, 180, 30);
const RED: u16 = rgb(255, 60, 60);
const BLUE: u16 = rgb(80, 160, 255);

fn draw_ui(
    m5: &mut M5Unified,
    yaw_zero: u16,
    pitch_zero: u16,
    yaw_raw: Option<i32>,
    pitch_raw: Option<i32>,
    torque: bool,
    status: &str,
) {
    let d = &mut m5.display;
    d.fill_screen(BG);
    d.set_text_size(2);

    d.set_text_color(BLUE, BG);
    d.set_cursor(4, 4);
    let _ = d.println("== SERVO CALIBRATION ==");

    // Yaw row
    d.set_text_color(FG, BG);
    d.set_cursor(4, 36);
    let _ = d.print("YAW  zero:");
    d.set_text_color(GREEN, BG);
    let _ = d.print(&format!("{:4}", yaw_zero));
    d.set_text_color(FG, BG);
    let _ = d.print("  now:");
    match yaw_raw {
        Some(r) => {
            d.set_text_color(AMBER, BG);
            let _ = d.println(&format!("{:4}", r));
        }
        None => {
            d.set_text_color(RED, BG);
            let _ = d.println(" ---");
        }
    }

    // Pitch row
    d.set_text_color(FG, BG);
    d.set_cursor(4, 68);
    let _ = d.print("PTCH zero:");
    d.set_text_color(GREEN, BG);
    let _ = d.print(&format!("{:4}", pitch_zero));
    d.set_text_color(FG, BG);
    let _ = d.print("  now:");
    match pitch_raw {
        Some(r) => {
            d.set_text_color(AMBER, BG);
            let _ = d.println(&format!("{:4}", r));
        }
        None => {
            d.set_text_color(RED, BG);
            let _ = d.println(" ---");
        }
    }

    // Torque indicator
    d.set_cursor(4, 100);
    d.set_text_color(FG, BG);
    let _ = d.print("Torque: ");
    if torque {
        d.set_text_color(GREEN, BG);
        let _ = d.println("ON ");
    } else {
        d.set_text_color(RED, BG);
        let _ = d.println("OFF");
    }

    // Status line
    d.set_cursor(4, 132);
    d.set_text_color(AMBER, BG);
    let _ = d.println(status);

    // Button legend at bottom
    d.set_text_size(1);
    d.set_text_color(FG, BG);
    d.set_cursor(0, 224);
    let _ = d.print("[A] torque off  [B] go home  [C] set home+save");
}

fn main() -> ExampleResult {
    link_patches();
    let mut m5 = M5Unified::begin()?;
    m5.display.set_brightness(200);

    let mut servos = match StackChanServos::new() {
        Ok(mut s) => {
            s.set_torque(true);
            Some(s)
        }
        Err(_) => None,
    };

    let mut torque = true;
    let mut status = String::from("Ready.");
    let mut btn_a_frames = 0u32;

    loop {
        m5.update();

        let (yaw_zero, pitch_zero) = servos
            .as_ref()
            .map(|s| (s.yaw_zero(), s.pitch_zero()))
            .unwrap_or((YAW_ZERO_DEFAULT, PITCH_ZERO_DEFAULT));

        let yaw_raw = servos.as_mut().and_then(|s| s.read_raw_pos(1));
        let pitch_raw = servos.as_mut().and_then(|s| s.read_raw_pos(2));

        // Track BtnA hold duration (factory reset at 2 s = 40 frames at 50 ms)
        if m5.buttons.a().is_pressed() {
            btn_a_frames += 1;
        } else {
            if btn_a_frames > 0 && btn_a_frames < 40 {
                // Short press: release torque
                torque = false;
                if let Some(ref mut s) = servos {
                    s.set_torque(false);
                }
                status = String::from("Torque OFF. Move servos by hand.");
            }
            btn_a_frames = 0;
        }
        if btn_a_frames >= 40 {
            if let Some(ref mut s) = servos {
                s.reset_calibration();
            }
            torque = true;
            if let Some(ref mut s) = servos {
                s.set_torque(true);
            }
            status = String::from("Factory defaults restored.");
            btn_a_frames = 0;
        }

        // BtnB: re-enable torque and return to home position
        if m5.buttons.b().was_clicked() {
            torque = true;
            if let Some(ref mut s) = servos {
                s.set_torque(true);
                s.move_both(0, 0, 500, 0);
            }
            status = String::from("Moving to home...");
        }

        // BtnC: sample current positions and save as new home
        if m5.buttons.c().was_clicked() {
            if let Some(ref mut s) = servos {
                let y = s.calibrate_from_current_pos(1);
                let p = s.calibrate_from_current_pos(2);
                let ok = s.save_calibration();
                status = if ok {
                    format!("Saved! yaw={} pitch={}", y.unwrap_or(0), p.unwrap_or(0))
                } else {
                    String::from("NVS write failed.")
                };
            } else {
                status = String::from("No servos connected.");
            }
        }

        draw_ui(
            &mut m5, yaw_zero, pitch_zero, yaw_raw, pitch_raw, torque, &status,
        );
        m5.delay_ms(50);
    }
}
