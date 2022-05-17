use std::path::PathBuf;
use clap::Parser;

use crate::cli::verbosity;

use super::verbosity::VerbosityLevel;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
	/// The verbosity level, valid values are: Debug, Info, Warn and Error
	#[clap(short, long)]
	verbosity: Option<String>,

	/// The config file path
	#[clap(short, long, parse(from_os_str))]
	config_path: Option<PathBuf>,
}

impl Args {
	pub fn get_config_path(&self) -> PathBuf {
		let args = self::Args::parse();

		match args.config_path {
			Some(_) => args.config_path.unwrap(),
			None => PathBuf::from("/etc/ruhroh.conf"),
		}
	}

	pub fn get_verbosity_level(&self) -> Result<VerbosityLevel, ()> {
		let args = self::Args::parse();

		if args.verbosity.is_some() {
			let result = verbosity::get_verbosity_level(args.verbosity.unwrap());

			match result {
				Ok(_) => Ok(result.unwrap()),
				Err(()) => Err(()),
			}
		} else {
			Ok(verbosity::DEFAULT_VERB_LEVEL)
		}
	}
}
