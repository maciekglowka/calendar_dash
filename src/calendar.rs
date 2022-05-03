use chrono;
use gcp_auth;
use reqwest;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::time;
use urlencoding;

const BASE_URL: &str = "https://www.googleapis.com/calendar/v3/calendars";

#[derive(Deserialize, Debug)]
pub struct EventTime {
    pub dateTime: chrono::DateTime<chrono::Local>,
    pub timeZone: String
}

#[derive(Deserialize, Debug)]
pub struct EventItem {
    pub start: Option<EventTime>,
    pub end: Option<EventTime>,
    pub description: Option<String>,
    pub summary: Option<String>
}

#[derive(Deserialize, Debug)]
struct EventResponse {
    pub items: Vec<EventItem>
}

pub async fn update_task(calendar_id: String, refresh_secs: u64, tx: mpsc::Sender<Vec<EventItem>>) {
    let service_account = gcp_auth::CustomServiceAccount::from_file("key.json").unwrap();
    let auth = gcp_auth::AuthenticationManager::from(service_account);

    let scopes = &["https://www.googleapis.com/auth/calendar.readonly"];
    let token =  auth.get_token(scopes).await.unwrap();

    println!("test");
    loop {
        let start = chrono::offset::Local::now().to_rfc3339();
        let end = (chrono::offset::Local::now() + chrono::Duration::days(4)).to_rfc3339();

        let start = urlencoding::encode(&start).into_owned(); 
        let end = urlencoding::encode(&end).into_owned(); 
        let url = format!("{}/{}/events?timeMin={}&timeMax={}&singleEvents=true", BASE_URL, calendar_id, start, end);

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

        tx.send(response.items).await;
        println!("sent");
        time::sleep(time::Duration::from_secs(refresh_secs)).await;
    }
}