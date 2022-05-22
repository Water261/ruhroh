use std::{path::PathBuf, fs::File, io::{BufReader, Error}};
use serde::Deserialize;
use crate::config::device_types::DeviceTypes;

#[derive(Deserialize)]
pub struct Config {
	pub ignored_devices: Option<Vec<String>>,
	pub devices: Vec<Device>,
}

impl Config {
	pub fn new(path: PathBuf) -> Result<Config, Error> {
		let file = File::open(path);

		if let Ok(config_file) = file {
			let reader = BufReader::new(config_file);
			let config = serde_json::from_reader(reader)?;
	
			Ok(config)
		} else {
			Ok(Config::default())
		}
	}

	pub fn default() -> Config {
		Config {
			ignored_devices: Some(Vec::new()),
			devices: vec![
				Device {
					name: Some(DeviceTypes::Mouse.to_string()),
					is_generic: Some(true),
					smartshift: Some(30),
					hires_scroll: Some(HiResScroll {
						enabled: Some(false),
						invert: Some(false),
						target: Some(false),
					}),
					dpi: Some(1000),
				}
			]
		}
	}
}

#[derive(Deserialize)]
pub struct Device {
	/// This is a required string field that defines the name of the device. 
	/// To get the name of the device, launch logid with the device connected
	/// and it should print out a message with the device name. 
	/// `(e.g. name: "MX Master";)`
	pub name: Option<String>,
	/// WARNING! Do not set this in your configuration file
	/// This just indicates to ruhroh that it is a generic device and
	/// the settings should apply to everything
	pub is_generic: Option<bool>,
	/// This is an 32bit Integer field that defines the SmartShift settings for
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
	/// `(e.g. sensor 0: 1000 dpi, sensor 1: 800 dpi -> dpi: [1000, 800])`
	pub dpi: Option<u32>
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
