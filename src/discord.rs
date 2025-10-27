// src/discord.rs

use discord_presence::Client;
use std::time::SystemTime;
use crate::config::Config;

/// Struct to manage the Discord Rich Presence connection.
pub struct DiscordRpc {
    client: Client,
    large_image: String,
    small_image: String,
}

impl DiscordRpc {
    /// Creates a new instance of DiscordRpc.
    pub fn new(config: &Config) -> Self {
    // Replace with your actual Discord App ID
    let mut client = Client::new(1429846275737518222);
    // Start the client connection so `set_activity` can be used.
    // Different versions of the `discord-presence` crate have differing
    // signatures for `start()` (some return `()` while others return a
    // `Result`). Discard the return value to be compatible with both.
    let _ = client.start();
        Self {
            client,
            large_image: config.general.large_image.clone(),
            small_image: config.general.small_image.clone(),
        }
    }

    /// Updates the Discord Rich Presence with the given command and directory.
    pub fn update_presence(&mut self, command: &str, cwd: &str) {
        let start_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let command = command.to_string();
        let cwd = cwd.to_string();
        let large_image = self.large_image.clone();
        let small_image = self.small_image.clone();

        if let Err(e) = self.client.set_activity(|activity| {
            activity
                .state(&command)
                .details(&cwd)
                .assets(|assets| {
                    assets
                        .large_image(&large_image)
                        .small_image(&small_image)
                })
                .timestamps(|timestamps| timestamps.start(start_time))
        }) {
            eprintln!("Failed to update activity: {}", e);
        }
    }
}