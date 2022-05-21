use std::{path::{PathBuf}, fs::File, io::BufReader, error::Error};
use crate::config::configuration::Config;

pub fn load_configuration(path: PathBuf) -> Result<Config, Box<dyn Error>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);

	let config = serde_json::from_reader(reader)?;

	Ok(config)
}

pub fn get_default_config() -> Config {
	todo!();
	// Config {
	// 	devices: vec![
	// 		Device {
	// 			name: SpecialDeviceTypes::Mouse.to_string(),
	// 		},
	// 		Device {
	// 			name: SpecialDeviceTypes::Keyboard.to_string(),
	// 		}
	// 	],
	// }
}