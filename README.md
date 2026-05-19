# Player Control

MacOS-like control centre player for Arch Linux. Control system-wide media playback via TUI.

## Features
- List active media sources (MPRIS)
- Play/Pause toggle
- Next/Previous track navigation
- TUI interface (Ratatui)
- Waybar integration compatible

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

### Build
```bash
cargo build --release
```

## Architecture
Uses a backend abstraction to support development on non-Linux platforms:
- `MprisBackend`: Native Linux MPRIS2 integration.
- `MockBackend`: Development mock (auto-active on non-Linux).

## Waybar Integration
Example configuration:
```json
"custom/player": {
    "exec": "player-control",
    "format": " {}",
    "on-click": "alacritty -e player-control"
}
```
