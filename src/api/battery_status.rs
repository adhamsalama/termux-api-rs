use super::errors::TermuxError;
use serde::Deserialize;

use std::process::Command;
#[derive(Debug, Deserialize)]
pub enum HealthStatus {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "GOOD")]
    Good,
    #[serde(rename = "OVERHEAT")]
    Overheat,
    #[serde(rename = "DEAD")]
    Dead,
    #[serde(rename = "OVER_VOLTAGE")]
    OverVoltage,
    #[serde(rename = "UNSPECIFIED_FAILURE")]
    UnspecifiedFailure,
    #[serde(rename = "COLD")]
    Cold,
}

#[derive(Debug, Deserialize)]
pub enum PluggedStatus {
    #[serde(rename = "UNPLUGGED")]
    Unplugged,
    #[serde(rename = "AC")]
    Ac,
    #[serde(rename = "USB")]
    Usb,
    #[serde(rename = "WIRELESS")]
    Wireless,
}

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "CHARGING")]
    Charging,
    #[serde(rename = "DISCHARGING")]
    Discharging,
    #[serde(rename = "FULL")]
    Full,
    #[serde(rename = "NOT_CHARGING")]
    NotCharging,
}

#[derive(Debug, Deserialize)]
pub struct BatteryStatus {
    pub health: HealthStatus,
    pub percentage: u8,
    pub plugged: PluggedStatus,
    pub status: Status,
    pub temperature: f32,
    pub current: i32,
}

pub struct TermuxBatteryStatus {}
impl TermuxBatteryStatus {
    pub fn new() -> Self {
        TermuxBatteryStatus {}
    }
    pub fn run(&self) -> Result<BatteryStatus, TermuxError> {
        let mut command = Command::new("termux-battery-status");
        let output = command.output();
        match output {
            Ok(output) => {
                if output.status.success() {
                    let battery_status: BatteryStatus =
                        serde_json::from_slice(&output.stdout).unwrap();

                    return Ok(battery_status);
                }
                Err(TermuxError::Output(output.to_owned()))
            }
            Err(e) => Err(TermuxError::IOError(e)),
        }
    }
}
