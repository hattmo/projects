#![feature(lazy_cell)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//! # Nmap Scan Result
//!

mod scan_result;

use axum::routing::get;
use chrono::Local;
use quick_xml::de::from_str;
use scan_result::ScanResult;
use std::{
    env,
    io::{self, Error, ErrorKind, Result},
    sync::LazyLock,
};
use tokio::{fs::read, net::TcpListener, process::Command, sync::RwLock};

static LAST_SCAN: LazyLock<RwLock<ScanResult>> =
    LazyLock::new(|| RwLock::new(ScanResult::default()));

#[tokio::main]
async fn main() {
    let scan_job = tokio::spawn(scan_job());
    let web_job = tokio::spawn(web_job());
    scan_job.await.unwrap().unwrap();
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
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn scan_job() -> Result<()> {
    let target_env = env::var("TARGET").or(Err(io::Error::other("No target set")))?;
    let targets: Vec<_> = target_env
        .split(' ')
        .map(str::trim)
        .map(ToString::to_string)
        .collect();
    loop {
        println!("Starting scan at {}", Local::now());
        println!("nmap -vvv -A -T1 --unprivileged -oX scan.xml {target_env}");
        let mut command = Command::new("nmap");
        command
            .arg("-vvv")
            .arg("-A")
            .arg("-T1")
            .arg("--unprivileged")
            .arg("-oX")
            .arg("scan.xml")
            .args(targets.clone());
        let res = command.status().await?;
        if !res.success() {
            println!("Scan failed with status: {res}");
        } else if let Err(e) = parse_xml().await {
            println!("Error parsing XML: {e}");
        };
        println!("Done scanning, sleeping for 1 hour");
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}

async fn parse_xml() -> Result<()> {
    println!("Parsing XML");

    let scan_results =
        String::from_utf8(read("scan.xml").await?).map_err(|e| Error::new(ErrorKind::Other, e))?;
    let results = from_str::<ScanResult>(scan_results.as_str())
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    *LAST_SCAN.write().await = results;
    Ok(())
}
