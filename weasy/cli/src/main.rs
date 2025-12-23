#![feature(never_type)]
use std::{error::Error, os::linux::raw::stat, sync::Arc, time::Duration};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
};
use tokio::sync::{Mutex, mpsc::UnboundedReceiver};
use zbus::{Connection, Proxy};

#[tokio::main]
async fn main() {
    let term = ratatui::init();
    let state = Arc::new(Mutex::new(String::from("Loading...")));
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            if let Err(e) = run(term, state, rx) {
                println!("Error: {e}");
            }
        });
    }
    if let Err(e) = connect_dbus(state).await {
        println!("Error: {e}");
    };
    ratatui::restore();
}

fn run(
    mut term: DefaultTerminal,
    state: Arc<Mutex<String>>,
    mut rx: UnboundedReceiver<()>,
) -> Result<(), Box<dyn Error>> {
    while let Some(_) = rx.blocking_recv() {
        let state = state.clone();
        term.draw(|frame| render(frame, state))?;
    }
    Ok(())
}

fn render(frame: &mut Frame, state: Arc<Mutex<String>>) {
    let state = state.blocking_lock();
    let data = state.as_str();
    frame.render_widget(data, frame.area());
}

async fn connect_dbus(state: Arc<Mutex<String>>) -> Result<(), zbus::Error> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    let conn = Connection::system().await?;
    let p = Proxy::new(
        &conn,
        "org.freedesktop.hostname1",
        "org/freedesktop/hostname1",
        "org.freedesktop.hostname1",
    )
    .await?;
    let hn: String = p.get_property("Hostname").await?;
    let mut state = state.lock().await;
    *state = hn;
    Ok(())
}
