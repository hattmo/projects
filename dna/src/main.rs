#![feature(try_blocks)]

use clap::Parser;
use owo_colors::OwoColorize;
use std::{net::IpAddr, process::exit, sync::Arc, time::Duration};
use tokio::{
    io::{self, AsyncReadExt},
    net::{lookup_host, TcpStream},
    sync::mpsc::unbounded_channel,
    time::timeout,
};

#[derive(Parser)]
struct Cli {
    host: String,
    lower_port: u16,
    upper_port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let start_port = cli.lower_port;
    let end_port = cli.upper_port.unwrap_or(start_port);
    let width = end_port.to_string().len() * 2 + 1;
    let scanner = Arc::new(Scanner::new(&cli.host).await.unwrap_or_else(|_| {
        println!("Could not resolve dns");
        exit(1);
    }));
    println!("Starting scan");
    let (send, mut recv) = unbounded_channel();
    for port in start_port..(end_port + 1) {
        let scanner = scanner.clone();
        let send = send.clone();
        tokio::spawn(async move { send.send(scanner.scan_port(port).await) });
    }
    drop(send);
    let mut results = Vec::with_capacity((end_port - start_port + 1).into());
    while let Some(res) = recv.recv().await {
        results.push(res);
    }
    results.sort();
    let mut first_down = start_port;
    let mut on_down = false;

    for res in results {
        match (res.result, on_down) {
            (ScanResult::ScanSuccess(header), true) => {
                let range = if res.port - 1 == first_down {
                    format!("{}", first_down)
                } else {
                    format!("{}-{}", first_down, res.port - 1)
                };
                let down = format!("{1:<0$.0$} Down", width, range);
                println!("{}", down.red());
                let up = format!(
                    "{1:<0$.0$} UP   :: {2}",
                    width,
                    res.port,
                    header.unwrap_or("No Header".to_owned())
                );
                println!("{}", up.green());
                on_down = false;
            }
            (ScanResult::ScanSuccess(header), false) => {
                let up = format!(
                    "{1:<0$.0$} UP   :: {2}",
                    width,
                    res.port,
                    header.unwrap_or("No Header".to_owned())
                );
                println!("{}", up.green());
            }
            (ScanResult::ScanFail, true) => {}
            (ScanResult::ScanFail, false) => {
                first_down = res.port;
                on_down = true;
            }
        }
    }
    if on_down {
        let range = if end_port == first_down {
            format!("{}", first_down)
        } else {
            format!("{}-{}", first_down, end_port)
        };
        let down = format!("{1:<0$.0$} Down", width, range);
        println!("{}", down.red());
    }
}

#[derive(Clone, Copy)]
struct Scanner {
    ip: IpAddr,
}

struct ScanData {
    port: u16,
    result: ScanResult,
}
enum ScanResult {
    ScanSuccess(Option<String>),
    ScanFail,
}

impl PartialEq for ScanData {
    fn eq(&self, other: &Self) -> bool {
        self.port == other.port
    }
}

impl Eq for ScanData {}

impl PartialOrd for ScanData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.port.cmp(&other.port))
    }
}

impl Ord for ScanData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.port.cmp(&other.port)
    }
}

impl Scanner {
    pub async fn new(host: &str) -> Result<Self, io::Error> {
        let addr = lookup_host(format!("{}:0", host))
            .await?
            .next()
            .ok_or(io::ErrorKind::Other)?;
        Ok(Scanner { ip: addr.ip() })
    }
    pub async fn scan_port(&self, port: u16) -> ScanData {
        match TcpStream::connect((self.ip, port)).await {
            Ok(mut conn) => {
                let mut buf = [0u8; 32];
                let bytes_read =
                    match timeout(Duration::from_secs(5), conn.read(&mut buf[..])).await {
                        Ok(Ok(size)) => size,
                        _ => {
                            return ScanData {
                                port,
                                result: ScanResult::ScanSuccess(None),
                            }
                        }
                    };
                let buf = &buf[0..bytes_read];
                let header: String = buf
                    .into_iter()
                    .map(|byte| {
                        if byte.is_ascii() && !byte.is_ascii_whitespace() {
                            return char::from(*byte).to_string();
                        } else {
                            return format!("{{0x{:x}}}", byte);
                        }
                    })
                    .collect();
                return ScanData {
                    port,
                    result: ScanResult::ScanSuccess(Some(header)),
                };
            }
            Err(_) => ScanData {
                port,
                result: ScanResult::ScanFail,
            },
        }
    }
}
