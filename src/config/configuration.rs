use crate::config::action;

pub enum ConfigVersion {
	V1(ConfigV1)
}

pub enum DeviceVersion {
	V1(DeviceV1)
}

pub enum ButtonMappingVersion {
	V1(ButtonMappingV1)
}

pub struct DeviceV1 {
	/// The device's name
	/// For example, a MX Master 3 would be Wireless Mouse MX Master 3
	name: String,
	/// The smartshift threshold, 0 will disable it
	smartshift: u8,
	/// The DPI setting
	dpi: u32,
	/// The button mappings
	buttons: Vec<ButtonMappingV1>,
}

pub struct ConfigV1 {
	devices: Vec<DeviceV1>,
}

pub struct ButtonMappingV1 {
	/// The control id of the button
	control_id: String,
	/// The action to perform
	action: action::ActionTypeV1
}