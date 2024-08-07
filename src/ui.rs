use crate::{app, utils, syntax, filesystem};

use std::fs::DirEntry;
use ratatui::layout::Margin;
use ratatui::symbols::scrollbar;
use ratatui::text::{Line, Span};
use ratatui::widgets::block::Title;
use ratatui::widgets::{ListItem, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{ Color, Modifier, Style },
    widgets::{ Block, Borders, List, ListDirection, Paragraph },
    text::Text,
};

// Add unicode to entries
fn displayable_path( entry: &DirEntry ) -> String {
    let mut unicode = match entry.file_type() {
        Ok(ft) => {
            if ft.is_dir() {
                String::from(utils::DIRECTORY_UNICODE)
            }
            else {
                String::from(utils::FILE_UNICODE)
            }
        },
        Err(_) => {
            String::from("")
        }
    };

    match entry.file_name().to_str() {
        Some(s) => {
            unicode.push_str(s);
        }
        None => {}
    }

    return unicode;
}

// Take in file contents and app, hightight file using syntect
// Return paragraph containing highlighted text
fn apply_syntax<'a>(app: &'a app::App, file: &'a filesystem::SelectedFile) -> Paragraph<'a> {
    let syn_file = syntax::highlight_file(&app.syntax_helper, &file.path, &file.contents);

    let mut syntax_lines = vec![];

    for line in syn_file {
        let mut combined_line: Vec<Span> = vec![];
        for word in line {
            let foreground = word.0.foreground;
            let text = word.1;
            let vv = Span::styled(text, Style::default()
                .fg(Color::Rgb(foreground.r, foreground.g, foreground.b)));

            combined_line.push(vv);
        }

        syntax_lines.push(
                Line::from(combined_line)
            );
    }

    let title = match file.path.to_str() {
        Some(path) => path,
        None => "" // No path?
    };

    Paragraph::new(syntax_lines)
        .style(Style::default().fg(Color::White))
        .block(Block::bordered()
            .title(Title::from(title)))
        .scroll((app.scroll_value.try_into().unwrap(), 0))
}

fn render_scroll_bar (f: &mut Frame, rect: &Rect, scroll_state: &mut ScrollbarState) {
    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .symbols(scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"))
            .style(Style::default().fg(Color::Cyan)),
        rect.inner(&Margin {
            vertical: 1,
            horizontal: 0,
        }),
        scroll_state
    );
}

fn render_header_block (f: &mut Frame, rect: &Rect, app: &app::App) {
    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut lines = vec![];
    lines.push(Line::from(vec![
            Span::styled(String::from(app.current_path.to_str().unwrap()), Style::default().fg(Color::Yellow))
    ]));

    let title = Paragraph::new(Text::from(lines)).block(title_block);

    f.render_widget(title, *rect);
}

fn render_main_block(f: &mut Frame, rects: &Vec<Rect>, app: &app::App) {
    let files: Vec<ListItem> = app.curr_path_entries.items
        .iter()
        .map(|f| ListItem::new(displayable_path(f)))
        .collect();

    // Render entries
    let list = List::new(files)
        .block(Block::bordered())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default()
        .add_modifier(Modifier::BOLD | Modifier::ITALIC))
        .highlight_symbol(utils::RIGHT_ARROW_UNICODE) // TODO CONFIGURABLE
        .direction(ListDirection::TopToBottom); // TODO MAKE THIS CONFIGURABLE

    let state = &app.curr_path_entries.state;

    // Render file contents
    if rects.len() > 1 {
        match &app.current_selected_file {
            Some(file) => {
                let paragraph = apply_syntax(app, file);

                let mut scroll_state = match file.line_count > app.scroll_offset {
                    true => {
                        app.file_view_scroll_state.content_length(file.line_count - app.scroll_offset)
                    },
                    false => {
                        app.file_view_scroll_state.content_length(file.line_count)
                    }
                };
                       
                f.render_widget(paragraph, rects[1]);
                render_scroll_bar(f, &rects[1], &mut scroll_state);
            }
            None => {
                // No file selected 
            }
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
