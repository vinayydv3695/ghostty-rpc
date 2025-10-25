// src/config.rs

use serde::Deserialize;
use std::fs;
use std::path::PathBuf;
use directories::ProjectDirs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
}

#[derive(Debug, Deserialize)]
pub struct GeneralConfig {
    pub refresh_interval: u64,
    pub show_directory: bool,
    pub exclude: Vec<String>,
    pub large_image: String,
    pub small_image: String,
}

impl Config {
    /// Loads the configuration from the specified TOML file.
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = get_config_path()?;
        let config_content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_content)?;
        Ok(config)
    }
}

/// Loads the configuration from a specific path.
pub fn load_config(path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

/// Gets the path to the configuration file.
fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let project_dirs = ProjectDirs::from("com", "your_name", "ghostty-rpc")
        .ok_or("Unable to find project directories")?;
    let config_dir = project_dirs.config_dir();
    let config_file = config_dir.join("config.toml");

    // Create the config directory if it doesn't exist
    if !config_dir.exists() {
        std::fs::create_dir_all(config_dir)?;
    }

    Ok(config_file)
}