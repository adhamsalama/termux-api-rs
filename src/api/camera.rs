use std::process::Command;

use serde::Deserialize;

use super::errors::TermuxError;

#[derive(Debug, Deserialize)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Deserialize)]
pub struct PhysicalSize {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Deserialize)]
pub struct CameraInfo {
    pub id: String,
    pub facing: String,
    pub jpeg_output_sizes: Vec<Size>,
    pub focal_lengths: Vec<f32>,
    pub auto_exposure_modes: Vec<String>,
    pub physical_size: PhysicalSize,
    pub capabilities: Vec<String>,
}

pub type CamerasInfo = Vec<CameraInfo>;

pub struct TermuxCameraInfo {}

impl TermuxCameraInfo {
    pub fn new() -> Self {
        TermuxCameraInfo {}
    }

    pub fn run(self) -> Result<CamerasInfo, TermuxError> {
        let mut command = Command::new("termux-camera-info");
        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let camera_info: CamerasInfo = serde_json::from_slice(&output.stdout).unwrap();
                    return Ok(camera_info);
                }
                Err(TermuxError::Output(output.to_owned()))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}

pub struct TermuxCameraPhoto {
    id: Option<u8>,
    save_path: Option<String>,
}

impl TermuxCameraPhoto {
    pub fn new() -> Self {
        TermuxCameraPhoto {
            id: None,
            save_path: None,
        }
    }

    pub fn id(mut self, id: u8) -> Self {
        self.id = Some(id);
        self
    }

    pub fn save_path(mut self, path: String) -> Self {
        self.save_path = Some(path);
        self
    }

    pub fn run(self) -> Result<(), TermuxError> {
        let mut command = Command::new("termux-camera-photo");
        if let Some(id) = self.id {
            command.arg("-c").arg(format!("{}", id));
        }
        if let Some(path) = self.save_path {
            command.arg(path);
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

pub struct TermuxCamera {
    pub info: TermuxCameraInfo,
    pub photo: TermuxCameraPhoto,
}

impl TermuxCamera {
    pub fn new() -> Self {
        TermuxCamera {
            info: TermuxCameraInfo::new(),
            photo: TermuxCameraPhoto::new(),
        }
    }
}
