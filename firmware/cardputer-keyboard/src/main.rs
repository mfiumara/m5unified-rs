use std::{thread, time::Duration};

use anyhow::Result;
use m5unified::{colors, Cardputer, GrovePin, NecFrame};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();

    let mut cardputer = Cardputer::begin()?;
    cardputer.display.set_rotation(1);
    cardputer.display.fill_screen(colors::BLACK);
    cardputer
        .display
        .set_text_color(colors::GREEN, colors::BLACK);
    cardputer.display.set_text_size(1);
    cardputer.display.set_cursor(4, 4);
    cardputer.display.println("Cardputer Rust keyboard")?;
    cardputer.display.set_cursor(4, 20);
    cardputer.display.println("Type keys; enter clears")?;

    let sd_mounted = cardputer.sd.try_begin().is_ok();
    cardputer.display.set_cursor(4, 36);
    cardputer.display.println(if sd_mounted {
        "SD mounted"
    } else {
        "SD unavailable"
    })?;
    if sd_mounted {
        let _ = cardputer.sd.write_file("/m5rs_boot.txt", b"cardputer rust\n");
    }
    let ir_ready = cardputer.ir.try_begin().is_ok();
    cardputer.display.set_cursor(4, 48);
    cardputer.display.println(if ir_ready {
        "IR ready"
    } else {
        "IR unavailable"
    })?;
    let grove_ready = cardputer.grove.i2c_try_begin().is_ok();
    let grove_devices = if grove_ready {
        cardputer.grove.i2c_scan().len()
    } else {
        0
    };
    cardputer.display.set_cursor(4, 60);
    cardputer
        .display
        .println(&format!("Grove I2C: {grove_devices}"))?;
    let grove_adc = cardputer.grove.analog_read_millivolts(GrovePin::G1);
    cardputer.display.set_cursor(4, 72);
    cardputer
        .display
        .println(&format!("Grove G1: {grove_adc:?} mV"))?;

    let mut line = String::new();
    render_line(&mut cardputer, &line)?;

    loop {
        cardputer.update();

        if cardputer.keyboard.is_change() && cardputer.keyboard.is_pressed() {
            if let Some(state) = cardputer.keyboard.state() {
                if state.del {
                    line.pop();
                }
                if state.enter {
                    line.clear();
                }
                line.push_str(&state.word_lossy());
                render_line(&mut cardputer, &line)?;
            }
        }

        if cardputer.button_a.was_pressed() {
            cardputer.keyboard.set_capslocked(!cardputer.keyboard.capslocked());
            let _ = cardputer.ir.send_nec(NecFrame {
                address: 0x1111,
                command: 0x34,
                repeats: 0,
            });
            render_line(&mut cardputer, &line)?;
        }

        thread::sleep(Duration::from_millis(20));
    }
}

fn render_line(cardputer: &mut Cardputer, line: &str) -> Result<()> {
    cardputer.display.fill_rect(0, 84, 240, 44, colors::BLACK);
    cardputer.display.set_cursor(4, 84);
    cardputer
        .display
        .println(if cardputer.keyboard.capslocked() {
            "caps: on"
        } else {
            "caps: off"
        })?;
    cardputer.display.set_cursor(4, 100);
    cardputer.display.println(line)?;
    Ok(())
}
