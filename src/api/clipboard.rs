use std::process::Command;

use super::errors::TermuxError;

pub struct TermuxClipboardGet {}

impl TermuxClipboardGet {
    pub fn new() -> Self {
        TermuxClipboardGet {}
    }

    pub fn run(&self) -> Result<String, TermuxError> {
        let mut command = Command::new("termux-clipboard-get");
        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    match String::from_utf8(output.stdout) {
                        Ok(value) => return Ok(value),
                        Err(_) => return Ok(String::new()),
                    }
                }
                Err(TermuxError::Output(output.to_owned()))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}

pub struct TermuxClipboardSet {
    content: Option<String>,
}

impl TermuxClipboardSet {
    pub fn new() -> Self {
        TermuxClipboardSet { content: None }
    }

    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn run(self) -> Result<(), TermuxError> {
        let mut command = Command::new("termux-clipboard-set");
        if let Some(content) = self.content {
            command.arg(content);
        }
        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    return Ok(());
                }
                Err(TermuxError::Output(output.to_owned()))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}

pub struct TermuxClipboard {
    pub get: TermuxClipboardGet,
    pub set: TermuxClipboardSet,
}

impl TermuxClipboard {
    pub fn new() -> Self {
        TermuxClipboard {
            get: TermuxClipboardGet::new(),
            set: TermuxClipboardSet::new(),
        }
    }
}
