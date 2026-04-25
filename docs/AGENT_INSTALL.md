# CCNotify Installation Guide (for AI Agents)

You are configuring CCNotify — a native OS notification tool for Claude Code.
When Claude Code finishes a task, it will show a toast/notification with sound.

## Step 1: Detect platform

Determine the user's OS and architecture:
- Windows x86_64: `x86_64-pc-windows-msvc`
- macOS Intel: `x86_64-apple-darwin`
- macOS Apple Silicon: `aarch64-apple-darwin`
- Linux x86_64: `x86_64-unknown-linux-gnu`

## Step 2: Download the binary

> **IMPORTANT**: The download URLs below will be populated after the first release is published.
> Check https://github.com/MoYeRanqianzhi/CCNotify/releases/latest for actual download links.

Download the appropriate binary for the user's platform from the latest GitHub release.

There are two versions per platform:
- `ccnotify` — notification with system default sound (smaller)
- `ccnotify-sound` — notification + custom audio file playback via `--sound` flag (slightly larger)

Ask the user which version they prefer. Default to `ccnotify-sound` for maximum flexibility.

## Step 3: Install the binary

```bash
mkdir -p "$HOME/.claude/notify"
# Move/copy the downloaded binary to:
# $HOME/.claude/notify/ccnotify      (or ccnotify.exe on Windows)
chmod +x "$HOME/.claude/notify/ccnotify"
```

## Step 4: Configure Claude Code hook

Read the user's `~/.claude/settings.json`. If a `hooks` key exists, merge into it. Otherwise create it.

### Notification only (no custom sound):

```json
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

### With custom sound:

If the user has a custom audio file they want to play, use this configuration:

```json
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

Replace the sound path with the actual path to the user's audio file.

**IMPORTANT**: Always use `$HOME` in the hook command, never `%USERPROFILE%`. Claude Code hooks run via bash.

## Step 5: Verify

Run the binary directly to confirm it works:

```bash
$HOME/.claude/notify/ccnotify -t "CCNotify" -b "Installation successful"
```

The user should see a system notification appear. If it works, the setup is complete.

## CLI Reference

```
ccnotify [OPTIONS]

OPTIONS:
  -t, --title <TITLE>  Notification title (default: "Claude Code")
  -b, --body <BODY>    Notification body (default: "Task completed")
  -s, --sound <PATH>   Play audio file at given path (sound version only)
  -h, --help           Show help
  -V, --version        Show version
```
