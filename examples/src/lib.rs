//! Shared helpers for translated M5Unified Rust samples.

use m5unified::{colors, M5Unified};

pub type ExampleResult = Result<(), Box<dyn std::error::Error>>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OptionalIntegration {
    pub name: &'static str,
    pub feature: &'static str,
    pub reason: &'static str,
}

impl OptionalIntegration {
    pub const fn new(name: &'static str, feature: &'static str, reason: &'static str) -> Self {
        Self {
            name,
            feature,
            reason,
        }
    }
}

pub fn banner(m5: &mut M5Unified, title: &str) -> ExampleResult {
    m5.display.fill_screen(colors::BLACK);
    m5.display.set_cursor(0, 0);
    m5.display.set_text_size(2);
    m5.display.set_text_color(colors::WHITE, colors::BLACK);
    m5.display.println(title)?;
    m5.display.println("m5unified-rs translated sample")?;
    Ok(())
}

pub fn unavailable_integration(
    m5: &mut M5Unified,
    integration: OptionalIntegration,
) -> ExampleResult {
    m5.display
        .println(&format!("{} unavailable", integration.name))?;
    m5.display
        .println(&format!("feature: {}", integration.feature))?;
    m5.display.println(integration.reason)?;
    Ok(())
}

pub fn finite_loop<F>(m5: &mut M5Unified, iterations: usize, mut f: F) -> ExampleResult
where
    F: FnMut(&mut M5Unified, usize) -> ExampleResult,
{
    for i in 0..iterations {
        m5.update();
        f(m5, i)?;
        m5.delay_ms(10);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_integration_records_gate() {
        let integration = OptionalIntegration::new("MP3", "mp3-decoder", "decoder is app-level");
        assert_eq!(integration.name, "MP3");
        assert_eq!(integration.feature, "mp3-decoder");
        assert_eq!(integration.reason, "decoder is app-level");
    }
}
