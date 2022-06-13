use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension
    },
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router
};
use lazy_static::lazy_static;
use std::{
    io,
    net::SocketAddr,
    sync::Arc
};
use tera::{Context, Tera};
use tokio::sync::{broadcast, Mutex};
use tower_http::services::ServeDir;

use crate::settings;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(_) => panic!("Could not load templates!")
        }
    };
    static ref CACHE: Mutex<String> = Mutex::new(String::new());
}

struct AppState {
    tx: broadcast::Sender<String>,
    ws_host: String,
    ws_port: u16
}

pub async fn start_server(settings: settings::Server, mut tx: broadcast::Sender<String>) {
    let addr: SocketAddr = format!("{}:{}", settings.host, settings.port)
        .parse().expect("Incorrect hostname");

    let app_state = Arc::new(AppState {
        tx: tx.clone(),
        ws_host: settings.host,
        ws_port: settings.port
    });

    tokio::spawn(async move {
        cache_handler(tx.clone()).await;
    }); 

    let app = Router::new()
        .route("/", get(main_handler))
        .route("/ws", get(ws_handler))
        .fallback(get_service(ServeDir::new("./static")).handle_error(handle_error))
        .layer(Extension(app_state));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Could not start the server!");
}

async fn main_handler(
    Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
    let mut context = Context::new();
    context.insert("ws_host", &state.ws_host);
    context.insert("ws_port", &state.ws_port);
    match TEMPLATES.render("index.html", &context) {
        Ok(html) => Html(html),
        Err(e) => Html(e.to_string())
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(state): Extension<Arc<AppState>>
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    send_cache(&mut socket).await;
    let mut rx = state.tx.subscribe();
    loop {
        let data = rx.recv().await;
        if let Ok(json) = data {
            if socket
                .send(Message::Text(json))
                .await
                .is_err() { return; }
        }
    }
}

async fn send_cache(socket: & mut WebSocket) {
    let cache = CACHE.lock().await;
    socket.send(Message::Text(cache.to_owned())).await;
}

async fn handle_error(error: io::Error) -> impl IntoResponse {
    Html(error.to_string())
}

async fn cache_handler(tx: broadcast::Sender<String>) {
    let mut rx = tx.subscribe();
    loop {
        let data = rx.recv().await;
        if let Ok(json) = data {
            let mut cache = CACHE.lock().await;
            *cache = json;
        }
    }
}