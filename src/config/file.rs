use log::debug;
use serde::Deserialize;
use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind},
    path::PathBuf,
};

#[derive(Deserialize)]
pub struct Config {
    /// Devices can be ignored from being detected and used in logid.
    /// To ignore a device, create a field called ignore as an array
    /// of the PIDs of the devices you want to ignore.
    pub ignored_devices: Option<Vec<String>>,

    /// Devices are defined in an array field called devices.
    /// This array consists of objects that define a device.
    pub devices: Option<Vec<Device>>,

    /// The config version to use, if not specified then it
    /// is assumed that it is 1.
    pub config_version: Option<u32>,
}

#[derive(Deserialize)]
pub struct Device {
    /// This is a required string field that defines the name of the device.
    /// To get the name of the device, launch logid with the device connected
    /// and it should print out a message with the device name.
    pub name: Option<String>,
    /// This is an 8bit Integer field that defines the SmartShift settings for
    /// a mouse that supports it.
    pub smartshift: Option<u8>,
    /// This is an object field that defines the HiRes mouse-scrolling settings
    /// for a device that supports it.
    pub hires_scroll: Option<HiResScroll>,
    /// This is an optional integer/array field that defines the DPI for a
    /// mouse that supports adjustable DPI. `(e.g. dpi: 1000;)`
    /// If your mouse has multiple sensors, you may define separate DPIs
    /// for those sensors by using an array and placing the value in the
    /// sensor's index
    pub dpi: Option<u32>,
    /// This is an optional array field that defines the mappings for buttons.
    pub buttons: Option<Vec<Button>>,
}

#[derive(Deserialize)]
pub struct HiResScroll {
    /// This is an optional boolean field that defines whether the mouse wheel
    /// should be high resolution or not.
    pub enabled: Option<bool>,
    /// This is an optional boolean field that defines whether to invert the
    /// mouse wheel.
    pub invert: Option<bool>,
    /// This is an optional boolean field that defines whether mousewheel
    /// events should send as an HID++ notification or work normally
    /// (true for HID++ notification, false for normal usage). This option must
    /// be set to true to remap the scroll wheel action.
    pub target: Option<bool>,
}

#[derive(Deserialize)]
pub struct Button {
    /// This is a required integer field that defines the Control ID of
    /// the button that is being remapped. (e.g. cid: 0xc4;)
    pub cid: Option<String>,

    /// This is a required object field that defines the new action of
    /// the button. (e.g. action: { ... }; )
    pub action: Option<Action>,
}

#[derive(Deserialize)]
pub struct Action {
    /// This is a required string field that defines the type of action.
    /// (e.g. type: "None";). The following is a list of possible actions
    /// with their additional fields.
    r#type: Option<ActionType>,
}

#[derive(Deserialize)]
pub enum ActionType {
    Gesture,
    Keypress,
    None,
    ToggleSmartShift,
    ToggleHiresScroll,
    CycleDPI,
    ChangeDPI,
    ChangeHost,
}

impl Config {
    pub fn from_path(path: PathBuf) -> Result<Config, Error> {
        debug!("Opening file from path {:?}", path.as_os_str());
        let file = File::open(path);

        if let Ok(config_file) = file {
            debug!("Loading configuration");
            let reader = BufReader::new(config_file);
            let config = serde_json::from_reader(reader)?;

            Ok(config)
        } else {
            debug!("The specified path is invalid, falling back to default.");

            let msg = file
                .err()
                .unwrap_or_else(|| Error::new(ErrorKind::Other, "Error unwrapping failed"));

            Err(msg)
        }
    }
}
