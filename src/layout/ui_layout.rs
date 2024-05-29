use crate::{appmain, utils::{get_file_name, get_file_type}};

use tui::{
    Frame,
    backend::Backend,
    widgets::{Block, Borders, BorderType, Clear, List, ListItem, Paragraph},
    layout::{Layout, Direction, Constraint, Alignment},
    style::{Style, Color, Modifier},
};
use unicode_width::UnicodeWidthStr;

#[path ="./centered_rect.rs"]
mod centered_rect;


pub fn main_layout<B: Backend>(app: &mut appmain::MainApp, f: &mut Frame<B> ) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .horizontal_margin(5)
        .vertical_margin(1)
        .constraints(
            [
                Constraint::Percentage(5),
                Constraint::Percentage(95),
            ].as_ref()
        )
        .split(f.size());


    let title = Block::default()
        .title(app.title)
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::Yellow));

    let mut its: Vec<ListItem> = vec![];
    for (file, _type) in app.list_items.items.iter() {
        let file_string: String = format!("{}  {}", 
            get_file_type(_type).to_owned(), 
            file.to_owned());
        its.push(ListItem::new(file_string));
    }

    let file_system_list = List::new(its)
        .highlight_style(Style::default()
            .add_modifier(Modifier::BOLD | Modifier::ITALIC | Modifier::UNDERLINED)
            .fg(Color::White))
        .highlight_symbol("");


    f.render_widget(title, chunks[0]);
    f.render_stateful_widget(file_system_list, chunks[1], &mut app.list_items.state);

    handle_app_flags(app, f);
}

// If file is selected, then show preview of contents for now
// If directory show files in dir.
// If Sym: not sure yet
fn select_item_popup<B: Backend>(app: &mut appmain::MainApp, f: &mut Frame<B>) {
    let item_str: String = match app.list_items.state.selected() {
        Some(s) => {
            get_file_name(app, s)
        }
        None => { String::from(" No Item Selected ")}
    };

    let popup_rect = Block::default()
        .title(item_str + " | Press Esc to close ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let area = centered_rect::centered(80, 30, f.size());
    f.render_widget(Clear, area);
    f.render_widget(popup_rect, area);
}

fn add_file_layout<B: Backend>(app: &mut appmain::MainApp, f: &mut Frame<B>) {
    let add_file_menu = Paragraph::new(app.input.input.as_ref())
        .alignment(Alignment::Center)
        .block(Block::default().title(" Add file: | Esc to exit")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title_alignment(Alignment::Left))
        .style(Style::default().fg(Color::Blue));

    let area = centered_rect::centered(35, 7, f.size());

    f.render_widget(Clear, area);
    f.render_widget(add_file_menu, area);

    let cursor_start = (area.width / 2) + ((app.input.input.width() as u16 + 1) / 2) + area.x;
    f.set_cursor(cursor_start , area.y + 1);
}

fn confirm_layout<B: Backend>(_is_deleting: bool, app: &mut appmain::MainApp, f: &mut Frame<B>) {
    let idx = app.list_items.state.selected().unwrap();
    let file_name = get_file_name(app, idx);

    let text: String = format!("\nAbout to delete `{}` | Are you sure?\n(y/n)", file_name);

    let area = centered_rect::centered(25, 9, f.size());

    let confirmation_window = Paragraph::new(text.to_owned())
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title("Confirm Action")
            .title_alignment(Alignment::Center)
            .style(Style::default().fg(Color::Red)));
    
    f.render_widget(Clear, area);
    f.render_widget(confirmation_window, area);
}

fn handle_app_flags<B: Backend>(app: &mut appmain::MainApp, f: &mut Frame<B>) {
    if app.show_popup {
        select_item_popup(app, f);
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
        add_file_layout(app, f);
        
    }
    
    if app.show_confirmation {
        confirm_layout(true, app, f);
    }
}
