#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//! # Nmap Scan Result
//!

use axum::{
    extract::State,
    response::{Html, IntoResponse},
    routing::{get, post},
    Json,
};
use clap::{Parser, ValueEnum};
use futures::future::join;
use quick_xml::{events::attributes::Attribute, name::QName, reader::Reader};
use serde::Serialize;
use std::{
    collections::HashMap, io::Result as IoResult, process::Stdio, sync::Arc, time::Duration,
};

use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpListener,
    spawn,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    time::sleep,
};

#[derive(serde::Deserialize, Serialize)]
struct ScanRequest {
    targets: Vec<String>,
}

#[tokio::main]
async fn main() -> IoResult<()> {
    let (send, recv) = unbounded_channel();
    let scan_job = tokio::spawn(scan_job(recv));
    let web_job = tokio::spawn(web_job(80, send));
    scan_job.await.unwrap();
    web_job.await.unwrap();
    Ok(())
}

async fn web_job(port: u16, job_queue: UnboundedSender<ScanRequest>) {
    let app = axum::Router::new()
        .route("/", get(main_route))
        .route("/scan", post(scan_route))
        .with_state(job_queue);
    let listener = TcpListener::bind(("0.0.0.0", port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// #[cfg(test)]
// mod test {
//     use crate::ScanQuery;
//
//     #[test]
//     fn test_query() {
//         let q = ScanQuery { targets: vec![1] };
//         let q = serde_json::to_string(&q);
//         println!("{}", q.unwrap());
//         assert!(false)
//     }
// }

#[derive(askama::Template)]
#[template(path = "index.html")]
struct IndexTmpl {
    foo: String,
}

async fn main_route() -> impl IntoResponse {
    Html(
        IndexTmpl {
            foo: "Bar".to_string(),
        }
        .to_string(),
    )
}

#[axum::debug_handler]
async fn scan_route(
    State(job_queue): State<UnboundedSender<ScanRequest>>,
    Json(req): Json<ScanRequest>,
) -> impl IntoResponse {
    job_queue.send(req).or(Err("Bad Data"))
}

async fn scan_job(job_queue: UnboundedReceiver<ScanRequest>) {
    let job_queue: Arc<Mutex<UnboundedReceiver<ScanRequest>>> = Arc::new(Mutex::new(job_queue));
    let jobs: Box<[_]> = (0..4)
        .map(|_| tokio::spawn(scan_worker(job_queue.clone())))
        .collect();
    futures::future::join_all(jobs).await;
}

async fn scan_worker(job_queue: Arc<Mutex<UnboundedReceiver<ScanRequest>>>) {
    while let Some(req) = job_queue.lock().await.recv().await {
        let Ok(mut child) = tokio::process::Command::new("nmap")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .arg("-A")
            .arg("-oX")
            .arg("-")
            .arg("--stats-every")
            .arg("2s")
            .args(req.targets)
            .spawn()
        else {
            continue;
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill().await;
            continue;
        };
        // let write_job = spawn(async move {
        //     loop {
        //         sleep(Duration::from_secs(2)).await;
        //         let Ok(_) = stdin.write_all(b" \n\r").await else {
        //             break;
        //         };
        //         println!("INTURRUPTED");
        //     }
        // });
        let read_job = spawn(async move {
            let mut buf = Vec::new();
            let mut reader = Reader::from_reader(BufReader::new(stdout));
            while let Ok(res) = reader.read_event_into_async(&mut buf).await {
                match res {
                    quick_xml::events::Event::Start(bytes_start) => {
                        let QName(name) = bytes_start.name();
                        let name = String::from_utf8_lossy(name);
                        println!("Start Event: {name}");
                    }
                    quick_xml::events::Event::End(bytes_end) => {
                        let QName(name) = bytes_end.name();
                        let name = String::from_utf8_lossy(name);
                        println!("End Event: {name}");
                    }
                    quick_xml::events::Event::Empty(bytes_start) => {
                        let QName(name) = bytes_start.name();
                        let attr: HashMap<_, _> = bytes_start
                            .attributes()
                            .flatten()
                            .map(|Attribute { key, value }| {
                                let QName(key) = key;
                                let key = String::from_utf8_lossy(key).to_string();
                                let value = String::from_utf8_lossy(value.as_ref()).to_string();
                                (key, value)
                            })
                            .collect();
                        let name = String::from_utf8_lossy(name);
                        println!("Empty Event: {name:?} {attr:#?}");
                    }
                    quick_xml::events::Event::Text(bytes_text) => {
                        let Ok(event) = bytes_text.decode().and_then(|i| Ok(i.to_string())) else {
                            continue;
                        };
                        println!("Text Event: {} len: {}", event, bytes_text.len());
                    }
                    quick_xml::events::Event::Eof => break,
                    _ => continue,
                }
            }
        });
        let (_, Ok(ret)) = join(read_job, child.wait()).await else {
            continue;
        };
        println!("Process ended: {ret}");
    }
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
            Self::Paranoid => "-T0",
            Self::Sneaky => "-T1",
            Self::Polite => "-T2",
            Self::Normal => "-T3",
            Self::Aggressive => "-T4",
            Self::Insane => "-T5",
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
