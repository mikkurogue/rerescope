use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ignore::Walk;
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Modifier, Style},
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use std::{error::Error, io};

struct App {
    files: Vec<String>,
    filtered_files: Vec<String>,
    query: String,
    selected: usize,
    list_state: ListState,
}

impl App {
    fn new(files: Vec<String>) -> Self {
        let filtered_files = files.clone();

        Self {
            files,
            filtered_files,
            query: String::new(),
            selected: 0,
            list_state: ListState::default(),
        }
    }

    fn update_filtered_files(&mut self) {
        if self.query.is_empty() {
            self.filtered_files = self.files.clone();
        } else {
            let matcher = SkimMatcherV2::default();
            let mut scored_files: Vec<(String, i64)> = self
                .files
                .iter()
                .filter_map(|file| {
                    matcher
                        .fuzzy_match(file, &self.query)
                        .map(|score| (file.clone(), score))
                })
                .collect();

            // Sort by match score descending
            scored_files.sort_by(|a, b| b.1.cmp(&a.1));

            // Now set filtered_files just to the filenames
            self.filtered_files = scored_files.into_iter().map(|(file, _)| file).collect();
        }
        self.selected = 0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let files: Vec<String> = Walk::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter_map(|e| e.path().to_str().map(|s| s.to_string()))
        .collect();

    let mut app = App::new(files);

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    // After exit, print selected file
    if let Some(selected) = app.filtered_files.get(app.selected) {
        println!("{}", selected);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.area());

            // Input box

            let input = Paragraph::new(Text::from(app.query.as_str()))
                .block(Block::default().borders(Borders::ALL).title("Search"));
            f.render_widget(input, chunks[0]);

            // File list
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
            f.render_stateful_widget(list, chunks[1], &mut app.list_state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => {
                        app.query.push(c);
                        app.update_filtered_files();
                    }
                    KeyCode::Backspace => {
                        app.query.pop();
                        app.update_filtered_files();
                    }
                    KeyCode::Up => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if app.selected + 1 < app.filtered_files.len() {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }
}
