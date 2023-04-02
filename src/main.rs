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
mod ui;

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
        let _ = ui::ui(app, terminal);
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

// TODO: Help menu
// TODO: Display contents of folder
// TODO: Keymaps to do commands
