use std::rc::Rc;

use ratatui::{
    Frame,
    layout::Rect,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub struct Input;

impl Input {
    pub fn render(app: &mut App, chunks: &Rc<[Rect]>, frame: &mut Frame) {
        let input = Paragraph::new(Text::from(app.query.as_str()))
            .block(Block::default().borders(Borders::ALL).title("Search"));
        frame.render_widget(input, chunks[0]);
    }
}
