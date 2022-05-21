use std::path::PathBuf;
use clap::Parser;
use log::{info, warn, error};
use crate::config::configuration::{Config, Device, SpecialDeviceTypes};
use crate::config::config_file::{load_configuration, get_default_config};

mod config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
	/// The config file path
	#[clap(short, long, parse(from_os_str))]
	pub config_path: Option<PathBuf>,
}

const DEFAULT_CONFIG_PATH: &str = "/etc/ruhroh.conf";

fn main() {
	let args = CliArgs::parse();
	let config_path = args.config_path.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

	info!("Loading configuration file {:?}", config_path.to_str());
	let config = load_configuration(config_path).unwrap_or_else(|_| get_default_config());
}
