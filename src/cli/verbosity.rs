pub enum VerbosityLevel {
	Debug,
	Info,
	Warn,
	Error
}

pub const DEFAULT_VERB_LEVEL: VerbosityLevel = VerbosityLevel::Debug;

pub fn get_verbosity_level(verbosity_string: String) -> Result<VerbosityLevel, ()> {
	let ver_as_str = verbosity_string.as_str();

	match ver_as_str {
		"debug" => Ok(VerbosityLevel::Debug),
		"info" => Ok(VerbosityLevel::Info),
		"warn" => Ok(VerbosityLevel::Warn),
		"error" => Ok(VerbosityLevel::Error),
		_ => Err(())
	}
}