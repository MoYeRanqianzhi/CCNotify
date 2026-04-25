# CCNotify - Shared Memory

## Architecture
- Rust cross-platform binary, 133KB (notification-only) / 150KB (with sound)
- Windows: winrt-notification + winmm MCI; macOS: osascript + afplay; Linux: notify-send + paplay
- Two build profiles via Cargo features: `default` (pure notification) and `sound` (+ audio playback)
- Release profile: strip + LTO + opt-level=z + panic=abort

## Key Decisions
- 2026-04-26: Chose Rust compiled binary over PowerShell script — solves hook path expansion bug, cross-platform, zero runtime deps
- 2026-04-26: System notification sounds instead of embedded audio — smaller binary, native UX
- 2026-04-26: winmm MCI for Windows audio playback — no extra crate, supports MP3/WAV/WMA
- 2026-04-26: raw.githubusercontent.com for agent guide links — agents can directly fetch the raw markdown

## Current State
- GitHub: https://github.com/MoYeRanqianzhi/CCNotify
- v0.2.0: released with 8 binaries (4 platforms x 2 versions)
- CI/CD: GitHub Actions auto-builds on tag push
- Agent guide: docs/AGENT_INSTALL.md with real download URLs
- Hook config updated: `$HOME/.claude/notify/ccnotify.exe -s $HOME/.claude/notify/new-notification.mp3`
