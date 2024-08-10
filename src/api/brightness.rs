use std::process::Command;

use super::errors::TermuxError;

pub enum Brightness {
    Level(u8),
    Auto,
}

pub struct TermuxBrightness {
    level: Brightness,
}
impl TermuxBrightness {
    pub fn new() -> Self {
        Self {
            level: Brightness::Auto,
        }
    }
    pub fn set(mut self, brightness: Brightness) -> Self {
        self.level = brightness;
        self
    }
    pub fn run(&self) -> Result<(), TermuxError> {
        let mut command = Command::new("termux-brightness");
        match self.level {
            Brightness::Level(lvl) => {
                command.arg(format!("{lvl}"));
            }
            Brightness::Auto => {
                command.arg("auto".to_string());
            }
        }
        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    return Ok(());
                }
                Err(TermuxError::Output(output))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}
