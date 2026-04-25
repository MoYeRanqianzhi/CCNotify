use std::error::Error;

#[cfg(windows)]
pub fn show(title: &str, body: &str, silent: bool) -> Result<(), Box<dyn Error>> {
    use winrt_notification::{Duration, Sound, Toast};

    let mut toast = Toast::new(Toast::POWERSHELL_APP_ID)
        .title(title)
        .text1(body)
        .duration(Duration::Short);

    toast = if silent {
        toast.sound(None)
    } else {
        toast.sound(Some(Sound::Default))
    };

    toast.show()?;
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn show(title: &str, body: &str, silent: bool) -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    let script = if silent {
        format!(
            "display notification \"{}\" with title \"{}\"",
            body.replace('\\', "\\\\").replace('"', "\\\""),
            title.replace('\\', "\\\\").replace('"', "\\\""),
        )
    } else {
        format!(
            "display notification \"{}\" with title \"{}\" sound name \"Glass\"",
            body.replace('\\', "\\\\").replace('"', "\\\""),
            title.replace('\\', "\\\\").replace('"', "\\\""),
        )
    };

    Command::new("osascript").args(["-e", &script]).output()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn show(title: &str, body: &str, silent: bool) -> Result<(), Box<dyn Error>> {
    use std::process::Command;

    Command::new("notify-send").args([title, body]).output()?;

    if !silent {
        let _ = Command::new("canberra-gtk-play")
            .args(["-i", "message-new-instant"])
            .output();
    }
    Ok(())
}
