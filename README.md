# Player Control

MacOS-like control centre player for Arch Linux. Control system-wide media playback via TUI.

## Features
- List active media sources (MPRIS)
- Play/Pause, Next/Previous track navigation
- Browser tab deduplication (hides main browser instance if tab extension handles the track)
- Stable UI list sorting and selection tracking across state refreshes
- TUI interface (Ratatui)
- Hyprland + Waybar integration script

## Controls
- `↑/↓`: Navigate players
- `Space`: Play/Pause
- `n`: Next track
- `p`: Previous track
- `q`: Quit

## Installation

### Prerequisites
- Rust (cargo)
- `dbus` system libraries (for Linux)
- `playerctl` (for backend MPRIS parsing)

### Build
```bash
cargo build --release
```

## Waybar & Hyprland Integration

A toggle script is included for launching the TUI as a floating window from Waybar.

**1. Hyprland Rules (`~/.config/hypr/hyprland.conf`)**
```conf
windowrulev2 = float, class:^(player-control-tui)$
windowrulev2 = size 800 400, class:^(player-control-tui)$
windowrulev2 = center, class:^(player-control-tui)$
windowrulev2 = animation popin 80%, class:^(player-control-tui)$
```

**2. Waybar Module (`~/.config/waybar/config.jsonc`)**
```json
"custom/player-control": {
    "format": "󰎆",
    "tooltip": true,
    "tooltip-format": "Player Control TUI",
    "on-click": "/path/to/player-control/scripts/waybar-toggle.sh",
    "return-type": "text"
}
```

## Architecture
Uses a backend abstraction to support development on non-Linux platforms:
- `MprisBackend`: Native Linux MPRIS2 integration.
- `MockBackend`: Development mock (auto-active on non-Linux).
