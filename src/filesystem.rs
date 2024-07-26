use ratatui::widgets::ListState;
use std::path::{Path, PathBuf};
use std::{fs::{self, DirEntry, read_to_string}, io};

pub struct DirectoryList {
    pub items: Vec<DirEntry>,
    pub state: ListState,
    pub path: PathBuf,
}

pub struct SelectedFile {
    pub path: PathBuf,
    pub contents: String,
    pub line_count: usize,
}

pub fn get_dir_entries (path: &Path) -> Result<Vec<DirEntry>, io::Error> {
    match fs::read_dir(path) {
        Ok(entries) => {
            return entries
                .map(|m| m)
                .collect::<Result<Vec<DirEntry>, io::Error>>();
        }
        Err(e) => {
            return Err(e);
        }
    };
}

impl SelectedFile {
    pub fn from_path(path: PathBuf) -> SelectedFile {
        let vec_contents:Vec<String> = match read_to_string(&path) {
            Ok(read_file) => {
                read_file
                    .lines()
                    .map(String::from)
                    .collect()
            }
            _ => {
                vec![String::from("\n")]
            }
        };

        SelectedFile {
            path: PathBuf::from(path),
            contents: vec_contents.join("\n"),
            line_count: vec_contents.len() as usize,
        }
    }
}

impl DirectoryList {
    pub fn new(path: &PathBuf) -> DirectoryList {
        let curr_entries = match get_dir_entries(path) {
            Ok(entries) => {
                entries
            },
            Err(_) => Vec::new()
        };

        DirectoryList {
            items: curr_entries,
            state: ListState::default(),
            path: PathBuf::from("DAWUD TEST")
        }
    }


    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
