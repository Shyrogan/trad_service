use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new()
        .routes(routes!(register))
}

/// A new subject represent a subject that is currently not being handled
/// by the platform.
#[derive(Deserialize, ToSchema, Debug)]
pub struct NewSubject {
    /// A custom name for a translation subject aka. a website,
    /// this is used in the administrator interface.
    pub custom_name: Option<String>,
    /// The URL for that subject, generally a domain.
    /// Please note that subdomains are not supported for a given domain.
    pub domain: String
}

#[utoipa::path(
    post,
    path="",
    request_body=NewSubject,
    responses(
        (status=200)
    )
)]
pub async fn register(Json(subject): Json<NewSubject>) -> impl IntoResponse {
    println!("{:?}", subject);

    StatusCode::OK
}
