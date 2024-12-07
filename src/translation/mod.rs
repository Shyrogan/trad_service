use axum::extract::ws::Message;
use utoipa_axum::{router::OpenApiRouter, routes};

pub mod ws;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(ws::handler))
}

/// Handles a single message encoded in UTF-8, this method is called by both the REST
/// and WS APIs.
async fn handle_message_utf8(message: String) -> Message {
    Message::Text(message)
}
