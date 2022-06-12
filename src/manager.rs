use std::io::Error;

use crate::config::file::Device;

pub struct DeviceManager {
	device: Device
}

impl DeviceManager {
	pub fn new(device: Device) -> Self {
		DeviceManager { device: device }
	}

	pub fn start() -> Result<(), Error> {
		todo!()
	}

	pub fn stop() -> Result<(), Error> {
		todo!()
	}
}