use chrono;
use eframe::{epi, egui};
use gcp_auth;
use reqwest;
use serde_json;
use urlencoding;
use serde::Deserialize;
use std::fs::File;
use std::io;

const BASE_URL: &str = "https://www.googleapis.com/calendar/v3/calendars";
const SETTINGS_FILE: &str = "settings.json";

#[derive(Deserialize)]
struct Settings {
    calendar_id: String
}

#[derive(Default)]
struct MyEguiApp {}

impl epi::App for MyEguiApp {
    fn name(&self) -> &str {
        "Calendar"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello Calendar");
        });
    }
}

#[derive(Deserialize, Debug)]
struct EventTime {
    pub dateTime: chrono::DateTime<chrono::Local>,
    pub timeZone: String
}

#[derive(Deserialize, Debug)]
struct EventItem {
    pub start: EventTime,
    pub end: EventTime
}

#[derive(Deserialize, Debug)]
struct EventResponse {
    pub items: Vec<EventItem>
    // pub items: Vec<serde_json::Value>
}

#[tokio::main]
async fn main() {
    let file = File::open(SETTINGS_FILE).expect("Settings file not found!");
    let reader = io::BufReader::new(file);

    let settings: Settings = serde_json::from_reader(reader).expect("Settings couldn't be parsed!");

    let service_account = gcp_auth::CustomServiceAccount::from_file("key.json").unwrap();
    let auth = gcp_auth::AuthenticationManager::from(service_account);

    let scopes = &["https://www.googleapis.com/auth/calendar.readonly"];
    let token =  auth.get_token(scopes).await.unwrap();
    println!("{:?}", token.as_str());

    let start = chrono::offset::Local::now().to_rfc3339();
    let end = (chrono::offset::Local::now() + chrono::Duration::days(1)).to_rfc3339();
    
    let start = urlencoding::encode(&start).into_owned(); 
    let end = urlencoding::encode(&end).into_owned(); 
    let url = format!("{}/{}/events?timeMin={}&timeMax={}&singleEvents=true", BASE_URL, settings.calendar_id, start, end);
    println!("{}", url);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token.as_str()))
        .send()
        .await
        .unwrap()
        .json::<EventResponse>()
        .await
        .unwrap();

    // println!("{:?}", response);

    for item in response.items {
        println!("{:?}", item);
    }

    // let app = MyEguiApp::default();
    // let mut native_options = eframe::NativeOptions::default();
    // native_options.decorated = true;
    // native_options.maximized = false;
    // eframe::run_native(Box::new(app), native_options);
}
