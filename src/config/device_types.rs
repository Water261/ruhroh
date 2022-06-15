use serde::Deserialize;
use std::fmt::Display;

#[derive(Deserialize)]
pub enum DeviceTypes {
    Mouse,
    Keyboard,
}

impl Display for DeviceTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceTypes::Mouse => write!(f, "Mouse"),
            DeviceTypes::Keyboard => write!(f, "Keyboard"),
        }
    }
}
