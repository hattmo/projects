#![feature(never_type)]
use std::{error::Error, io, time::Duration};

use crossterm::{
    self,
    event::{self, Event as TermEvent},
};
use ratatui::DefaultTerminal;
use tokio::{
    sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
    task::JoinHandle,
};

use futures::StreamExt;

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
    let render_handle: JoinHandle<Result<(), Box<str>>> = tokio::spawn(handle_render(rx));
    let event_handle: JoinHandle<Result<(), Box<str>>> = {
        let tx = tx.clone();
        tokio::spawn(async move {
            let mut stream = event::EventStream::new();
            while let Some(foo) = stream.next().await {}
            Ok(())
        })
    };
    let dbus_handle: JoinHandle<Result<(), Box<str>>> = tokio::spawn(handle_dbus(tx));
    ratatui::restore();
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

async fn handle_render(mut rx: UnboundedReceiver<AppEvent>) -> Result<(), Box<str>> {
    let mut term = ratatui::init();
    let mut state = AppState {
        msg: "Loading".to_owned(),
    };
    loop {
        redraw(&mut term, &state).err_str()?;
        match rx.recv().await {
            Some(evt) => {
                if process_event(&mut state, evt)? {
                    break;
                }
            }
            None => break,
        }
    }
    Ok(())
}

fn process_event(state: &mut AppState, event: AppEvent) -> Result<bool, Box<str>> {
    match event {
        AppEvent::TermEvent(event) => todo!(),
        AppEvent::DbusEvent(DbusEvent::HostName(hn)) => todo!(),
    }
    Ok(false)
}

fn redraw(term: &mut DefaultTerminal, state: &AppState) -> io::Result<()> {
    term.draw(|frame| {
        frame.render_widget(state.msg.as_str(), frame.area());
    })?;
    Ok(())
}

async fn handle_dbus(event_stream: UnboundedSender<AppEvent>) -> Result<(), Box<str>> {
    tokio::time::sleep(Duration::from_secs(1)).await;
    let conn = Connection::system().await.err_str()?;
    let p = Proxy::new(
        &conn,
        "org.freedesktop.hostname1",
        "org/freedesktop/hostname1",
        "org.freedesktop.hostname1",
    )
    .await
    .err_str()?;
    let hn: String = p.get_property("Hostname").await.err_str()?;
    event_stream
        .send(AppEvent::DbusEvent(DbusEvent::HostName(hn)))
        .map_err(|e| zbus::Error::Failure(e.to_string()))
        .err_str()?;
    Ok(())
}

trait ErrorString<T> {
    fn err_str(self) -> Result<T, Box<str>>;
}

impl<T, E> ErrorString<T> for Result<T, E>
where
    E: ToString,
{
    fn err_str(self) -> Result<T, Box<str>> {
        self.map_err(|e| e.to_string().into())
    }
}
