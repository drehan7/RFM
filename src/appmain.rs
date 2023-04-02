use std::path::PathBuf;
use std::fs;
use crate::listitem;

pub struct MainApp<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub start_path: PathBuf,
    pub list_items: listitem::ListItems,
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
        } }
}
