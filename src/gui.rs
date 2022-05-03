use::chrono;
use eframe::{epi, egui};
use tokio::sync::mpsc;

use crate::calendar;

// const EVENT_HEIGHT: f32 = 100.;
const PADDING: f32 = 20.;

pub struct CalendarEvent {
    pub start: chrono::DateTime<chrono::Local>,
    pub end: chrono::DateTime<chrono::Local>,
    pub label: String
}

pub struct GuiApp {
    pub rx: mpsc::Receiver<Vec<calendar::EventItem>>,
    pub events: Vec<CalendarEvent>
}

impl GuiApp {
    pub fn new(rx: mpsc::Receiver<Vec<calendar::EventItem>>) -> Self {
        Self {
            rx: rx,
            events: Vec::new()
        }
    }
    fn update_events_from_gcal(&mut self, gcal_events: Vec::<calendar::EventItem>) {
        self.events = Vec::new();
        for gcal_event in gcal_events {
            if gcal_event.start.is_none() || gcal_event.end.is_none() {
                continue;
            }

            let event = CalendarEvent{
                start: gcal_event.start.unwrap().dateTime,
                end: gcal_event.end.unwrap().dateTime,
                label: match gcal_event.summary {
                    Some(s) => s,
                    None => String::new()
                }
            };
            self.events.push(event);
        }
    }
    fn render_events(&self, ctx: &egui::Context, ui: &mut eframe::egui::Ui) {
        let start_time = chrono::NaiveTime::from_hms(8, 0, 0);
        let end_time = chrono::NaiveTime::from_hms(20, 0, 0);
        let total_duration = (end_time - start_time).num_seconds();

        let clip_rect = ui.clip_rect();
        let offset = clip_rect.max - clip_rect.min;

        for (idx, event) in self.events.iter().enumerate() {
            let bottom = time_to_px(event.start, start_time, total_duration, offset.y);
            let top = time_to_px(event.end, start_time, total_duration, offset.y);
            let rect = egui::Rect::from_min_max(egui::Pos2::new(PADDING, bottom), egui::Pos2::new(offset.x - PADDING, top));
            let mut window = egui::Window::new(&event.label)
                .fixed_pos(rect.min)
                .id(egui::Id::new(idx))
                .show(ctx, |ui| {
                    ui.add(egui::Label::new(&event.label));
                });
        }
    }
}

impl epi::App for GuiApp {
    fn name(&self) -> &str {
        "Calendar"
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {   
        let res = self.rx.try_recv();
        let mut updated = false;
        match res {
            Ok(events) => {
                updated = true;
                self.update_events_from_gcal(events);
            },
            Err(_) => ()
        };

        let panel = egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Calendar");
            ui.add_space(PADDING);
            // println!("{:?}", ui.clip_rect());
            self.render_events(ctx, ui);
        });
        if updated { ctx.request_repaint(); }
    }
}

fn time_to_px(time: chrono::DateTime<chrono::Local>, start_time: chrono::NaiveTime, total_secs: i64, max_px: f32) -> f32 {
    let duration = time.time() - start_time;
    max_px * duration.num_seconds() as f32 / total_secs as f32
}
