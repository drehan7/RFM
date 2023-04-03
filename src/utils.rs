use std::{
    io::{Stdout, stdout, Error},
    path::PathBuf,
    fs::File,
    env,
};
use tui::{
    Terminal,
    backend::{Backend, CrosstermBackend},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

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

pub fn add_file(file_name: &str) {
    let _ = File::create(file_name);
}
