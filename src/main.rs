use std::io;
#[allow(unused_imports)]
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
    widgets::{Widget, Block, Borders, List, ListItem, ListState},
    layout::{Layout, Constraint, Direction, Rect},
    style::{Style, Modifier, Color},
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


mod appmain;
mod listitem;
mod utils;


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
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(95),
                    ].as_ref()
                )
                .split(f.size());

            let block = Block::default()
                .title(app.title)
                .borders(Borders::TOP);
            f.render_widget(block, chunks[0]);

            let its: Vec<ListItem> = app.list_items.items
                .iter()
                .map(|i|
                    ListItem::new(i.as_ref())
                )
                .collect();
            let list = List::new(its)
                .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("~> ");
            f.render_stateful_widget(list, chunks[1], &mut app.list_items.state);
        });

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'q' => {
                            app.should_quit = true;
                        },
                        'j' => {
                            app.list_items.next();
                        },
                        'k' => {
                            app.list_items.prev();
                        }
                        'U' => {
                            app.list_items.go_first();
                        },
                        'D' => {
                            app.list_items.go_last();
                        },
                        _ => {}
                    }
                },
                KeyCode::Esc => {
                    app.list_items.unselect();
                }
                _ => {},
            }
        }

        if app.should_quit {
            return Ok(())
        }
    }

}

