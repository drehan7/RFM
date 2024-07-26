
use std::io::{self, Stderr};

use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::{
        execute,
        terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen}
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal
};

pub type Tui = Terminal<CrosstermBackend<Stderr>>;

pub fn init() -> io::Result<Tui> {
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let term = Terminal::new(backend)?;

    Ok(term)
}

pub fn restore() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        io::stderr(),
        LeaveAlternateScreen,
        DisableMouseCapture,
    )?;
    // terminal.show_cursor()?;

    Ok(())
}
