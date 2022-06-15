use crate::{config::file::Config, exit::ExitCode, manager::DeviceManager};
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use log::{debug, error, info, warn};
use std::{path::PathBuf, process::exit, sync::mpsc::channel};

mod config;
mod exit;
mod manager;

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
    let config_path = args
        .config_path
        .unwrap_or_else(|| PathBuf::from(DEFAULT_CONFIG_PATH));
    let verbosity_level = args.verbosity.log_level();

    init_logger(verbosity_level);

    info!("Loading configuration file {:?}", config_path.to_str());
    let config = load_configuration(config_path);

	info!("Getting the specified devices");
    let devices = config.devices.unwrap_or_else(|| {
        warn!("No devices specified, exiting");
        exit(ExitCode::NoDevices.to_i32());
    });

	info!("Setting up the device manager");
    let manager = setup_device_manager(devices);

	info!("Setting up the quit handler");
    setup_quit_handler();

    info!("Stopping the device manager");
    manager.stop().unwrap_or_else(|_| {
        error!("Failed to stop the device manager, exiting and letting the kernel clean up");
        exit(ExitCode::DMStopFail.to_i32());
    });

	info!("Exiting")
}

fn init_logger(verbosity_level: Option<log::Level>) {
    if let Some(log_level) = verbosity_level {
        simple_logger::init_with_level(log_level).unwrap_or_else(|_| {
            println!("Failed to initialise logger, exiting.");
            exit(ExitCode::LoggerInitialisationFail.to_i32());
        });
        info!("Initialied logger with level {}", log_level.as_str());
    }
}

fn load_configuration(config_path: PathBuf) -> Config {
    Config::from_path(config_path).unwrap_or_else(|_| {
        error!("Failed to load config, changing to default.");
        exit(ExitCode::ConfigurationLoadFail.to_i32());
    })
}

fn setup_device_manager(devices: Vec<config::file::Device>) -> DeviceManager {
    let manager = DeviceManager::new(Some(devices));
    manager.start().unwrap_or_else(|_| {
        error!("Failed to start DeviceManager");
        exit(ExitCode::DMStartFail.to_i32());
    });
    manager
}

fn setup_quit_handler() {
    let (tx, rx) = channel::<()>();
    ctrlc::set_handler(move || {
        tx.send(()).expect("Couldn't send Quit signal");
    })
    .expect("Failed setting the Quit Handler");
    rx.recv().expect("Failed to recieve message");
}
