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

## Current State
- v0.1.0: notification-only version (tag: v0.1.0)
- v0.2.0: added sound feature (tag: v0.2.0)
- dist/ccnotify.exe (133KB) and dist/ccnotify-sound.exe (150KB) ready
