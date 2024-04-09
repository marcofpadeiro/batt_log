use ratatui::widgets::ListState;

pub struct SessionList {
    pub state: ListState,
    items: Vec<Session>,
    last_selected: Option<usize>,
}

enum SessionType {
    Discharging,
    Charging,
}

struct Session {
    id: u32,
    session_type: SessionType,
    events: Vec<Event>,
}

struct Event {
    id: u32,
    session_id: u32,
    timestamp: String,
    capacity: u32,
    power_draw: f32,
}

impl SessionList {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }
    pub fn go_top(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}
