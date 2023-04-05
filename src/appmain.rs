use std::{
    fs,
    path::PathBuf,
    collections::HashMap
};
use crate::{listitem, input};

pub struct MainApp<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub start_path: PathBuf,
    pub list_items: listitem::ListItems,
    pub show_popup: bool,
    pub show_help: bool,
    pub show_confirmation: bool,
    pub add_file_popup: bool,
    pub input: input::Input,
}

impl<'a> MainApp<'a> {
    pub fn new(title: &'a str, start_path: PathBuf) -> MainApp {
        let mut l = HashMap::new();
        for entry in fs::read_dir(&start_path).unwrap() {
            let e = &entry.unwrap();
            let file_name = e.file_name().to_str().unwrap().to_owned();
            let file_type: listitem::FileType = listitem::get_file_type(&e);

            l.insert(file_name, file_type);

        }

        MainApp {
            title,
            should_quit: false,
            start_path,
            list_items: listitem::ListItems::from_items(l),
            show_popup: false,
            show_help: false,
            show_confirmation: false,
            add_file_popup: false,
            input: input::Input::new(),
        } 
    }

    pub fn refresh_items(&mut self) {
        let dir = PathBuf::from(&self.start_path);
        let mut l = HashMap::new();
        for entry in fs::read_dir(dir).unwrap() {
            let e = &entry.unwrap();
            let file_name = e.file_name().to_str().unwrap().to_owned();
            let file_type: listitem::FileType = listitem::get_file_type(&e);

            l.insert(file_name, file_type);

        }

        self.list_items = listitem::ListItems::from_items(l);
    }
}
