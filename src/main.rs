use clap::Parser;
use crate::cli::args;

mod cli;

fn main() {
	let args = args::Args::parse();
	let config_path = args::Args::get_config_path(&args);
	let verbosity_lvl = args::Args::get_verbosity_level(&args);
}
