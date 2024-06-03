use std::io;
use std::path::PathBuf;

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::Terminal;

use crate::filesystem::{DirectoryList, SelectedFile};
use crate::{ui, utils};

#[derive(Debug, PartialEq)]
pub enum AppState {
    Running,
    Quit,
}

pub enum AppView {
    FileTree,
    FileContent,
}

pub struct App {
    pub title: String,
    pub app_state: AppState,
    pub current_path: PathBuf,
    pub curr_path_entries: DirectoryList,
    pub current_selected_file: Option<SelectedFile>,
    pub app_view: AppView,
    pub scroll_value: u16,
}

impl App {
    pub fn new(title: &str, path: &PathBuf) -> App {
        let curr_entries = match utils::get_dir_entries(path) {
            Ok(entries) => entries,
            Err(_) => Vec::new()
        };

        App {
            title: String::from(title),
            app_state: AppState::Running,
            current_path: path.to_owned(),
            curr_path_entries: DirectoryList::new(curr_entries),
            current_selected_file: None,
            app_view: AppView::FileTree,
            scroll_value: 0,
        }
    }

    fn handle_key_event(&mut self) -> Result<(), io::Error>{
        if let Event::Key(key) = event::read().unwrap() {
            if key.kind == event::KeyEventKind::Release {
            }

            match key.code {
                KeyCode::Char('q') => {
                    self.app_state = AppState::Quit;
                }
                KeyCode::Char('h') => {
                }
                KeyCode::Char('j') => {
                    match self.app_view {
                        AppView::FileTree => {
                            self.curr_path_entries.next();
                        }
                        // Scroll file
                        AppView::FileContent => {
                            match &self.current_selected_file {
                                Some(f) => {
                                    if self.scroll_value < f.line_count - 1 {
                                        self.scroll_value += 1;
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                }
                KeyCode::Char('k') => {
                    match self.app_view {
                        AppView::FileTree => {
                            self.curr_path_entries.prev();
                        }
                        // Scroll file
                        AppView::FileContent => {
                            if self.scroll_value > 0 {
                                self.scroll_value -= 1;
                            }
                        }
                    }
                }
                KeyCode::Char('l') => {}
                KeyCode::Tab => {
                    match self.current_selected_file {
                        Some(_) => {
                            self.scroll_value = 0;
                            self.app_view = match self.app_view {
                                AppView::FileContent => {
                                    AppView::FileTree
                                }
                                AppView::FileTree => {
                                    AppView::FileContent
                                }
                            };
                        }
                        None => {}
                    }
                }
                KeyCode::Enter => {
                    // Select currently highlighted file if file
                    match self.app_view {
                        AppView::FileTree => self.app_view = AppView::FileContent,
                        _ => {}
                    };

                    let state = &self.curr_path_entries.state;
                    let i = match state.selected() {
                        Some(i) => i,
                        None => 0
                    };

                    let selected_entry = &self.curr_path_entries.items[i];
                    if selected_entry.path().is_dir() {
                        // Handle changing dirs
                        self.current_selected_file = None;
                    }
                    // other file types handle here TODO
                    else {
                        self.current_selected_file = Some(SelectedFile::from_path(selected_entry.path()));
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw<B: Backend> (&self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        let _ = terminal.draw(|f| {
            ui::render_ui(f, self)
        });

        Ok(())
    }

    // TODO: Error handling
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<bool> {
        while self.app_state != AppState::Quit {
            self.draw(terminal)?;
            self.handle_key_event()?;
        }

        Ok(true)
    }

}
