#![windows_subsystem = "windows"]

use std::io::Cursor;

use cursive::{views::Dialog, Cursive, CursiveExt};
use kira::{
    manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings},
    sound::{
        streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings},
        FromFileError, Region,
    },
    tween::Tween,
};
use scenes::intro;

mod scenes;

static JOY: &[u8] = include_bytes!("../assets/joy.ogg");
static SELECT: &[u8] = include_bytes!("../assets/select.ogg");
static FIER_ME: &[u8] = include_bytes!("../assets/FierMe.ogg");
static CHILL: &[u8] = include_bytes!("../assets/chill.ogg");
static PATIENCE: &[u8] = include_bytes!("../assets/patience.ogg");
static CREDITS: &[u8] = include_bytes!("../assets/credits.ogg");
static OHNO: &[u8] = include_bytes!("../assets/OhNo.ogg");
static FEELINGS: &[u8] = include_bytes!("../assets/feelings.ogg");
static MYSTERIOUS: &[u8] = include_bytes!("../assets/mysterius.ogg");

pub struct GameData {
    pub audio_manager: AudioManager<DefaultBackend>,
    pub audios: [&'static [u8]; 9],
    pub music_loop: Option<StreamingSoundHandle<FromFileError>>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            audio_manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
                .expect("Failed to create audio manager"),
            audios: [
                JOY, SELECT, FIER_ME, CHILL, PATIENCE, CREDITS, OHNO, FEELINGS, MYSTERIOUS,
            ],
            music_loop: None,
        }
    }

    pub fn play(&mut self, index: usize) -> StreamingSoundHandle<FromFileError> {
        let sound_data = StreamingSoundData::from_cursor(
            Cursor::new(self.audios[index]),
            StreamingSoundSettings::default(),
        )
        .expect("Failed to load music");

        self.audio_manager
            .play(sound_data)
            .expect("Failed to play music")
    }

    pub fn play_click(&mut self) {
        let click =
            StreamingSoundData::from_cursor(Cursor::new(SELECT), StreamingSoundSettings::default())
                .expect("Failed to load music");

        self.audio_manager
            .play(click)
            .expect("Failed to play music");
    }

    pub fn play_loop(&mut self, index: usize) {
        let is_playing_loop = self.music_loop.is_some();
        if is_playing_loop {
            self.stop_loop();
        }

        let music = StreamingSoundData::from_cursor(
            Cursor::new(self.audios[index]),
            StreamingSoundSettings::default().loop_region(Region::default()),
        )
        .expect("Failed to load music");

        self.music_loop = Some(
            self.audio_manager
                .play(music)
                .expect("Failed to play music"),
        );
    }

    pub fn stop_loop(&mut self) {
        if let Some(mut music_loop) = self.music_loop.take() {
            music_loop
                .stop(Tween::default())
                .expect("Failed to stop music");
        }
    }
}

fn main() {
    error_advice();
    game();
}

fn game() {
    let mut game_data = GameData::new();
    game_data.play_loop(3);

    let mut siv = Cursive::default();

    siv.set_user_data(game_data);

    siv.set_window_title("Maintenance Assistant Tool");

    siv.add_layer(
        Dialog::text("Seja bem-vindo ao Maintenance Assistant Tool!")
            .title("Bem-vindo!")
            .button("Próximo", |s| {
                if let Some(game_data) = s.user_data::<GameData>() {
                    game_data.play_click();
                }

                s.pop_layer();
                intro(s);
            }),
    );

    siv.run();
}

#[cfg(windows)]
fn error_advice() {
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
