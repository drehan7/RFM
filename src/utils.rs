use std::{
    io::{Stdout, stdout, Error},
    path::PathBuf,
    fs::{File, remove_file},
    env,
};
use tui::{
    Terminal,
    backend::CrosstermBackend,
};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    terminal::EnterAlternateScreen,
};
use crate::{appmain, listitem};

pub fn path_from_args() -> PathBuf {
    let args: Vec<String> = env::args().collect();
    let arg_path = match args.len() {
        2 => {
            PathBuf::from(&args[1])
        },
        _ => {
            PathBuf::from(".")
        }
    };
    arg_path
}

pub fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Error>
{
    let mut stdout = stdout();
    let _ = execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

pub fn add_file(app: &mut appmain::MainApp) {
    let _ = File::create(&app.input.input);
    app.input.input.clear();
    app.add_file_popup = false;
}

pub fn get_file_name(app: &mut appmain::MainApp, idx: usize) -> String {
    let items = &app.list_items.items;
    let mut l = vec![];
    for key in items.keys() {
        l.push(key);
    }
    l[idx].to_owned()
}

pub fn get_file_type(file_type: &listitem::FileType) -> String {

    let ret = match file_type {
        listitem::FileType::NONE => {  String::from("\u{2753}").into() },
        listitem::FileType::DIRECTORY => {  String::from("\u{1f4c1}").into() },
        listitem::FileType::FILE => {  String::from("\u{1f5ce}").into()  },
        listitem::FileType::SYMLINK => {  String::from("\u{1f517}").into() },
    };

    ret
}

pub fn delete_file(app: &mut appmain::MainApp, idx: usize)  {
    // let item_keys: &Vec<_>= &app.list_items.items.into_keys().collect();

    let selected_file = get_file_name(app, idx);
    let _ = remove_file(selected_file);
    app.refresh_items();
}
