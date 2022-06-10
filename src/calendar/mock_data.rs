use chrono;

pub fn get_mock_data() -> Vec<super::EventItem> {
    let now = chrono::offset::Local::now();
    let mut v = Vec::new();
    for idx in -1..5 {
        let start = super::EventTime{
            dateTime: now + chrono::Duration::minutes(45 * idx),
            timeZone: String::new()
        };
        let end = super::EventTime{
            dateTime: now + chrono::Duration::minutes(45 * idx + 30),
            timeZone: String::new()
        };

        let event = super::EventItem{
            id: Some(format!("id_{}", idx)),
            start: Some(start),
            end: Some(end),
            summary: Some(format!("Był więc tylko jeden kruczek - paragraf {} 
                - który stwierdzał, że troska o własne życie w obliczu realnego 
                i bezpośredniego zagrożenia jest dowodem zdrowia psychicznego.", idx)),
            description: Some("Summary".to_string()),
            hangoutLink: Some("https://google.com".to_string())
        };
        v.push(event);
    };
    v
}