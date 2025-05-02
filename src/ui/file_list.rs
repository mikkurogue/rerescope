use std::rc::Rc;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem},
};

use crate::app::App;

pub struct FileList;

impl FileList {
    pub fn render(app: &mut App, chunks: &Rc<[Rect]>, frame: &mut Frame) {
        let items: Vec<ListItem> = app
            .filtered_files
            .iter()
            .map(|f| ListItem::new(Span::from(f.clone())))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Files"))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        app.list_state.select(Some(app.selected));
        frame.render_stateful_widget(list, chunks[1], &mut app.list_state);
    }
}
