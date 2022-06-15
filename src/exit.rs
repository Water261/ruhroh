#[derive(Clone, Copy)]
pub enum ExitCode {
    /// The logger failed to initialise
    LoggerInitialisationFail = 1,
    /// The configuration file failed to load
    ConfigurationLoadFail = 2,
    /// There aren't any devices in config file
    /// so we don't need to do anything
    NoDevices = 3,
    /// Failed to start device manager
    DMStartFail = 4,
    /// Failed to stop the device manager
    /// So we just let the kernel clean up
    DMStopFail = 5,
}

impl ExitCode {
    pub fn to_i32(self) -> i32 {
        let exit_code = &self;

        *exit_code as i32
    }
}
