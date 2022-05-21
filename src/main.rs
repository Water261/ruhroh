use std::path::PathBuf;
use clap::Parser;
use config::configuration::Config;
use crate::config::config_file;

mod config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
	/// The config file path
	#[clap(short, long, parse(from_os_str))]
	pub config_path: Option<PathBuf>,
}

const DEFAULT_CONFIG_PATH: &str = "/etc/ruhroh.conf";
const DEFAULT_CONFIG: Config = Config {

};

fn main() {
	let args = CliArgs::parse();
	let config_path = args.config_path.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));

	let config = config_file::load_configuration(config_path).unwrap_or_else(|_| DEFAULT_CONFIG);
}
