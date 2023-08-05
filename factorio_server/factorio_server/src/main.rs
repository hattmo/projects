#![feature(lazy_cell)]
use anyhow::Result;
use axum::{
    extract::Path,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server, ServiceExt,
};
use clap::Parser;
use service::Settings;
use std::{net::SocketAddr, sync::LazyLock};
use tower_http::services::ServeDir;

mod service;
mod settings;

static CONFIG: LazyLock<Config> = LazyLock::new(Config::parse);

#[derive(Parser)]
struct Config {
    #[clap(short, long, default_value = "./factorio")]
    factorio_path: String,
}

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Starting server");
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    let router = Router::new()
        .route("/", get(index))
        .route("/ui/:element", get(ui));
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;

    let mut service = service::Service::new();
    let map_gen_settings = serde_json::from_str(include_str!("map_gen_settings.json"))?;
    let map_settings = serde_json::from_str(include_str!("map_gen_settings.json"))?;
    let settings = Settings::new(&map_gen_settings, &map_settings).await?;
    let save = settings.create_save().await?;
    service.start_server(&save).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    Html(include_str!("ui/index.html"))
}

async fn ui(Path(path): Path<String>) -> impl IntoResponse {
    match path.as_str() {
        "server_settings.html" => Ok(Html(include_str!("ui/server_settings.html"))),
        _ => Err(()),
    }
}

#[cfg(test)]
mod test {
    use serde::{Deserialize, Serialize};
    use serde_json;
    #[derive(Serialize, Deserialize, Debug)]
    struct Foo {
        #[serde(skip_serializing_if = "Option::is_none")]
        bar: Option<u32>,
    }

    #[test]
    fn test() {
        let mut foo: Foo = serde_json::from_str("{}").unwrap();
        println!("{:?}", foo);
        foo.bar = Some(1);
        let out = serde_json::to_string_pretty(&foo).unwrap();
        println!("{}", out);
    }
}
