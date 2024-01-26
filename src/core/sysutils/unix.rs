//! # Unix specific utilities

use std::{
    env::{self, current_exe},
    fs::remove_file,
    io::{stdout, Write},
    process::{Command, Stdio},
    thread::sleep,
    time::Duration,
};

use super::{Error, Result};

/// Deletes the current process executable.
pub fn kill_my_self() -> Result<()> {
    let current_process_path = current_exe().map_err(Error::Io)?;

    let current_process = current_process_path
        .to_str()
        .ok_or(Error::CantConvertPathToStr)?;

    remove_file(current_process).map_err(Error::Io)?;

    Ok(())
}

/// Restarts the current process after a given time.
///
/// In Unix systems, this function will spawn the current shell or `/bin/sh` if the `SHELL` environment variable is not set and then kill it after the given time.
pub fn restart(time: Duration) -> Result<()> {
    let mut child = Command::new(env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string()))
        .envs(env::vars())
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(Error::Io)?;

    sleep(time);

    child.kill().map_err(Error::Io)?;

    print!("\n");
    stdout().flush().map_err(Error::Io)?;

    Ok(())
}
