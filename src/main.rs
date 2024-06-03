mod ui;
mod app;
mod utils;
mod filesystem;

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{ enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen };
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::error::Error;
use std::io;
use std::path::PathBuf;
use std::env::current_dir;

/*
/*! Main MVP for now:
*/      - List all entries in cwd.
        - Selecting File displays contents in split pane


        Selecting Dir `cd`s and displays contents
        Shortcuts to add, remove, rename file/dir
        Shortcut to 'yank' file and place in another directory

        Cool TODOS:
            Image support
            Tree view of filesystem
            Change permissions
            Symbolic Links
            Open file in editor
*/


fn main() -> Result<(), Box<dyn Error>> {
    // TODO CLI Arguments
    
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let curr_path = match current_dir() {
        Ok(_path) => PathBuf::from("src"),

        // Default to current
        Err(_) => PathBuf::from(".")
    };

    let mut app = app::App::new("RFM (Rusty File Manager)", &curr_path);
    app.run(&mut terminal)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    terminal.show_cursor()?;


    Ok(())
}

