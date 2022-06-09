use chrono;
use gcp_auth;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json;
use tokio::sync::broadcast;
use tokio::time;
use urlencoding;

use crate::settings;

const BASE_URL: &str = "https://www.googleapis.com/calendar/v3/calendars";
const CALENDAR_SCOPE: &str = "https://www.googleapis.com/auth/calendar.readonly";

#[derive(Deserialize, Debug, Serialize)]
pub struct EventTime {
    pub dateTime: chrono::DateTime<chrono::Local>,
    pub timeZone: String
}

#[derive(Deserialize, Debug, Serialize)]
pub struct EventItem {
    pub start: Option<EventTime>,
    pub end: Option<EventTime>,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub hangoutLink: Option<String>
}

#[derive(Deserialize, Debug)]
struct EventResponse {
    pub items: Vec<EventItem>
}

async fn fetch_data(settings: &settings::Calendar, token: &str) -> Option<Vec<EventItem>> {
    let today = chrono::offset::Local::today().and_hms(0, 0, 0);
    let start = today.to_rfc3339();
    let end = (today + chrono::Duration::days(settings.days_ahead)).to_rfc3339();

    let start = urlencoding::encode(&start).into_owned(); 
    let end = urlencoding::encode(&end).into_owned(); 
    let url = format!("{}/{}/events?timeMin={}&timeMax={}&singleEvents=true", BASE_URL, settings.calendar_id, start, end);

    let client = reqwest::Client::new();

    let response = match client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await {
            Ok(r) => r,
            Err(_) => return None
        };

    match response.json::<EventResponse>().await {
        Ok(r) => Some(r.items),
        Err(_) => None
    }
}

pub async fn update_task(settings: settings::Calendar, tx: broadcast::Sender<String>) {
    let service_account = gcp_auth::CustomServiceAccount::from_file(&settings.key_path).expect("Key could not be parsed!");
    let auth = gcp_auth::AuthenticationManager::from(service_account);

    let scopes = &[CALENDAR_SCOPE];
    let token =  auth.get_token(scopes).await.expect("Token could not be generated!");

    loop {
        if let Some(items) = fetch_data(&settings, token.as_str()).await {
            if let Ok(json_data) = serde_json::to_string(&items) {
                tx.send(json_data.to_string());
            }
        }
        time::sleep(time::Duration::from_secs(settings.refresh_secs)).await;
    }
}