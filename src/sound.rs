use std::error::Error;
use std::path::Path;

#[cfg(windows)]
pub fn play(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Err(format!("audio file not found: {path}").into());
    }

    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "winmm")]
    extern "system" {
        fn mciSendStringW(
            command: *const u16,
            return_string: *mut u16,
            return_length: u32,
            callback: usize,
        ) -> u32;
    }

    fn mci(cmd: &str) -> u32 {
        let wide: Vec<u16> = OsStr::new(cmd).encode_wide().chain(Some(0)).collect();
        unsafe { mciSendStringW(wide.as_ptr(), std::ptr::null_mut(), 0, 0) }
    }

    let abs_path = std::fs::canonicalize(path)?;
    let path_str = abs_path.to_string_lossy().replace('/', "\\");
    // strip UNC prefix \\?\ that canonicalize adds on Windows
    let path_str = path_str.strip_prefix("\\\\?\\").unwrap_or(&path_str);

    mci("close ccnotify_snd");
    let err = mci(&format!("open \"{path_str}\" alias ccnotify_snd"));
    if err != 0 {
        return Err(format!("mci open failed (error {err}): {path_str}").into());
    }
    mci("play ccnotify_snd wait");
    mci("close ccnotify_snd");

    Ok(())
}

#[cfg(target_os = "macos")]
pub fn play(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Err(format!("audio file not found: {path}").into());
    }
    std::process::Command::new("afplay").arg(path).status()?;
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn play(path: &str) -> Result<(), Box<dyn Error>> {
    if !Path::new(path).exists() {
        return Err(format!("audio file not found: {path}").into());
    }
    if std::process::Command::new("paplay").arg(path).status().is_err() {
        std::process::Command::new("aplay").arg(path).status()?;
    }
    Ok(())
}
