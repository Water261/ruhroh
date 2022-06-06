use crate::config::file;

pub enum ConfigVersion {
	V1(file::Config),
}

pub enum DeviceVersion {
	V1(file::Device),
}

pub enum HiResScrollVersion {
	V1(file::HiResScroll),
}

pub enum ButtonVersion {
	V1(file::Button),
}

pub enum ActionVersion {
	V1(file::Action)
}
