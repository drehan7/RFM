use crate::{appmain, utils};

use tui::{
    backend::Backend,
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
    layout::{Layout, Direction, Constraint},
    style::{Style, Color, Modifier},
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};
use unicode_width::UnicodeWidthStr;
#[path ="./centered_rect.rs"]
mod centered_rect;


pub fn main_layout<B: Backend>(app: &mut appmain::MainApp, terminal: &mut Terminal<B>) {
        let _ = terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(94),
                        Constraint::Percentage(1),
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
                ).collect();
            let list = List::new(its.as_ref())
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("");
            f.render_stateful_widget(list, chunks[1], &mut app.list_items.state);

            let help_block = Block::default()
                .title("Press 'h' to toggle Help Menu | Press q to exit")
                .borders(Borders::NONE);
            f.render_widget(help_block, chunks[2]);

            if app.show_popup {
                let t = match app.list_items.state.selected() {
                    None => {
                        " No Item Selected "
                    },
                    Some(s) => {
                        &app.list_items.items[s]
                    }
                };
                let pop = Block::default().title(t.to_owned() + " (press q to close) ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                let area = centered_rect::centered(60, 20, f.size());
                f.render_widget(Clear, area);
                f.render_widget(pop, area);
            }
            
            if app.show_help {
                let help_menu = Block::default().title("Help Menu")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                let area = centered_rect::centered(80, 60, f.size());
                f.render_widget(Clear, area);
                f.render_widget(help_menu, area);
            }
            
            if app.add_file_popup {
                let add_file_menu = Paragraph::new(app.input.input.as_ref())
                    .block(Block::default().title(" Add file? ").borders(Borders::ALL).border_type(BorderType::Rounded))
                    .style(Style::default().fg(Color::Blue));
                let area = centered_rect::centered(35, 7, f.size());
                f.render_widget(Clear, area);
                f.render_widget(add_file_menu, area);
                f.set_cursor(area.x + app.input.input.width() as u16 + 1, area.y + 1);
                
            }
        });

        if let Event::Key(key) = event::read().unwrap() {
            if app.add_file_popup {
                match key.code {
                    // KeyCode::Char(c) => { _inp.input += &format!("{}",c); },
                    KeyCode::Char(c) => { app.input.add(c); },
                    KeyCode::Esc => { app.add_file_popup = false; },
                    KeyCode::Enter => { 
                        utils::add_file(app);
                        app.refresh_items();
                    },
                    KeyCode::Backspace => { app.input.delete(); }
                    _ => {},
                }
            } else {

            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'h'|'H' => {
                            if app.add_file_popup || app.show_popup { return; }
                            app.show_help = !app.show_help;
                        },
                        'a' => {
                            if app.show_help || app.show_popup { return; }
                            app.add_file_popup = !app.add_file_popup;
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
                            match app.list_items.state.selected() {
                                Some(s) => {
                                    utils::delete_file(app, s);
                                },
                                None => {},
                            };
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
        }
}
