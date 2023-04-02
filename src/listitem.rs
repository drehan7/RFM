use tui::widgets::ListState;

pub struct ListItems {
    pub items: Vec<String>,
    pub state: ListState,
}

impl ListItems {
    pub fn from_items(items: Vec<String>) -> ListItems {
        ListItems {
            items,
            state: ListState::default(),
        }
    }

    pub fn go_first(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_last(&mut self) {
        let i = self.items.len() -1;
        self.state.select(Some(i));
    }

    pub fn next(&mut self) {
        let idx = match self.state.selected() {
            Some(idx) => {
                if idx >= self.items.len() - 1 {
                    self.items.len() - 1
                } else {
                    idx + 1
                }

            },
            None => {
                0
            }
        };

        self.state.select(Some(idx));
    }

    pub fn prev(&mut self) {
        let idx = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    0 
                } else {
                    i - 1
                }
            },
            None => 0,
        };

        self.state.select(Some(idx));
    }
}
