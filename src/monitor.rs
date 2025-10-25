// This file contains the implementation for monitoring the last executed shell command and the current working directory.

use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

/// Reads the last executed shell command from the specified file.
pub fn read_last_command() -> io::Result<String> {
    let path = PathBuf::from("/tmp/ghostty_last_cmd");
    let mut command = String::new();
    fs::File::open(path)?.read_to_string(&mut command)?;
    Ok(command.trim().to_string())
}

/// Retrieves the current working directory.
pub fn get_current_directory() -> io::Result<String> {
    let cwd = std::env::current_dir()?;
    Ok(cwd.to_string_lossy().to_string())
}

/// Gets both the last command and current working directory.
pub fn get_last_command_and_cwd() -> io::Result<(String, String)> {
    let command = read_last_command().unwrap_or_else(|_| "unknown".to_string());
    let cwd = get_current_directory().unwrap_or_else(|_| "unknown".to_string());
    Ok((command, cwd))
}