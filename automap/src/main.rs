#![feature(try_blocks)]
#![feature(io_error_other)]

mod scan_result;

use axum::routing::get;
use lazy_static::lazy_static;
use quick_xml::de::from_str;
use scan_result::ScanResult;
use std::{
    io::{Error, ErrorKind, Result},
    process::Stdio,
};
use tokio::{
    process::Command,
    sync::RwLock,
    time::{self, Duration, Instant},
};

lazy_static! {
    static ref LAST_SCAN: RwLock<ScanResult> = RwLock::new(ScanResult::default());
}

#[tokio::main]
async fn main() {
    let scan_job = tokio::spawn(scan_job());
    let web_job = tokio::spawn(web_job());
    scan_job.await.unwrap();
    web_job.await.unwrap();
}

async fn web_job() {
    let app = axum::Router::new().route(
        "/",
        get(|| async {
            let guard = LAST_SCAN.read().await;
            guard.to_string()
        }),
    );

    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn scan_job() {
    let mut start = Instant::now();
    loop {
        println!("Starting scan job at {:?}", start);
        let _: Result<()> = try {
            let mut res = Command::new("nmap")
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .arg("-A")
                .arg("--unprivileged")
                .arg("-oX")
                .arg("scan.xml")
                .arg("172.30.0.0/24")
                .spawn()?;
            res.wait().await?;
        };
        println!("Scan job finished at {:?}", Instant::now());
        if let Err(e) = parse_xml().await {
            println!("Error parsing XML: {}", e);
        };
        while start < Instant::now() {
            start += Duration::from_secs(1800);
        }
        println!("Next scan job at {:?}", start);
        time::sleep_until(start).await;
    }
}

async fn parse_xml() -> Result<()> {
    let scan_results = String::from_utf8(tokio::fs::read("scan.xml").await?)
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    let results = from_str::<ScanResult>(scan_results.as_str())
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    let mut guard = LAST_SCAN.write().await;
    println!("{}", results);
    *guard = results;
    Ok(())
}
