use ratatui::widgets::ListState;
use std::path::PathBuf;
use std::fs::{read_to_string, DirEntry};

pub struct DirectoryList {
    pub items: Vec<DirEntry>,
    pub state: ListState,
}

pub struct SelectedFile {
    pub path: PathBuf,
    pub contents: String,
    pub line_count: u16,
}

impl SelectedFile {
    pub fn from_path(path: PathBuf) -> SelectedFile {
        let vec_contents:Vec<String> = read_to_string(&path)
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

        SelectedFile {
            path: PathBuf::from(path),
            contents: vec_contents.join("\n"),
            line_count: vec_contents.len() as u16,
        }
    }
}

impl DirectoryList {
    pub fn new(items: Vec<DirEntry>) -> DirectoryList {
        DirectoryList {
            items,
            state: ListState::default()
        }
    }

    pub fn set_items(&mut self, items: Vec<DirEntry>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                }
                else {
                    i + 1
                }
            }
            None => 0,
        };

        self.state.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                }
                else {
                    i - 1
                }
            }
            None => 0
        };

        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
