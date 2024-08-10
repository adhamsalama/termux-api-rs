use std::process::Command;

use super::errors::TermuxError;

pub enum ToastPosition {
    Top,
    Middle,
    Bottom,
}

pub struct TermuxToast {
    pub text: Option<String>,
    pub background_color: Option<String>,
    pub text_color: Option<String>,
    pub position: ToastPosition,
    pub short_duration: bool,
}

impl TermuxToast {
    pub fn text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        self
    }

    pub fn background_color(mut self, color: &str) -> Self {
        self.background_color = Some(color.to_string());
        self
    }

    pub fn text_color(mut self, color: &str) -> Self {
        self.text_color = Some(color.to_string());
        self
    }

    pub fn position(mut self, position: ToastPosition) -> Self {
        self.position = position;
        self
    }

    pub fn short_duration(mut self, short: bool) -> Self {
        self.short_duration = short;
        self
    }

    pub fn new() -> Self {
        Self {
            text: None,
            background_color: None,
            text_color: None,
            position: ToastPosition::Middle,
            short_duration: false,
        }
    }

    pub fn run(&mut self) -> Result<(), TermuxError> {
        let mut command = Command::new("termux-toast");

        if let Some(ref color) = self.background_color {
            command.arg("-b").arg(color);
        }
        if let Some(ref color) = self.text_color {
            command.arg("-c").arg(color);
        }
        command.arg("-g").arg(match self.position {
            ToastPosition::Top => "top",
            ToastPosition::Middle => "middle",
            ToastPosition::Bottom => "bottom",
        });

        if self.short_duration {
            command.arg("-s");
        }

        if let Some(ref text) = self.text {
            command.arg(text);
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
