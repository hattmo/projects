#![feature(once_cell)]

use std::sync::LazyLock;

use anyhow::Result;
use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::response::Response;
use axum::routing::*;
use axum::Router;
use clap::Parser;
use http::StatusCode;
use openai::Sessions;
use tracing::log;

mod openai;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long, env = "OPENAI_API_KEY")]
    openai_key: String,
}

static ARGS: LazyLock<Args> = LazyLock::new(|| Args::parse());

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let sessions = openai::Sessions::new().await?;
    let app = Router::new().route("/", get(api)).with_state(sessions);
    log::info!("Server started on port 3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}

async fn api(
    State(sessions): State<Sessions>,
    TypedHeader(Authorization(auth)): TypedHeader<Authorization<Bearer>>,
) -> StatusCode {
    let token = auth.token();
    StatusCode::NOT_FOUND
}
#[cfg(test)]
mod test {}
