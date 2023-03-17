use axum::{
    body::Bytes,
    routing::{get_service, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};

use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    let path = env::current_dir().unwrap();
    tracing::debug!("Serving {:?}", path);

    let serve_dir = get_service(ServeDir::new(path));
    let app = Router::new()
        .route("/exfil", post(exfil))
        .nest_service("/", serve_dir)
        .layer(cors);

    let addr = "0.0.0.0:8000".parse()?;
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

async fn exfil(body: Bytes) {
    tracing::debug!("{:?}", body);
}
