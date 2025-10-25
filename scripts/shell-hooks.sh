#!/bin/bash
# This script sets up a shell hook for bash/zsh to capture the last executed command.

# Check if the directory for the temp file exists
if [ ! -d /tmp ]; then
  mkdir -p /tmp
fi

# Set the PROMPT_COMMAND to capture the last command executed
PROMPT_COMMAND='echo "$(history 1 | sed "s/ *[0-9]* *//")" > /tmp/ghostty_last_cmd' 

# Add instructions to the user
echo "Shell hook installed. The last executed command will be saved to /tmp/ghostty_last_cmd."