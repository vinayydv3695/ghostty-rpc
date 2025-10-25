# Ghostty RPC - Discord Rich Presence Integration

Ghostty RPC is a Rust-based application that integrates with Discord's Rich Presence feature, providing real-time updates based on the user's terminal activity in the Ghostty terminal. This project continuously updates your Discord status with the last executed shell command, current working directory, and session start time.

## Features

- **Real-time Updates**: Automatically updates your Discord status with the last executed command and current working directory.
- **Customizable**: Easily configurable through a TOML file.
- **Systemd Integration**: Can be run as a user-level service for seamless operation.
- **Cross-Platform**: Designed to work on various operating systems.

## Installation

### AUR (Arch User Repository)

You can install Ghostty RPC from the AUR. Use an AUR helper like `yay` or `paru`:

```bash
yay -S ghostty-rpc
```

### Manual Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/vinayydv3695/ghostty-rpc.git
   cd ghostty-rpc
   ```

2. Build the project:

   ```bash
   cargo build --release
   ```

3. Install the binary:

   ```bash
   sudo cp target/release/ghostty-rpc /usr/local/bin/
   ```

4. Copy the systemd service file:

   ```bash
   cp assets/ghostty-rpc.service ~/.config/systemd/user/
   ```

5. Enable and start the service:

   ```bash
   systemctl --user enable ghostty-rpc.service
   systemctl --user start ghostty-rpc.service
   ```

## CLI Usage

You can run the application with various options:

```bash
ghostty-rpc --config <path> --debug --once --interval <secs>
```

### Options

- `--config <path>`: Specify the path to the configuration file.
- `--debug`: Enable debug logging.
- `--once`: Run the application once and exit.
- `--interval <secs>`: Set the refresh interval for updating Discord presence.

## Service Management

To manage the Ghostty RPC service, use the following commands:

- Start the service:

  ```bash
  systemctl --user start ghostty-rpc.service
  ```

- Stop the service:

  ```bash
  systemctl --user stop ghostty-rpc.service
  ```

- Check the status of the service:

  ```bash
  systemctl --user status ghostty-rpc.service
  ```

## Configuration

The configuration file is located at `~/.config/ghostty-rpc/config.toml`. An example configuration file is provided in `assets/config.toml.example`.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Versioning

Current version: 1.0.0

## Badges

[![Build Status](https://img.shields.io/github/workflow/status/yourusername/ghostty-rpc/CI)](https://github.com/yourusername/ghostty-rpc/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

For more information, please refer to the documentation in the respective files.
