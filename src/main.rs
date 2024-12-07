pub mod subject;
pub mod open_api;
pub mod config;
pub mod translation;

use axum::Router;
use config::Config;
use open_api::ApiDoc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::Scalar;

pub fn router() -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/subject", subject::router())
        .nest("/api/v1/translation", translation::router())
        .split_for_parts();

    router.merge(Scalar::new(api))
}

#[tokio::main]
pub async fn main() {
    let config = Config::new();

    let listener = tokio::net::TcpListener::bind(config.address.clone())
        .await
        .expect("Failed to initialize socket");

    axum::serve(listener, router().into_make_service())
        .await
        .expect("Failed to start server")
}

