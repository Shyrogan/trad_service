pub mod subject;
pub mod open_api;
pub mod config;
pub mod translation;

use axum::Router;
use axum_otel_metrics::HttpMetricsLayerBuilder;
use config::Config;
use open_api::ApiDoc;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

pub fn router() -> Router {
    let metrics = HttpMetricsLayerBuilder::new()
        .with_service_name(env!("CARGO_PKG_NAME").to_string())
        .with_service_version(env!("CARGO_PKG_VERSION").to_string())
        .build();

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/subject", subject::router())
        .nest("/api/v1/translation", translation::router())
        .split_for_parts();

    router
        .merge(Scalar::with_url("/api/v1/scalar", api))
        .merge(metrics.routes())
        .layer(metrics)
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

