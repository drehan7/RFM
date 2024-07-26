mod ui;
mod app;
mod utils;
mod filesystem;
mod errors;
mod tui;

use std::error::Error;
use std::env;
use std::path::{PathBuf, Path};

/*
/*! Main MVP for now:
*/      
        - Selecting File displays contents in split pane


        Selecting Dir `cd`s and displays contents
        Shortcuts to add, remove, rename file/dir
        Shortcut to 'yank' file and place in another directory

        Cool TODOS:
            Configuration file
            Tree view of filesystem
            Syntax Highlighting
            Image support
            Change permissions
            Symbolic Links
            Open file in editor
*/

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let curr_path = match args.len(){
        2 => { 
            if Path::new(&args[1]).exists() {
                PathBuf::from(&args[1])
            }
            else {
                PathBuf::from(env::current_dir().unwrap())
            }
        },
        // Debug
        _ => PathBuf::from(env::current_dir().unwrap()),
    };

    errors::install_hooks()?;
    let mut terminal = tui::init()?;

    // TODO CONFIG
    const SCROLL_OFFSET: usize = 15;

    let mut app = app::App::new("RFM (Rusty File Manager)", &curr_path, SCROLL_OFFSET);
    app.run(&mut terminal)?;

    Ok(tui::restore()?)
}

