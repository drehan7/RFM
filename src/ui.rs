use std::fs::read_to_string;

use crate::app;

use ratatui::widgets::ListItem;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{ Color, Modifier, Style },
    widgets::{ Block, Borders, List, ListDirection, Paragraph },
    text::Text,
};


fn render_header_block (f: &mut Frame, rect: &Rect, app: &app::App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        app.title.to_owned(),
        Style::default().fg(Color::Yellow),
    ))
    .block(title_block);
    f.render_widget(title, *rect);
}

fn render_main_block(f: &mut Frame, rects: &Vec<Rect>, app: &app::App) {
    let files: Vec<ListItem> = app.curr_path_entries.items
        .iter()
        .map(|f| 
            ListItem::new(f.path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned())
        )
        .collect();

    let file_view_header = app.current_path.clone()
        .into_os_string()
        .to_str()
        .unwrap()
        .to_owned();

    let list = List::new(files)
        .block(Block::bordered().title(file_view_header))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("-> ")
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

    let state = &app.curr_path_entries.state;

    // Render file contents
    if rects.len() > 1 {
        match &app.current_selected_file {
            Some(file) => {
                let paragraph = Paragraph::new(file.contents.clone())
                    .style(Style::default().fg(Color::White).bg(Color::Black))
                    .block(Block::bordered())
                    .scroll((app.scroll_value, 0));

                f.render_widget(paragraph, rects[1]);
            }
            None => {}
        }
    }

    f.render_stateful_widget(list, rects[0], &mut state.to_owned());

}

fn render_footer_block (f: &mut Frame, rect: &Rect, _app: &app::App) {
    let footer_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let footer = Paragraph::new(Text::styled(
        "Footer Template",
        Style::default().fg(Color::LightGreen),
    ))
    .block(footer_block);

    f.render_widget(footer, *rect);
}

pub fn render_ui(f: &mut Frame, app: &app::App ) {
    let area = f.size();

    // Top header, middle main, bottom footer
    let horizontal = Layout::vertical([Constraint::Percentage(5), Constraint::Percentage(90), Constraint::Percentage(5)]);
    let vertical = match app.app_view {
        app::AppView::FileContent => {
            Layout::horizontal([Constraint::Ratio(1,2),Constraint::Ratio(1,2)])
        }
        app::AppView::FileTree => {
            Layout::horizontal([Constraint::Ratio(1,1)])
        }
    };

    let [header, main, footer] = horizontal.areas(area);

    let ff = match app.app_view {
        app::AppView::FileContent => {vertical.areas::<2>(main).to_vec()}
        app::AppView::FileTree => {vertical.areas::<1>(main).to_vec()}
    };

    render_header_block(f, &header, app);
    render_main_block(f, &ff, app); // Display file entries
    render_footer_block(f, &footer, app);
}
