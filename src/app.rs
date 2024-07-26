use std::io;
use std::path::PathBuf;

use color_eyre::{
    eyre::WrapErr,
    Result
};

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::widgets::ScrollbarState;
use ratatui::Terminal;

use crate::filesystem::{DirectoryList, SelectedFile};
use crate::ui;

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
    pub file_view_scroll_state: ScrollbarState,
    pub app_view: AppView,
    pub scroll_value: usize,
    pub scroll_offset: usize,
}

impl App {
    pub fn new(title: &str, path: &PathBuf, scroll_offset: usize) -> App {
        App {
            title: String::from(title),
            app_state: AppState::Running,
            current_path: path.to_owned(),
            curr_path_entries: DirectoryList::new(path),
            current_selected_file: None,
            app_view: AppView::FileTree,
            scroll_value: 0,
            file_view_scroll_state: ScrollbarState::default(),
            scroll_offset, 
        }
    }

    fn update_scroll_state(&mut self, value: usize) {
        self.scroll_value = value;
        self.file_view_scroll_state = 
            self.file_view_scroll_state.position(value);
    }

    fn can_scroll(&mut self, line_count: usize) -> bool {
        line_count > self.scroll_offset && self.scroll_value < (line_count - self.scroll_offset)
    }

    fn page_jump_up(&mut self) {
    }

    fn page_jump_down(&mut self) {
    }

    fn handle_events(&mut self) -> Result<()>{
        match event::read()? {
            Event::Key(key_event) if key_event.kind == event::KeyEventKind::Press => self
                .handle_key_event(key_event)
                .wrap_err_with(|| format!("handling key event failed:\n{key_event:#?}")),
            _ => Ok(()),
        }
    }

    fn handle_key_event(&mut self, key_event: event::KeyEvent) -> Result<()>{
        // if let Event::Key(key) = event::read().unwrap() {
        //     if key.kind == event::KeyEventKind::Release {
        //     }

        match key_event.code {
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
                                if self.can_scroll(f.line_count) {
                                    self.update_scroll_state((self.scroll_value) + 1);
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
                            self.update_scroll_state((self.scroll_value) - 1);
                        }
                    }
                }
            }
            // Jump Up by 5th of page? TODO:
            KeyCode::Char('u') => {
                match self.app_view {
                    // Scroll file
                    AppView::FileContent => {
                        self.page_jump_up();
                    },
                    _ => {}
                }
            }
            // Jump down by 5th of page? TODO:
            KeyCode::Char('d') => {
                match self.app_view {
                    // Scroll file
                    AppView::FileContent => {
                        self.page_jump_down();
                    },
                    _ => {}
                }
            }
            // Top of file
            KeyCode::Char('g') => {
                match self.app_view {
                    AppView::FileContent => {
                        self.update_scroll_state(0)
                    },
                    _ => {}
                }
            }
            // Bottom of file
            KeyCode::Char('G') => {
                match self.app_view {
                    AppView::FileContent => {
                        match &self.current_selected_file {
                            Some(f) => {
                                let lc = f.line_count;
                                if self.can_scroll(lc) {
                                    self.update_scroll_state(lc - self.scroll_offset)
                                }
                            },
                            None => {}
                        }
                    },
                    _ => {}
                }
            }
            KeyCode::Char('l') => {}
            KeyCode::Tab => {
                match self.current_selected_file {
                    Some(_) => {
                        self.scroll_value = 0;
                        self.file_view_scroll_state = 
                            self.file_view_scroll_state.position(self.scroll_value);
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
                    AppView::FileTree => {
                        self.app_view = AppView::FileContent;
                    },
                    AppView::FileContent => {
                        self.app_view = AppView::FileTree;
                    }
                };

                // Reset scroll
                self.scroll_value = 0;

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
            let _ = self.handle_events();
        }

        Ok(true)
    }

}
