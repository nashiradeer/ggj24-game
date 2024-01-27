//! TODO: Add game's description here

#![windows_subsystem = "windows"]

use cursive::{Cursive, CursiveExt};
use scenes::intro;

mod scenes;

fn main() {
    init();
    game();
}

fn game() {
    let mut siv = Cursive::default();

    siv.set_window_title("Maintenance Assistant Tool");

    siv.add_layer(intro());

    siv.run();
}

#[cfg(windows)]
fn init() {
    use std::env;

    use win32console::console::WinConsole;
    use win_msgbox::Okay;
    use windows::core::{w, HSTRING};

    let current_executable = env::current_exe().expect("Failed to get current executable path");

    let exe_name = current_executable
        .file_name()
        .expect("Failed to get current executable name")
        .to_str()
        .expect("Failed to convert current executable name to str");

    if let Err(e) = win_msgbox::error::<Okay>(w!("This application failed to start because UnityEngine.dll was not found. Starting our Maintenance Assistant Tool...").0).title(HSTRING::from(format!("{} - System Error", exe_name)).as_ptr()).show() {
        panic!("Failed to show error message: {}", e);
    }

    if let Err(e) = WinConsole::alloc_console() {
        panic!("Failed to allocate console: {}", e);
    }
}
