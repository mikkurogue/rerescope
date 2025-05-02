use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;
use ratatui::widgets::ListState;

pub struct App {
    pub files: Vec<String>,
    pub filtered_files: Vec<String>,
    pub query: String,
    pub selected: usize,
    pub list_state: ListState,
}

impl App {
    pub fn new(files: Vec<String>) -> Self {
        let filtered_files = files.clone();

        Self {
            files,
            filtered_files,
            query: String::new(),
            selected: 0,
            list_state: ListState::default(),
        }
    }

    pub fn update_filtered_files(&mut self) {
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
