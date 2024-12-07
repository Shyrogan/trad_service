use axum::{extract::{ws::{Message, WebSocket}, WebSocketUpgrade}, response::IntoResponse};

use super::handle_message_utf8;

#[utoipa::path(
    get,
    path="",
    responses(
        (status=101, description="Connects to the WebSocket that emits translations for a website.")
    )
)]
pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    // When the websocket is closed, `recv` returns None
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(text) => match text {
                Message::Text(translation_key) => {
                    let _ = socket.send(handle_message_utf8(translation_key).await)
                        .await;
                },
                _ => println!("Invalid data type")
            },
            Err(e) => println!("An error ocurred {:?}", e),
        }
    }
}

