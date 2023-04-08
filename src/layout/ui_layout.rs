use crate::{appmain, utils::{self, get_file_name, get_file_type}};
use tui::{
    Frame,
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
                .horizontal_margin(2)
                .vertical_margin(1)
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
                .borders(Borders::TOP)
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(block, chunks[0]);

            let mut its: Vec<ListItem> = vec![];
            for (file, _type) in app.list_items.items.iter() {
                // its.push(ListItem::new(file.to_owned()));
                let file_string: String = format!("{}  {}", get_file_type(_type).to_owned(), file.to_owned());
                // file.to_owned() + get_file_type(_type);
                its.push(ListItem::new(file_string));
            }
            let list = List::new(its)
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("");
            f.render_stateful_widget(list, chunks[1], &mut app.list_items.state);

            let help_block = Block::default()
                .title("Press 'h' to toggle Help Menu | Press q to exit")
                .borders(Borders::NONE);
            f.render_widget(help_block, chunks[2]);

            if app.show_popup {
                let t: String = match app.list_items.state.selected() {
                    None => {
                        String::from(" No Item Selected ")
                    },
                    Some(s) => {
                        get_file_name(app, s)
                    }
                };
                let pop = Block::default().title(t + " (press q to close) ")
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
            
            if app.show_confirmation {
                confirm_layout(true, app, f);
            }
        });

        if let Event::Key(key) = event::read().unwrap() {
            if app.add_file_popup {
                match key.code {
                    KeyCode::Char(c) => { app.input.add(c); },
                    KeyCode::Esc => { app.add_file_popup = false; },
                    KeyCode::Backspace => { app.input.delete(); },
                    KeyCode::Enter => { 
                        utils::add_file(app);
                        app.refresh_items();
                    },
                    _ => {},
                }
            } else if app.show_confirmation {
                match key.code {
                    KeyCode::Char(c) => {
                        match c {
                            'y' => {
                                let idx = match app.list_items.state.selected() {
                                    Some(idx) => {
                                        utils::delete_file(app, idx);
                                        app.show_confirmation = false;
                                    },
                                    None => {}
                                };
                            }
                            _ => { app.show_confirmation = false; }
                        };
                    }
                    _ => {}
                };

            }
            else {

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
    }

}

fn confirm_layout<B: Backend>(is_deleting: bool, app: &mut appmain::MainApp, f: &mut Frame<B>) {

        let idx = app.list_items.state.selected().unwrap();
        let file_name = get_file_name(app, idx);
        let text: String = format!("About to delete file: {} Are you sure?", file_name);
        let area = centered_rect::centered(40, 20, f.size());
        let confirmation_window = Paragraph::new(text.to_owned())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Confirmation Window. Press y to confirm. Any key to escape.")
                .style(Style::default().fg(Color::Red)));
        
        f.render_widget(Clear, area);
        f.render_widget(confirmation_window, area);
}
