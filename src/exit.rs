#[derive(Clone, Copy)]
pub enum ExitCode {
	LoggerInitialisationFail = 1,
	ConfigurationLoadFail = 2,
	NoDevices = 3,
}

impl ExitCode {
	pub fn to_i32(&self) -> i32 {
		*self as i32
	}
}