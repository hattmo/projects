use std::io::Result as IoResult;

use crossterm::event::{self, KeyCode};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Style, Stylize},
    widgets::{
        block::Title, Block, List, ListDirection, ListItem, ListState, StatefulWidget, Widget,
    },
};

pub fn client_start() -> IoResult<()> {
    let mut terminal = ratatui::init();
    let mut nodes = Nodes::default();
    nodes.update_node(
        "foo".to_string(),
        vec!["bar".to_string(), "baz".to_string()],
    );
    nodes.update_node(
        "foo2".to_string(),
        vec!["bar2".to_string(), "baz2".to_string()],
    );
    loop {
        terminal.draw(|frame| frame.render_widget(&mut nodes, frame.area()))?;
        match event::read()? {
            event::Event::Key(key_event) => match key_event.code {
                KeyCode::Char('q') => break,
                KeyCode::Up => nodes.move_cursor(Dir::Up),
                KeyCode::Down => nodes.move_cursor(Dir::Down),
                KeyCode::Left => nodes.move_cursor(Dir::Left),
                KeyCode::Right => nodes.move_cursor(Dir::Right),
                _ => {}
            },
            _ => {}
        }
    }
    ratatui::restore();
    Ok(())
}

#[derive(Default)]
struct Nodes {
    active: Side,
    node_state: ListState,
    file_state: ListState,
    nodes: Vec<Node>,
}

enum Side {
    Left,
    Right,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Default for Side {
    fn default() -> Self {
        Self::Left
    }
}

struct Node {
    name: String,
    files: Vec<String>,
}

impl Nodes {
    fn move_cursor(&mut self, direction: Dir) {
        match (direction, &self.active) {
            (Dir::Up, Side::Left) => {
                self.file_state.select_first();
                self.node_state.select_previous();
            }
            (Dir::Down, Side::Left) => {
                self.file_state.select_first();
                self.node_state.select_next();
            }

            (Dir::Up, Side::Right) => {
                self.file_state.select_previous();
            }
            (Dir::Down, Side::Right) => {
                self.file_state.select_next();
            }

            (Dir::Left, Side::Right) => self.active = Side::Left,
            (Dir::Right, Side::Left) => self.active = Side::Right,
            _ => {}
        }
    }

    fn update_node(&mut self, name: String, files: Vec<String>) {
        if let Some(exists) = self.nodes.iter_mut().find(|i| i.name == name) {
            exists.files = files;
        } else {
            self.nodes.push(Node { name, files });
        }
    }
}

impl Widget for &mut Nodes {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let [left, right] = split_area(area);
        if let Some(Node { name, files }) =
            self.node_state.selected().and_then(|i| self.nodes.get(i))
        {
            let files: Vec<_> = files.iter().map(|i| i.as_str()).collect();
            let file_list = create_list(files, name.as_str());
            StatefulWidget::render(file_list, right, buf, &mut self.file_state);
        } else {
            Widget::render(Block::bordered(), right, buf);
        }
        let node_names: Vec<_> = self.nodes.iter().map(|i| i.name.as_str()).collect();
        let node_list = create_list(node_names, "Nodes");
        StatefulWidget::render(node_list, left, buf, &mut self.node_state);
    }
}

fn create_list<'a, T, U>(items: T, title: U) -> List<'a>
where
    T: IntoIterator,
    T::Item: Into<ListItem<'a>>,
    U: Into<Title<'a>>,
{
    List::new(items)
        .scroll_padding(2)
        .block(Block::bordered().title(title))
        .style(Style::new().white())
        .highlight_symbol(">> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom)
}

fn split_area(area: Rect) -> [Rect; 2] {
    Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
        .areas::<2>(area.clone())
}
