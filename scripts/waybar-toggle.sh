#!/usr/bin/env bash

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/.."
BIN="$DIR/target/release/player-control"
CLASS="player-control-tui"

# Check if window exists in Hyprland
if hyprctl clients -j | grep -q "\"class\": \"$CLASS\""; then
    hyprctl dispatch closewindow "class:^($CLASS)$"
else
    # Build if binary missing
    if [ ! -f "$BIN" ]; then
        cd "$DIR" && cargo build --release
    fi

    # Launch in available terminal
    if command -v kitty >/dev/null; then
        kitty --class "$CLASS" -e "$BIN" &
    elif command -v foot >/dev/null; then
        foot --app-id "$CLASS" "$BIN" &
    elif command -v alacritty >/dev/null; then
        alacritty --class "$CLASS","$CLASS" -e "$BIN" &
    else
        notify-send "Error" "Kitty, Foot, or Alacritty not found."
    fi
fi
