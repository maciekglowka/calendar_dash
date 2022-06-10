use serde::Deserialize;

pub const SETTINGS_FILE: &str = "settings.toml";

#[derive(Deserialize)]
pub struct Settings {
    pub calendar: Calendar,
    pub server: Server
}   

#[derive(Clone, Deserialize)]
pub struct Calendar {
    pub calendar_id: String,
    pub refresh_secs: u64,
    pub days_ahead: i64,
    pub key_path: String,
    pub debug: bool
}

#[derive(Clone, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16
}