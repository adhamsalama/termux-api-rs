use std::process::Command;

use super::errors::TermuxError;

pub struct TermuxNotification {
    pub title: Option<String>,
    pub content: Option<String>,
    pub action: Option<String>,
    pub button1_text: Option<String>,
    pub button1_action: Option<String>,
    pub button2_text: Option<String>,
    pub button2_action: Option<String>,
    pub button3_text: Option<String>,
    pub button3_action: Option<String>,
    pub group: Option<String>,
    pub id: Option<u32>,
    pub image_path: Option<String>,
    pub led_color: Option<String>,
    pub led_off: Option<u32>,
    pub led_on: Option<u32>,
    pub on_delete: Option<String>,
    pub ongoing: bool,
    pub priority: Option<String>,
    pub sound: bool,
    pub vibrate_pattern: Option<String>,
    pub notification_type: Option<String>,
    pub media_next: Option<String>,
    pub media_pause: Option<String>,
    pub media_play: Option<String>,
    pub media_previous: Option<String>,
    pub alert_once: bool,
}

impl TermuxNotification {
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn action(mut self, action: &str) -> Self {
        self.action = Some(action.to_string());
        self
    }

    pub fn button1(mut self, text: &str, action: &str) -> Self {
        self.button1_text = Some(text.to_string());
        self.button1_action = Some(action.to_string());
        self
    }

    pub fn button2(mut self, text: &str, action: &str) -> Self {
        self.button2_text = Some(text.to_string());
        self.button2_action = Some(action.to_string());
        self
    }

    pub fn button3(mut self, text: &str, action: &str) -> Self {
        self.button3_text = Some(text.to_string());
        self.button3_action = Some(action.to_string());
        self
    }

    pub fn group(mut self, group: &str) -> Self {
        self.group = Some(group.to_string());
        self
    }

    pub fn id(mut self, id: &u32) -> Self {
        self.id = Some(id.clone());
        self
    }

    pub fn image_path(mut self, path: &str) -> Self {
        self.image_path = Some(path.to_string());
        self
    }

    pub fn led_color(mut self, color: &str) -> Self {
        self.led_color = Some(color.to_string());
        self
    }
    pub fn led_off(mut self, milliseconds: u32) -> Self {
        self.led_off = Some(milliseconds);
        self
    }

    pub fn led_on(mut self, milliseconds: u32) -> Self {
        self.led_on = Some(milliseconds);
        self
    }

    pub fn on_delete(mut self, action: &str) -> Self {
        self.on_delete = Some(action.to_string());
        self
    }

    pub fn ongoing(mut self, ongoing: bool) -> Self {
        self.ongoing = ongoing;
        self
    }

    pub fn priority(mut self, priority: &str) -> Self {
        self.priority = Some(priority.to_string());
        self
    }

    pub fn sound(mut self, sound: bool) -> Self {
        self.sound = sound;
        self
    }

    pub fn vibrate(mut self, pattern: &str) -> Self {
        self.vibrate_pattern = Some(pattern.to_string());
        self
    }

    pub fn notification_type(mut self, ntype: &str) -> Self {
        self.notification_type = Some(ntype.to_string());
        self
    }

    pub fn media_next(mut self, action: &str) -> Self {
        self.media_next = Some(action.to_string());
        self
    }

    pub fn media_pause(mut self, action: &str) -> Self {
        self.media_pause = Some(action.to_string());
        self
    }

    pub fn media_play(mut self, action: &str) -> Self {
        self.media_play = Some(action.to_string());
        self
    }

    pub fn media_previous(mut self, action: &str) -> Self {
        self.media_previous = Some(action.to_string());
        self
    }

    pub fn alert_once(mut self, alert_once: bool) -> Self {
        self.alert_once = alert_once;
        self
    }

    pub fn new() -> Self {
        Self {
            action: None,
            alert_once: false,
            button1_text: None,
            button1_action: None,
            button2_text: None,
            button2_action: None,
            button3_text: None,
            button3_action: None,
            content: None,
            group: None,
            id: None,
            image_path: None,
            led_color: None,
            led_off: None,
            led_on: None,
            on_delete: None,
            ongoing: false,
            priority: None,
            sound: false,
            title: None,
            vibrate_pattern: None,
            notification_type: None,
            media_next: None,
            media_pause: None,
            media_play: None,
            media_previous: None,
        }
    }
    pub fn run(&self) -> Result<(), TermuxError> {
        let mut command = Command::new("termux-notification");

        if let Some(ref action) = self.action {
            command.arg("--action").arg(action);
        }
        if self.alert_once {
            command.arg("--alert-once");
        }
        if let Some(ref text) = self.button1_text {
            command.arg("--button1").arg(text);
        }
        if let Some(ref text) = self.button1_action {
            command.arg("--button1-action").arg(text);
        }
        if let Some(ref text) = self.button2_text {
            command.arg("--button2").arg(text);
        }
        if let Some(ref text) = self.button2_action {
            command.arg("--button2-action").arg(text);
        }
        if let Some(ref text) = self.button3_text {
            command.arg("--button3").arg(text);
        }
        if let Some(ref text) = self.button3_action {
            command.arg("--button3-action").arg(text);
        }
        if let Some(ref content) = self.content {
            command.arg("-c").arg(content);
        }
        if let Some(ref group) = self.group {
            command.arg("--group").arg(group);
        }
        if let Some(ref id) = self.id {
            command.arg("-i").arg(format!("{}", id));
        }
        if let Some(ref image_path) = self.image_path {
            command.arg("--image-path").arg(image_path);
        }
        if let Some(ref led_color) = self.led_color {
            command.arg("--led-color").arg(led_color);
        }
        if let Some(led_off) = self.led_off {
            command.arg("--led-off").arg(led_off.to_string());
        }
        if let Some(led_on) = self.led_on {
            command.arg("--led-on").arg(led_on.to_string());
        }
        if let Some(ref on_delete) = self.on_delete {
            command.arg("--on-delete").arg(on_delete);
        }
        if self.ongoing {
            command.arg("--ongoing");
        }
        if let Some(ref priority) = self.priority {
            command.arg("--priority").arg(priority);
        }
        if self.sound {
            command.arg("--sound");
        }
        if let Some(ref title) = self.title {
            command.arg("-t").arg(title);
        }
        if let Some(ref vibrate) = self.vibrate_pattern {
            command.arg("--vibrate").arg(vibrate);
        }
        if let Some(ref notification_type) = self.notification_type {
            command.arg("--type").arg(notification_type);
        }
        if let Some(ref media_next) = self.media_next {
            command.arg("--media-next").arg(media_next);
        }
        if let Some(ref media_pause) = self.media_pause {
            command.arg("--media-pause").arg(media_pause);
        }
        if let Some(ref media_play) = self.media_play {
            command.arg("--media-play").arg(media_play);
        }
        if let Some(ref media_previous) = self.media_previous {
            command.arg("--media-previous").arg(media_previous);
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
