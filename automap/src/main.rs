#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//! # Nmap Scan Result
//!

mod scan_result;

use axum::routing::get;
use chrono::Local;
use clap::{Parser, ValueEnum};
use quick_xml::de::from_str;
use scan_result::ScanResult;
use std::{
    io::{Error, ErrorKind, Result as IoResult},
    sync::LazyLock,
    time::Duration,
};
use tokio::{fs::read, net::TcpListener, process::Command, sync::RwLock};

static LAST_SCAN: LazyLock<RwLock<ScanResult>> =
    LazyLock::new(|| RwLock::new(ScanResult::default()));
static ARGS: LazyLock<Arguments> = LazyLock::new(|| Arguments::parse());

#[tokio::main]
async fn main() -> IoResult<()> {
    // command
    let timing = ARGS.timing.as_ref().unwrap_or(&Timing::Normal);
    let mut command_args: Vec<String> = vec![
        "-vvv",
        "-A",
        &timing.as_flag(),
        "--unprivileged",
        "-oX",
        "scan.xml",
    ]
    .into_iter()
    .map(ToOwned::to_owned)
    .collect();
    command_args.append(ARGS.targets.clone().as_mut());
    let command_args = command_args.leak();
    let cmd_str = command_args.join(" ");

    // sleep
    let sleep = ARGS.sleep.clone().unwrap_or("1d".to_owned());
    let sleep = parse_duration::parse(&sleep).map_err(|e| Error::new(ErrorKind::Other, e))?;

    // port
    let port = ARGS.port.unwrap_or(3000);

    println!("Running nmap with args: {cmd_str}");
    println!("Will rerun every {sleep:?}");
    println!("Listening of port: {port}");
    let scan_job = tokio::spawn(scan_job(command_args, sleep));
    let web_job = tokio::spawn(web_job(port));
    scan_job.await?.unwrap();
    web_job.await.unwrap();
    Ok(())
}

async fn web_job(port: u16) {
    let app = axum::Router::new().route(
        "/",
        get(|| async {
            let guard = LAST_SCAN.read().await;
            guard.to_string()
        }),
    );
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn scan_job(command_args: &[String], sleep: Duration) -> IoResult<()> {
    loop {
        println!("Starting scan at {}", Local::now());
        let mut command = Command::new("nmap");
        command.args(command_args);
        let res = command.status().await?;
        if !res.success() {
            println!("Scan failed with status: {res}");
        } else if let Err(e) = parse_xml().await {
            println!("Error parsing XML: {e}");
        };
        println!("Done scanning, sleeping for {sleep:?}");
        tokio::time::sleep(sleep).await;
    }
}

async fn parse_xml() -> IoResult<()> {
    println!("Parsing XML");

    let scan_results =
        String::from_utf8(read("scan.xml").await?).map_err(|e| Error::new(ErrorKind::Other, e))?;
    let results = from_str::<ScanResult>(scan_results.as_str())
        .map_err(|e| Error::new(ErrorKind::Other, e))?;
    *LAST_SCAN.write().await = results;
    Ok(())
}

#[derive(ValueEnum, Clone)]
enum Timing {
    Paranoid,
    Sneaky,
    Polite,
    Normal,
    Aggressive,
    Insane,
}

impl Timing {
    fn as_flag(&self) -> String {
        match self {
            Timing::Paranoid => "-T0",
            Timing::Sneaky => "-T1",
            Timing::Polite => "-T2",
            Timing::Normal => "-T3",
            Timing::Aggressive => "-T4",
            Timing::Insane => "-T5",
        }
        .to_owned()
    }
}

#[derive(Parser)]
struct Arguments {
    #[arg(short, long)]
    timing: Option<Timing>,
    #[arg(short, long)]
    port: Option<u16>,
    #[arg(short, long)]
    sleep: Option<String>,
    targets: Vec<String>,
}
