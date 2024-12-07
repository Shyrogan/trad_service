use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(description = "An OpenAPI specification")
)]
pub struct ApiDoc;

