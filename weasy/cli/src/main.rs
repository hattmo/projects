#![feature(never_type)]
use std::{error::Error, time::Duration};

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event as TermEvent},
};
use tokio::{
    sync::mpsc::{UnboundedSender, unbounded_channel},
    task::spawn_blocking,
};
use zbus::{Connection, Proxy};

struct AppState {
    msg: String,
}

enum DbusEvent {
    HostName(String),
}

enum AppEvent {
    TermEvent(TermEvent),
    DbusEvent(DbusEvent),
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = unbounded_channel();
    spawn_blocking(move || {
        let res: Result<(), Box<dyn Error>> = ratatui::run(|term| {
            let mut state = AppState {
                msg: "Loading".to_owned(),
            };
            render(term, &state)?;
            while let Some(evt) = rx.blocking_recv() {
                if handle_event(evt, &mut state) {
                    break;
                };
                render(term, &state)?;
            }
            Ok(())
        });
        if let Err(e) = res {
            println!("Error: {e}");
        }
    });
    {
        let tx = tx.clone();
        spawn_blocking(move || {
            while let Ok(evt) = event::read() {
                if let Err(_) = tx.send(AppEvent::TermEvent(evt)) {
                    break;
                };
            }
        });
    }
    if let Err(e) = connect_dbus(tx).await {
        println!("DBUS Error: {e}");
    };
}

fn handle_event(evt: AppEvent, state: &mut AppState) -> bool {
    match evt {
        AppEvent::TermEvent(term_event) => match term_event {
            TermEvent::Key(key_event) => match key_event.code {
                event::KeyCode::Enter => return true,
                _ => {}
            },
            _ => {}
        },
        AppEvent::DbusEvent(dbus_event) => match dbus_event {
            DbusEvent::HostName(hn) => state.msg = hn,
        },
    }
    return false;
}

fn render(term: &mut DefaultTerminal, state: &AppState) -> Result<(), Box<dyn Error>> {
    term.draw(|frame| {
        frame.render_widget(state.msg.as_str(), frame.area());
    })?;
    Ok(())
}

async fn connect_dbus(event_stream: UnboundedSender<AppEvent>) -> Result<(), zbus::Error> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let conn = Connection::system().await?;
    let p = Proxy::new(
        &conn,
        "org.freedesktop.hostname1",
        "org/freedesktop/hostname1",
        "org.freedesktop.hostname1",
    )
    .await?;
    let hn: String = p.get_property("Hostname").await?;
    event_stream
        .send(AppEvent::DbusEvent(DbusEvent::HostName(hn)))
        .map_err(|e| zbus::Error::Failure(e.to_string()))?;
    Ok(())
}
