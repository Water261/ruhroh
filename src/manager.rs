use std::io::Error;

use crate::config::file::Device;

pub struct DeviceManager {
    devices: Vec<Device>,
}

impl DeviceManager {
    pub fn new(devices: Option<Vec<Device>>) -> Self {
        DeviceManager {
            devices: devices.unwrap_or_default(),
        }
    }

    pub fn add_device(&mut self, device: Device) {
        self.devices.push(device);
    }

    pub fn start(&self) -> Result<(), Error> {
        Ok(())
    }

    pub fn stop(&self) -> Result<(), Error> {
        Ok(())
    }
}
