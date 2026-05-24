use axum::{Router, http::HeaderMap, response::Html, routing::get, serve};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new();
    let router = router.route("/", get(health_check));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    serve(listener, router).await?;
    println!("Hello, world!");
    Ok(())
}
async fn health_check(headers: ) -> Html<String> {
    let style = "\"border: solid\"";
    let rows: String = headers
        .into_iter()
        .map(|(key, val)| {
            let key = key
                .map(|k| k.as_str().to_string())
                .unwrap_or("_".to_string());
            let val = val.to_str().unwrap_or("_");
            format!("<tr><td style={style}>{key}</td><td style={style}>{val}</td></tr>")
        })
        .collect();
    let header_table = format!("<table style={style}>{rows}</table>");

    let rows: String = env::vars()
        .into_iter()
        .map(|(key, val)| {
            format!("<tr><td style={style}>{key}</td><td style={style}>{val}</td></tr>")
        })
        .collect();
    let env_table = format!("<table style={style}>{rows}</table>");
    format!("{header_table}{env_table}").into()
}
