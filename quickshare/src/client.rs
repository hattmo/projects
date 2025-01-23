use std::io::Result as IoResult;

use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn client_start() -> IoResult<()> {
    let mut terminal = ratatui::init();
    loop {
        terminal.draw(render)?;
        match event::read()? {
            event::Event::Key(key_event) => {
                if let KeyCode::Char('q') = key_event.code {
                    break;
                };
            }
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}

fn render(frame: &mut Frame) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(frame.area());
    frame.render_widget(
        Paragraph::new("Hello").block(Block::new().borders(Borders::ALL)),
        layout[0],
    );

    frame.render_widget(
        Paragraph::new("World").block(Block::new().borders(Borders::ALL)),
        layout[1],
    );
}
