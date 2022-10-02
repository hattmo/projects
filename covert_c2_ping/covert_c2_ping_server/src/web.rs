use std::time::Duration;

use serde::Deserialize;
use tokio::time::sleep;
use warp::{Filter, Rejection};

#[derive(Deserialize)]
pub struct NewAgent {
    pub arch: String,
    pub sleep: u32,
    pub pipe: String,
}

#[derive(Deserialize)]
pub struct PatchAgent {
    pub agentid: u32,
    pub sleep: Option<u32>,
}

pub async fn web_worker() -> () {
    let api = warp::path("api")
        .and(warp::get().map(|| "Connected agents"))
        .or(warp::post()
            .and(warp::body::json::<NewAgent>())
            .map(|body: NewAgent| "New agent added"))
        .or(warp::patch()
            .and(warp::body::json::<PatchAgent>())
            .map(|body: PatchAgent| format!("Patch agent sleep {:?}", body.sleep)));
    warp::filters::fs::dir("./static");
    warp::serve(api).bind(([0, 0, 0, 0], 8080)).await;
}
