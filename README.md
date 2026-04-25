# CCNotify

Cross-platform notification tool for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Sends a native OS notification when Claude Code finishes a task.

**133KB** binary. No runtime dependencies.

## Features

| Version | Binary | Size | Description |
|---------|--------|------|-------------|
| `ccnotify` | `ccnotify.exe` | 133KB | System notification with default sound |
| `ccnotify-sound` | `ccnotify-sound.exe` | 150KB | + Custom audio file playback via `--sound` |

### Platform Support

| Platform | Notification | System Sound | Custom Audio |
|----------|-------------|-------------|-------------|
| Windows 10/11 | Toast (WinRT) | `ms-winsoundevent` | winmm MCI (MP3/WAV/WMA) |
| macOS | `osascript` | Glass | `afplay` |
| Linux | `notify-send` | `canberra-gtk-play` | `paplay` / `aplay` |

## Installation

### From Source

```bash
# Notification only
cargo build --release

# With sound support
cargo build --release --features sound
```

Binary output: `target/release/ccnotify(.exe)`

### Pre-built

Download from [Releases](https://github.com/user/ccnotify/releases).

## Usage

```bash
# Default notification
ccnotify

# Custom title and body
ccnotify -t "Build Done" -b "All tests passed"

# With custom sound (sound version only)
ccnotify-sound -t "Done" -b "Task finished" -s "/path/to/sound.mp3"
```

## Claude Code Hook Configuration

Add to `~/.claude/settings.json`:

```jsonc
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            // Notification only
            "command": "$HOME/.claude/notify/ccnotify",
            // Or with custom sound:
            // "command": "$HOME/.claude/notify/ccnotify-sound -s $HOME/.claude/notify/sound.mp3",
            "type": "command"
          }
        ],
        "matcher": ""
      }
    ]
  }
}
```

> **Note:** Use `$HOME` (not `%USERPROFILE%`) — Claude Code hooks run via bash.

## CLI Reference

```
ccnotify [OPTIONS]

OPTIONS:
  -t, --title <TITLE>  Notification title (default: "Claude Code")
  -b, --body <BODY>    Notification body (default: "Task completed")
  -s, --sound <PATH>   Play audio file (sound version only)
  -h, --help           Show help
  -V, --version        Show version
```

## License

MIT
