use confirm::TermuxConfirmDialog;

pub mod confirm;

pub struct TermuxDialog {
    pub confirm: TermuxConfirmDialog,
}

impl TermuxDialog {
    pub fn new() -> Self {
        TermuxDialog {
            confirm: TermuxConfirmDialog::new(),
        }
    }
}
