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
use crate::appmain;

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

pub fn delete_file(app: &mut appmain::MainApp, idx: usize)  {
    let selected_file = &app.list_items.items[idx];
    let _ = remove_file(selected_file);
    app.refresh_items();
}
