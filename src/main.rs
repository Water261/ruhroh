use std::{path::PathBuf, process::exit};
use clap::Parser;
use log::{info, error};
use clap_verbosity_flag::{Verbosity, InfoLevel};
use crate::config::file::Config;
use crate::exit::ExitCode;

mod config;
mod exit;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
	/// The config file path
	#[clap(short, long, parse(from_os_str))]
	pub config_path: Option<PathBuf>,

	#[clap(flatten)]
	pub verbosity: Verbosity<InfoLevel>,
}

const DEFAULT_CONFIG_PATH: &str = "/etc/ruhroh.conf";

#[tokio::main]
async fn main() {
	let args = CliArgs::parse();
	let config_path = args.config_path.unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
	let verbosity_level = args.verbosity.log_level();

	if let Some(log_level) = verbosity_level {
		simple_logger::init_with_level(log_level).unwrap_or_else(|_| {
			println!("Failed to initialise logger, exiting.");
			exit(ExitCode::LoggerInitialisationFail.to_i32());
		});
		info!("Initialied logger with level {}", log_level.as_str());
	}

	info!("Loading configuration file {:?}", config_path.to_str());
	let config = Config::from_path(config_path).unwrap_or_else(|_| {
		error!("Failed to load config, changing to default.");
		exit(ExitCode::ConfigurationLoadFail.to_i32());
	});
}
