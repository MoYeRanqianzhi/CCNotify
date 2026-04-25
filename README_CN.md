# CCNotify

[English](README.md)

跨平台通知工具，用于 [Claude Code](https://docs.anthropic.com/en/docs/claude-code)。当 Claude Code 完成任务时发送系统原生通知（含提示音）。

**133KB** 二进制文件，零运行时依赖。

## AI Agent 快速配置

将以下文本发送给你的任意 AI 编程助手，即可自动完成 CCNotify 的安装配置：

```
Read https://raw.githubusercontent.com/MoYeRanqianzhi/CCNotify/master/docs/AGENT_INSTALL.md and follow the instructions to install and configure CCNotify for Claude Code.
```

## 功能

| 版本 | 大小 | 说明 |
|------|------|------|
| `ccnotify` | ~133KB | 系统通知 + 默认提示音 |
| `ccnotify-sound` | ~150KB | + 自定义音频文件播放 (`--sound`) |

### 平台支持

| 平台 | 通知方式 | 系统提示音 | 自定义音频 |
|------|---------|-----------|-----------|
| Windows 10/11 | Toast (WinRT) | `ms-winsoundevent` | winmm MCI (MP3/WAV/WMA) |
| macOS | `osascript` | Glass | `afplay` |
| Linux | `notify-send` | `canberra-gtk-play` | `paplay` / `aplay` |

## 安装

### 预编译二进制

从 [Releases](https://github.com/MoYeRanqianzhi/CCNotify/releases) 下载。

### 从源码编译

```bash
git clone https://github.com/MoYeRanqianzhi/CCNotify.git
cd CCNotify

# 仅通知
cargo build --release

# 含音频播放
cargo build --release --features sound
```

输出路径：`target/release/ccnotify(.exe)`

## 使用方法

```bash
# 默认通知
ccnotify

# 自定义标题和内容
ccnotify -t "构建完成" -b "所有测试通过"

# 播放自定义音频（sound 版本）
ccnotify -s "/path/to/sound.mp3"
ccnotify -t "完成" -b "任务已结束" -s "/path/to/sound.mp3"
```

## Claude Code Hook 配置

在 `~/.claude/settings.json` 中添加：

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

或使用自定义音效：

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

> **注意：** 使用 `$HOME` 而非 `%USERPROFILE%` — Claude Code hooks 通过 bash 执行。

## CLI 参考

```
ccnotify [OPTIONS]

OPTIONS:
  -t, --title <TITLE>  通知标题（默认："Claude Code"）
  -b, --body <BODY>    通知内容（默认："Task completed"）
  -s, --sound <PATH>   播放指定音频文件（sound 版本）
  -h, --help           显示帮助
  -V, --version        显示版本
```

## 许可证

[MIT](LICENSE)
