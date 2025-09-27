#![feature(try_blocks)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_docs)]

//! # Nmap Scan Result
//!

use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse},
    routing::{get, post},
};
use futures::{SinkExt, StreamExt};
use quick_xml::{
    events::{attributes::Attribute, BytesStart, Event},
    name::QName,
    reader::Reader,
};
use serde::Serialize;
use std::{collections::HashMap, io::Result as IoResult, process::Stdio, sync::Arc};

use tokio::{
    io::BufReader,
    net::TcpListener,
    spawn,
    sync::{
        broadcast::{self, error::RecvError, Sender},
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
};

#[derive(serde::Deserialize, Serialize)]
struct ScanRequest {
    targets: String,
}

#[derive(askama::Template, Clone)]
enum Node {
    #[template(path = "port.html")]
    Port(u16),
}

#[tokio::main]
async fn main() -> IoResult<()> {
    let (send, recv) = unbounded_channel();
    let event_queue = broadcast::Sender::new(40);
    let scan_job = spawn(scan_job(recv, event_queue.clone()));
    let web_job = spawn(web_job(80, send, event_queue));
    scan_job.await.unwrap();
    web_job.await.unwrap();
    Ok(())
}

async fn web_job(port: u16, job_queue: UnboundedSender<ScanRequest>, event_queue: Sender<Node>) {
    let app = axum::Router::new()
        .route("/", get(main_route))
        .route("/events", get(ws_route))
        .with_state((job_queue, event_queue));
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
struct IndexTmpl;

async fn main_route() -> impl IntoResponse {
    Html(IndexTmpl.to_string())
}

async fn ws_route(
    State((job_queue, event_queue)): State<(UnboundedSender<ScanRequest>, Sender<Node>)>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let handler = move |ws| async {
        ws_handler(ws, job_queue, event_queue).await;
    };
    ws.on_upgrade(handler)
}

async fn ws_handler(
    ws: WebSocket,
    job_queue: UnboundedSender<ScanRequest>,
    event_queue: Sender<Node>,
) {
    let (mut send, mut recv) = ws.split();
    spawn(async move {
        while let Some(res) = recv.next().await {
            let res: Result<(), String> = try {
                let text = res
                    .map_err(|e| e.to_string())?
                    .into_text()
                    .map_err(|e| e.to_string())?;
                println!("WS MESSAGE{{{text}}}");
                let mess = match serde_json::from_str(&text).map_err(|e| e.to_string()) {
                    Ok(mess) => mess,
                    Err(e) => {
                        println!("Error: {e}");
                        continue;
                    }
                };
                job_queue.send(mess).map_err(|e| e.to_string())?;
            };
            if let Err(e) = res {
                println!("{e}");
                break;
            }
        }
    });
    spawn(async move {
        let mut event_queue = event_queue.subscribe();
        loop {
            let event = match event_queue.recv().await {
                Ok(event) => event,
                Err(RecvError::Lagged(_)) => {
                    continue;
                }
                Err(RecvError::Closed) => break,
            };
            let event = event.to_string();
            if let Err(err) = send.send(Message::Text(event.into())).await {
                println!("{err}");
                break;
            };
        }
    });
}

async fn scan_job(job_queue: UnboundedReceiver<ScanRequest>, event_queue: Sender<Node>) {
    let job_queue: Arc<Mutex<UnboundedReceiver<ScanRequest>>> = Arc::new(Mutex::new(job_queue));

    let jobs: Box<[_]> = (0..4)
        .map(|_| spawn(scan_worker(job_queue.clone(), event_queue.clone())))
        .collect();
    futures::future::join_all(jobs).await;
}

async fn scan_worker(
    job_queue: Arc<Mutex<UnboundedReceiver<ScanRequest>>>,
    event_queue: Sender<Node>,
) {
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
            .arg(req.targets)
            .spawn()
        else {
            println!("Failed to spawn child");
            continue;
        };
        let Some(stdout) = child.stdout.take() else {
            let _ = child.kill().await;
            if let Err(e) = child.wait().await {
                println!("error joining child: {e}");
            };
            continue;
        };
        let mut buf = Vec::new();
        let mut reader = Reader::from_reader(BufReader::new(stdout));
        while let Ok(res) = reader.read_event_into_async(&mut buf).await {
            match res {
                Event::Start(bytes_start) => {
                    let QName(name) = bytes_start.name();
                    let attr = get_attrs(&bytes_start);
                    let name = String::from_utf8_lossy(name);
                    println!("Start Event: {name} {attr:#?}");
                    match name.as_ref() {
                        "port" => {
                            let port = attr
                                .get(&"portid".to_owned())
                                .unwrap_or(&"0".to_owned())
                                .parse()
                                .unwrap_or(0);
                            if let Err(e) = event_queue.send(Node::Port(port)) {
                                println!("Error sending: {e}");
                                return;
                            };
                        }
                        _ => {}
                    }
                }
                Event::End(bytes_end) => {
                    let QName(name) = bytes_end.name();
                    let name = String::from_utf8_lossy(name);
                    println!("End Event: {name}");
                }
                Event::Empty(bytes_start) => {
                    let QName(name) = bytes_start.name();
                    let attr = get_attrs(&bytes_start);
                    let name = String::from_utf8_lossy(name);
                    println!("Empty Event: {name} {attr:#?}");
                }
                Event::Text(bytes_text) => {
                    let Ok(event) = bytes_text.decode().and_then(|i| Ok(i.to_string())) else {
                        continue;
                    };
                    let event = event.trim();
                    if event.len() > 0 {
                        println!("Text Event: {event}");
                    }
                }
                Event::Eof => break,
                _ => continue,
            }
        }
        if let Err(e) = child.wait().await {
            println!("Error joining child: {e}");
        };
        println!("DONE WITH JOB");
    }
}

fn get_attrs(bytes_start: &BytesStart<'_>) -> HashMap<String, String> {
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
    attr
}
