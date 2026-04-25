mod notify;
#[cfg(feature = "sound")]
mod sound;

use std::process;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut title = "Claude Code";
    let mut body = "Task completed";
    #[cfg(feature = "sound")]
    let mut sound_path: Option<&str> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--title" => {
                i += 1;
                if i < args.len() {
                    title = &args[i];
                }
            }
            "-b" | "--body" => {
                i += 1;
                if i < args.len() {
                    body = &args[i];
                }
            }
            #[cfg(feature = "sound")]
            "-s" | "--sound" => {
                i += 1;
                if i < args.len() {
                    sound_path = Some(&args[i]);
                }
            }
            "-h" | "--help" => {
                print_help();
                return;
            }
            "-V" | "--version" => {
                println!("ccnotify {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            _ => {}
        }
        i += 1;
    }

    #[cfg(feature = "sound")]
    let silent = sound_path.is_some();
    #[cfg(not(feature = "sound"))]
    let silent = false;

    if let Err(e) = notify::show(title, body, silent) {
        eprintln!("notification failed: {e}");
        process::exit(1);
    }

    #[cfg(feature = "sound")]
    if let Some(path) = sound_path {
        if let Err(e) = sound::play(path) {
            eprintln!("sound playback failed: {e}");
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("ccnotify {} - Cross-platform notification for Claude Code", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE: ccnotify [OPTIONS]");
    println!();
    println!("OPTIONS:");
    println!("  -t, --title <TITLE>  Notification title (default: \"Claude Code\")");
    println!("  -b, --body <BODY>    Notification body (default: \"Task completed\")");
    #[cfg(feature = "sound")]
    println!("  -s, --sound <PATH>   Play audio file at specified path");
    println!("  -h, --help           Show help");
    println!("  -V, --version        Show version");
}
