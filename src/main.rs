use serde::Deserialize;
use std::fs;
use tokio::sync::broadcast;
use toml;

mod calendar;
mod server;
mod settings;


#[tokio::main]
async fn main() {
    let file = fs::read_to_string(settings::SETTINGS_FILE).expect("Settings file not found!");

    let settings: settings::Settings = toml::from_str(&file).expect("Settings could not be parsed!");

    let (tx, _) = broadcast::channel::<String>(255);

    tokio::join!(
        calendar::update_task(settings.calendar, tx.clone()),
        server::start_server(settings.server, tx.clone())
    );
}
