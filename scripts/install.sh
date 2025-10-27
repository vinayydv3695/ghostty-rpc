#!/bin/bash

# Build the Rust project
cargo build --release

# Copy the binary to /usr/local/bin
sudo cp target/release/ghostty-rpc /usr/local/bin/

# Copy the systemd service file to the appropriate directory
sudo cp ../assets/ghostty-rpc.service /etc/systemd/user/

# Enable the systemd service for the user
systemctl --user enable ghostty-rpc.service

# Start the systemd service
systemctl --user start ghostty-rpc.service

echo "Ghostty RPC has been installed and the service is running."