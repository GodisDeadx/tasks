mod gui;

use dirs::home_dir;
use iced::{window, Application, Settings};
use std::path::PathBuf;
use std::process::exit;

use crate::gui::json_handling;

#[cfg(target_os = "windows")]
mod windows {
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::ShowWindow;
    use winapi::um::winuser::SW_HIDE;

    pub fn hide_console() {
        unsafe {
            let console_window = GetConsoleWindow();
            ShowWindow(console_window, SW_HIDE);
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod windows {
    pub fn hide_console() {}
}

fn hide_console() {
    windows::hide_console();
}

use iced::window::icon::from_file;

fn main() -> iced::Result {
    #[cfg(target_os = "linux")]
    let icon_path = json_handling::get_path("img/icon.png");
    #[cfg(target_os = "windows")]
    let icon_path = json_handling::get_path("img\\icon.png");

    let icon = from_file(icon_path).unwrap();

    let window_position: json_handling::WindowPosition = match json_handling::read_settings() {
        Ok(settings) => json_handling::WindowPosition {
            x: settings.x,
            y: settings.y,
        },
        Err(_) => json_handling::WindowPosition { x: 100, y: 100 },
    };

    hide_console();
    let settings = Settings {
        window: window::Settings {
            size: (800, 335),
            icon: Some(icon),
            position: (window::Position::Specific(window_position.x, window_position.y)),
            ..Default::default()
        },
        ..Default::default()
    };

    gui::TaskList::run(settings)
}
