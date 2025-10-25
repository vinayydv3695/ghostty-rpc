// src/main.rs

use clap::{Command, Arg};
use env_logger::Env;
use log::{error, info};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod config;
mod discord;
mod monitor;

fn main() {
    // Initialize logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Set up command-line interface
    let matches = Command::new("Ghostty RPC")
        .version("1.0.0")
        .about("Updates Discord Rich Presence with Ghostty terminal information")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Path to the configuration file"),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .action(clap::ArgAction::SetTrue)
                .help("Enable debug logging"),
        )
        .arg(
            Arg::new("once")
                .short('o')
                .long("once")
                .action(clap::ArgAction::SetTrue)
                .help("Run once and exit"),
        )
        .arg(
            Arg::new("interval")
                .short('i')
                .long("interval")
                .value_name("SECONDS")
                .help("Update interval in seconds"),
        )
        .get_matches();

    // Load configuration
    let config = if let Some(config_path) = matches.get_one::<String>("config") {
        config::load_config(PathBuf::from(config_path)).unwrap_or_else(|err| {
            error!("Failed to load config: {}", err);
            std::process::exit(1);
        })
    } else {
        config::Config::load().unwrap_or_else(|err| {
            error!("Failed to load config: {}", err);
            std::process::exit(1);
        })
    };

    // Initialize Discord RPC
    let mut discord_rpc = discord::DiscordRpc::new(&config);

    // Daemon loop
    let interval = matches
        .get_one::<String>("interval")
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(config.general.refresh_interval);

    loop {
        // Read last command and current working directory
        let (last_command, cwd) = monitor::get_last_command_and_cwd().unwrap_or_else(|err| {
            error!("Failed to read last command or cwd: {}", err);
            ("unknown".to_string(), "unknown".to_string())
        });

        // Update Discord presence
        discord_rpc.update_presence(&last_command, &cwd);

        // Log the update
        info!("Updated Discord presence: Command: '{}', CWD: '{}'", last_command, cwd);

        // Check if running once
        if matches.get_flag("once") {
            break;
        }

        // Sleep for the specified interval
        thread::sleep(Duration::from_secs(interval));
    }
}