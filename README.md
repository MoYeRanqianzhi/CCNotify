# CCNotify

[中文文档](README_CN.md)

Cross-platform notification tool for [Claude Code](https://docs.anthropic.com/en/docs/claude-code). Sends a native OS notification (with sound) when Claude Code finishes a task.

**133KB** binary. No runtime dependencies.

## AI Agent Quick Setup

Give the following to any of your AI coding agents to auto-configure CCNotify:

```
Read https://github.com/MoYeRanqianzhi/CCNotify/blob/master/docs/AGENT_INSTALL.md and follow the instructions to install and configure CCNotify for Claude Code.
```

## Features

| Version | Size | Description |
|---------|------|-------------|
| `ccnotify` | ~133KB | System notification with default OS sound |
| `ccnotify-sound` | ~150KB | + Custom audio file playback via `--sound` |

### Platform Support

| Platform | Notification | System Sound | Custom Audio |
|----------|-------------|-------------|-------------|
| Windows 10/11 | Toast (WinRT) | `ms-winsoundevent` | winmm MCI (MP3/WAV/WMA) |
| macOS | `osascript` | Glass | `afplay` |
| Linux | `notify-send` | `canberra-gtk-play` | `paplay` / `aplay` |

## Installation

### Pre-built Binaries

Download from [Releases](https://github.com/MoYeRanqianzhi/CCNotify/releases).

### From Source

```bash
git clone https://github.com/MoYeRanqianzhi/CCNotify.git
cd CCNotify

# Notification only
cargo build --release

# With sound support
cargo build --release --features sound
```

Output: `target/release/ccnotify(.exe)`

## Usage

```bash
# Default notification
ccnotify

# Custom title and body
ccnotify -t "Build Done" -b "All tests passed"

# With custom sound (sound version only)
ccnotify -s "/path/to/sound.mp3"
ccnotify -t "Done" -b "Task finished" -s "/path/to/sound.mp3"
```

## Claude Code Hook Setup

Add to `~/.claude/settings.json`:

```jsonc
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "command": "$HOME/.claude/notify/ccnotify",
            "type": "command"
          }
        ],
        "matcher": ""
      }
    ]
  }
}
```

Or with custom sound:

```jsonc
{
  "hooks": {
    "Stop": [
      {
        "hooks": [
          {
            "command": "$HOME/.claude/notify/ccnotify -s $HOME/.claude/notify/sound.mp3",
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

[MIT](LICENSE)
