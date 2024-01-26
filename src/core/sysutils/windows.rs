//! # Windows specific utilities

use std::{
    env::current_exe,
    os::windows::process::CommandExt,
    process::{self, exit, Command, Stdio},
    time::Duration,
};

use super::{Error, Result};

/// Deletes the current process executable.
pub fn kill_my_self() -> Result<()> {
    let id = process::id();

    let current_process_path = current_exe().map_err(Error::Io)?;

    let current_process = current_process_path
        .to_str()
        .ok_or(Error::CantConvertPathToStr)?;

    Command::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
        .args([
            "-Command",
            &format!(
                r#"Wait-Process -Id {}; Remove-Item -LiteralPath "{}""#,
                id, current_process
            ),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000 | 0x00000200)
        .spawn()
        .map_err(Error::Io)?;

    Ok(())
}

/// Restarts the current process after a given time.
pub fn restart(time: Duration) -> Result<()> {
    let current_process_path = current_exe().map_err(Error::Io)?;

    let current_process = current_process_path
        .to_str()
        .ok_or(Error::CantConvertPathToStr)?;

    Command::new(r"C:\Windows\System32\WindowsPowerShell\v1.0\powershell.exe")
        .args([
            "-Command",
            &format!(
                r#"Start-Sleep -Milliseconds {};  Start-Process "{}""#,
                time.as_millis(),
                current_process
            ),
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(0x08000000 | 0x00000200)
        .spawn()
        .map_err(Error::Io)?;

    exit(0);
}
