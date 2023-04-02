use crate::{appmain, input};

use tui::{
    backend::Backend,
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
    layout::{Layout, Direction, Constraint, Rect},
    style::{Style, Color, Modifier},
    Terminal,
};
use crossterm::event::{self, Event, KeyCode};

pub fn ui<B: Backend>(app: &mut appmain::MainApp, terminal: &mut Terminal<B>) {
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
            let list = List::new(its)
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC | Modifier::BOLD).fg(Color::Yellow))
                .highlight_symbol("");
            f.render_stateful_widget(list, chunks[1], &mut app.list_items.state);

            let help_block = Block::default()
                .title("Press 'h' to toggle Help Menu")
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
                let area = centered_rect(60, 20, f.size());
                f.render_widget(Clear, area);
                f.render_widget(pop, area);
            }
            
            if app.show_help {
                let help_menu = Block::default().title("Help Menu")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                let area = centered_rect(80, 60, f.size());
                f.render_widget(Clear, area);
                f.render_widget(help_menu, area);
            }
            
            if app.add_file_popup {
                let _inp = input::Input::new();
                let add_file_menu = Block::default()
                    .title(" Add file? ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded);
                let area = centered_rect(30, 5, f.size());
                f.render_widget(Clear, area);
                f.render_widget(add_file_menu, area);
            }
        });

        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char(c) => {
                    match c {
                        'q' => {
                            if app.show_popup {
                                app.show_popup = false;
                            }
                        },
                        'h'|'H' => {
                            app.show_help = !app.show_help;
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
                        'a' => {
                            app.add_file_popup = !app.add_file_popup;
                        }
                        _ => {}
                    }
                },
                KeyCode::Esc => {
                    app.should_quit = true;
                }
                KeyCode::Enter => {
                    app.show_popup = true;
                }
                _ => {},
            }
        }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let layout_popup = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref()
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref()
        )
        .split(layout_popup[1])[1]
}
