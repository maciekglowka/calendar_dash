use serde::Deserialize;
use std::fs;
use tokio::sync::mpsc;
use toml;

mod calendar;
mod gui;

const SETTINGS_FILE: &str = "settings.toml";

#[derive(Deserialize)]
struct Settings {
    calendar_id: String,
    refresh_secs: u64
}

// #[derive(Default)]

fn main() {
    let file = fs::read_to_string(SETTINGS_FILE).expect("Settings file not found!");

    let settings: Settings = toml::from_str(&file).expect("Settings could not be parsed!");

    let (tx, mut rx) = mpsc::channel::<Vec<calendar::EventItem>>(128);

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.spawn(calendar::update_task(settings.calendar_id.clone(), settings.refresh_secs, tx));

    let app = gui::GuiApp::new(rx);
    let mut native_options = eframe::NativeOptions::default();
    native_options.decorated = true;
    native_options.maximized = true;
    eframe::run_native(Box::new(app), native_options);
}
