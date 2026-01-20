use std::{fs, io::Read, os::linux::raw::stat, process::exit, sync::mpsc, time::Duration};

use pty_process::blocking::{open, Command, Pts, Pty};
use ratatui::{
    crossterm::{
        self,
        event::{self, Event},
    },
    widgets::{List, ListState},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct TaskItem {
    name: String,
    command: String,
}

enum RedrawEvent {
    TermEvent(Event),
    Redraw,
}

impl From<Event> for RedrawEvent {
    fn from(value: Event) -> Self {
        Self::TermEvent(value)
    }
}

fn main() {
    let conf = fs::read_to_string(".doer").unwrap();
    let conf: Vec<TaskItem> = serde_yaml::from_str(&conf).unwrap();
    let conf = conf.as_slice();
    let (send_redraw, get_redraw) = mpsc::channel();
    let (send_cmd, get_cmd) = mpsc::channel();
    send_redraw.send(RedrawEvent::Redraw).unwrap();
    std::thread::scope(|t| {
        t.spawn(move || {
            while let Ok(cmd) = get_cmd.recv() {
                let Some(TaskItem { name: _, command }) = conf.get(cmd) else {
                    continue;
                };
                let (mut pty, pts) = open().unwrap();
                let child = Command::new("/bin/bash")
                    .arg("-c")
                    .arg(command)
                    .spawn(pts)
                    .unwrap();
                loop {
                    let mut buf = [0u8; 256];
                    let ammount = pty.read(&mut buf);
                }
            }
            eprintln!("spawner done");
        });
        t.spawn(move || {
            loop {
                let Ok(is_ready) = event::poll(Duration::from_millis(100)) else {
                    break;
                };
                if !is_ready {
                    if send_redraw.send(RedrawEvent::Redraw).is_err() {
                        break;
                    };
                    continue;
                }
                let Ok(event) = event::read() else {
                    break;
                };
                if send_redraw.send(event.into()).is_err() {
                    break;
                };
            }
            eprintln!("key done");
        });
        t.spawn(move || {
            ratatui::run(|term| {
                let mut state = ListState::default();
                state.select_first();
                while let Ok(event) = get_redraw.recv() {
                    match event {
                        RedrawEvent::TermEvent(Event::Key(key_event)) => match key_event.code {
                            event::KeyCode::Char('q') => break,
                            event::KeyCode::Down => state.select_next(),
                            event::KeyCode::Up => state.select_previous(),
                            event::KeyCode::Enter => {
                                if let Some(index) = state.selected() {
                                    send_cmd.send(index).unwrap()
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    };
                    term.draw(|frame| {
                        let area = frame.area();
                        let names = conf.iter().map(|i| i.name.as_str());
                        let list = List::new(names).scroll_padding(2).highlight_symbol(">");
                        frame.render_stateful_widget(list, area, &mut state);
                    })
                    .unwrap();
                }
                eprintln!("draw loop done");
            });
            eprintln!("draw done");
        });
    });
}
