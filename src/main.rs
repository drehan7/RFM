use std::io;
use tui::{ backend::Backend, Terminal };
use crossterm::{
    execute,
    event::DisableMouseCapture,
    terminal::{disable_raw_mode, enable_raw_mode, LeaveAlternateScreen},
};


mod appmain;
mod listitem;
mod utils;
mod layout;
mod input;

fn main() -> Result<(), io::Error>  {
    enable_raw_mode()?;
    let mut terminal = utils::init_terminal()?;
    let arg_path = utils::path_from_args();
    let mut app = appmain::MainApp::new("RFM", arg_path);
    let _ = run_app(&mut app, &mut terminal);

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
        let _ = layout::ui_layout::main_layout(app, terminal);
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
