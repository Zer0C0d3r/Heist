# Heist Live Tracking Shell Integration
# Add this to your ~/.bashrc or ~/.zshrc to enable real-time history tracking for Heist

export HEIST_LIVE_HISTORY="$HOME/.heist_live_history"

heist_live_track() {
  # Only log if not a duplicate of the last command
  if [ -n "$BASH_COMMAND" ]; then
    if [ ! -f "$HEIST_LIVE_HISTORY" ] || [ "$(tail -n 1 "$HEIST_LIVE_HISTORY" 2>/dev/null)" != "$BASH_COMMAND" ]; then
      printf "%s|%s\n" "$(date +'%Y-%m-%dT%H:%M:%S%z')" "$BASH_COMMAND" >> "$HEIST_LIVE_HISTORY"
    fi
  fi
}

# For Bash
if [ -n "$BASH_VERSION" ]; then
  PROMPT_COMMAND="heist_live_track; $PROMPT_COMMAND"
fi
# For Zsh
if [ -n "$ZSH_VERSION" ]; then
  precmd_functions+=(heist_live_track)
fi
