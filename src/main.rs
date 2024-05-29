use std::io;
use tui::{ backend::Backend, layout::{Constraint, Direction, Layout}, style::{Color, Modifier, Style}, widgets::{Block, Borders, List,  ListItem}, Terminal };
use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode}, execute, terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen}
};


mod appmain;
mod listitem;
mod utils;
mod layout;
mod input;

use layout::ui_layout;

fn main() -> Result<(), io::Error>  {
    /*
       setup terminal
       enable raw mode which ensures that user input is passed directly to our application
       without the terminal driver intercepting and carrying out processing of its own
    */
    enable_raw_mode()?;
    let mut terminal = utils::init_terminal()?;
    let arg_path = utils::path_from_args();
    let mut app = appmain::MainApp::new("RFM", arg_path);
    let _ = run_app(&mut app, &mut terminal);

    
    // Restore the terminal
    disable_raw_mode()?;
    let _ = execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    );
    terminal.show_cursor()?;

    Ok(())
}


fn run_app<B: Backend>(app: &mut appmain::MainApp, terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        // Come up with case for layouts to render
        let _ = terminal.draw(|f| {
            ui_layout::main_layout(app, f);
        });

        // Cleaner function for key events
        if let Event::Key(key) = event::read().unwrap() {
        match key.code {
            KeyCode::Char(c) => {
                match c {
                    'h'|'H' => {
                        app.show_help = !app.show_help;
                    },
                    'a' => {
                        // TESTING try to split the window
                        //split_window(f, )


                        // if app.show_help || app.show_popup { return; }
                        // app.add_file_popup = !app.add_file_popup;
                    }
                    'q' => {
                        if app.show_popup { app.show_popup = false; }
                        else {
                            app.should_quit = true; 
                        }
                    },
                    'j' => { app.list_items.next(); },
                    'k' => { app.list_items.prev(); },
                    'U' => { app.list_items.go_first(); },
                    'D' => { app.list_items.go_last(); },
                    'd' => {
                        app.show_confirmation = true;
                    }
                    _ => {}
                }
            },
            KeyCode::Esc => { if app.show_popup { app.show_popup = false; } }
            KeyCode::Enter => { app.show_popup = true; }
            KeyCode::Tab => { app.should_quit = true; }
            _ => {},
        }
        }
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

// IN PROGRESS:
// TODO: Overall cleanup and styling
// TODO: Display contents of folder
//  
// - ---------------------------------------------
// - ------MVP -----------------------------------
// - ---------------------------------------------
// TODO: Add computer memory information at bottom
// TODO: CLEAN UP EVENT HANDLING!!!!
// TODO: Help menu (dynamic with custom keymaps)
// TODO: Customize colors (fran)
// TODO: Keymaps to do commands
// TODO: Delete folders; empty and not-empty
// TODO: Add Directories
// TODO: Display contents of files
// TODO: Decompression of archives
// TODO: Moving and copying
// TODO: Customize keymaps
// TODO: Search feature
// TODO: Custom commands
// TODO: Sym links
// TODO: Movement across directories
// TODO: Sorting functionality

// --------- DONE -----------------------
// TODO: Confirmation of delete
// TODO: Show icons for file-type (fran) or color code
