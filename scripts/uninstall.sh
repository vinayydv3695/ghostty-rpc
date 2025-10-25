#!/bin/bash

# Stop the Ghostty RPC service
systemctl --user stop ghostty-rpc.service

# Disable the Ghostty RPC service
systemctl --user disable ghostty-rpc.service

# Remove the binary
rm -f /usr/local/bin/ghostty-rpc

# Remove the configuration directory
rm -rf ~/.config/ghostty-rpc

echo "Ghostty RPC has been uninstalled successfully."