use std::fmt;
use serde::Deserialize;

/// Since sometimes people may not include their own configuration file we
/// need to make a default configuration. Since we don't know the exact
/// device names they may use, we use these catch all types
pub enum SpecialDeviceTypes {
	Mouse,
	Keyboard,
}

impl fmt::Display for SpecialDeviceTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
			SpecialDeviceTypes::Mouse => write!(f, "Mouse"),
            SpecialDeviceTypes::Keyboard => write!(f, "Keyboard"),
		}
    }
}

#[derive(Deserialize)]
pub struct Config {
	pub ignored_devices: Vec<String>,
	pub devices: Vec<Device>,
}

#[derive(Deserialize)]
pub struct Device {
	/// This is a required string field that defines the name of the device. 
	/// To get the name of the device, launch logid with the device connected
	/// and it should print out a message with the device name. 
	/// `(e.g. name: "MX Master";)`
	pub name: String,
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
	pub enabled: bool,
	/// This is an optional boolean field that defines whether to invert the 
	/// mouse wheel.
	pub invert: bool,
	/// This is an optional boolean field that defines whether mousewheel 
	/// events should send as an HID++ notification or work normally 
	/// (true for HID++ notification, false for normal usage). This option must
	/// be set to true to remap the scroll wheel action.
	pub target: bool,
}