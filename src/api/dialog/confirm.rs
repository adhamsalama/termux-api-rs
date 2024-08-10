use serde::{Deserialize, Deserializer};
use std::process::Command;

use crate::api::errors::TermuxError;

#[derive(Debug, Deserialize)]
pub struct TermuxDialogConfirmResult {
    pub code: i32,

    #[serde(rename = "text", deserialize_with = "deserialize_yes_no")]
    pub confirmed: bool,
}

fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt.as_deref() {
        Some("yes") => Ok(true),
        Some("no") => Ok(false),
        _ => Err(serde::de::Error::custom("Unexpected value")),
    }
}

pub struct TermuxConfirmDialog {
    pub title: Option<String>,
    pub hint: Option<String>,
}

impl TermuxConfirmDialog {
    pub fn new() -> Self {
        Self {
            title: None,
            hint: None,
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn hint(mut self, hint: &str) -> Self {
        self.hint = Some(hint.to_string());
        self
    }

    pub fn run(&self) -> Result<TermuxDialogConfirmResult, TermuxError> {
        let mut command = Command::new("termux-dialog");
        command.arg("confirm");

        if let Some(ref title) = self.title {
            command.arg("-t").arg(title);
        }

        if let Some(ref hint) = self.hint {
            command.arg("-i").arg(hint);
        }

        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let result: TermuxDialogConfirmResult =
                        serde_json::from_slice(&output.stdout).unwrap();
                    Ok(result)
                } else {
                    Err(TermuxError::Output(output))
                }
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}
