use std::{path::PathBuf, process::exit, env, str::FromStr};
use clap::Parser;
use log::{info, error, Level};
use crate::config::file::Config;

mod config;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
	/// The config file path
	#[clap(short, long, parse(from_os_str))]
	pub config_path: Option<PathBuf>,

	/// Verbosity, default is error
	#[clap(short, long)]
	pub verbosity: Option<String>,
}

const DEFAULT_CONFIG_PATH: &str = "/etc/ruhroh.conf";
const DEFAULT_VERB_LVL: &str = "trace"; // TODO: Set to warn on release

#[tokio::main]
async fn main() {
	let args = CliArgs::parse();
	let config_path = args.config_path.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
	let verbosity_level = args.verbosity.unwrap_or_else(|| String::from(DEFAULT_VERB_LVL));
	let log_level = Level::from_str(verbosity_level.as_str()).unwrap_or_else(|_| {
		println!("Invalid verbosity level");
		println!("Valid levels are: trace, debug, info, warn, error");
		exit(1);
	});
	simple_logger::init_with_level(log_level).unwrap();

	info!("Loading configuration file {:?}", config_path.to_str());
	let config = Config::from_path(config_path).unwrap_or_else(|_| {
		error!("Failed to load config, changing to default.");
		Config::default()
	});

	for device in config.devices {
		
	}
}
