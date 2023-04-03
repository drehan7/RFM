use std::path::PathBuf;
use std::fs;
use crate::{listitem, input};

pub struct MainApp<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub start_path: PathBuf,
    pub list_items: listitem::ListItems,
    pub show_popup: bool,
    pub show_help: bool,
    pub add_file_popup: bool,
    pub input: input::Input,
}

impl<'a> MainApp<'a> {
    pub fn new(title: &'a str, start_path: PathBuf) -> MainApp {
        let dir = PathBuf::from(&start_path);
        let mut l = Vec::new();
        for entry in fs::read_dir(dir).unwrap() {
            l.push(
                entry
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_owned()
            );
        }

        MainApp {
            title,
            should_quit: false,
            start_path,
            list_items: listitem::ListItems::from_items(l),
            show_popup: false,
            show_help: false,
            add_file_popup: false,
            input: input::Input::new(),
        } 
    }

    pub fn refresh_items(&mut self) {
        let dir = PathBuf::from(&self.start_path);
        let mut l = Vec::new();
        for entry in fs::read_dir(dir).unwrap() {
            l.push(
                entry
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap()
                    .to_owned()
            );
        }

        self.list_items = listitem::ListItems::from_items(l);
    }
}
