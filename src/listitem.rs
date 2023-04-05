use tui::widgets::ListState;
use std::{
    collections::HashMap,
    fs::DirEntry
};

pub enum FileType {
    NONE,
    FILE,
    DIRECTORY,
    SYMLINK,
    // ZIP,
}

pub fn get_file_type(entry_ref: &DirEntry) -> FileType {
    let ret: FileType;
    let f_type = entry_ref.metadata().unwrap().file_type();
    if f_type.is_dir() { ret = FileType::DIRECTORY }
    else if f_type.is_file() { ret = FileType::FILE }
    else if f_type.is_symlink() { ret = FileType::SYMLINK }
    else { ret = FileType::NONE }

    ret
}

pub struct ListItems {
    // pub items: Vec<String>,
    pub items: HashMap<String, FileType>,
    pub state: ListState,
}

impl ListItems {
    pub fn from_items(items: HashMap<String, FileType>) -> ListItems {
        ListItems {
            items,
            state: ListState::default(),
        }
    }

    pub fn go_first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_last(&mut self) {
        let i = self.items.len() -1;
        self.state.select(Some(i));
    }

    pub fn next(&mut self) {
        let idx = match self.state.selected() {
            Some(idx) => {
                if idx >= self.items.len() - 1 {
                    self.items.len() - 1
                } else {
                    idx + 1
                }

            },
            None => {
                0
            }
        };

        self.state.select(Some(idx));
    }

    pub fn prev(&mut self) {
        let idx = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0 
                } else {
                    i - 1
                }
            },
            None => 0,
        };

        self.state.select(Some(idx));
    }
}
