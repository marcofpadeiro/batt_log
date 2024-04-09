use std::{collections::HashMap, error::Error};

use ratatui::widgets::ListState;
use rusqlite::{params, Connection};

const ITEMS_PER_PAGE: usize = 20;

pub struct SessionList {
    pub state: ListState,
    items: Vec<Session>,
    last_selected: Option<usize>,
    items_per_page: usize,
    current_page: usize,
    connection: Connection,
}

enum SessionType {
    Discharging,
    Charging,
}

impl SessionType {
    pub fn from_string(string: &str) -> Option<Self> {
        match string {
            "Discharging" => Some(SessionType::Discharging),
            "Charging" => Some(SessionType::Charging),
            _ => None,
        }
    }
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
    pub fn new(connection: Connection) -> Self {
        Self {
            state: ListState::default(),
            items: get_sessions_from_database(&connection, 1, ITEMS_PER_PAGE)
                .unwrap()
                .unwrap_or_else(|| vec![]),
            last_selected: None,
            items_per_page: ITEMS_PER_PAGE,
            current_page: 1,
            connection,
        }
    }

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

    pub fn previous_page(&mut self) {
        self.state.select(Some(0));

        if self.current_page > 1 {
            self.items = get_sessions_from_database(
                &self.connection,
                self.current_page - 1,
                self.items_per_page,
            )
            .unwrap()
            .unwrap();
            self.current_page -= 1;
        }
    }

    pub fn next_page(&mut self) {
        self.state.select(Some(0));

        if let Some(items) =
            get_sessions_from_database(&self.connection, self.current_page - 1, self.items_per_page)
                .unwrap()
        {
            self.items = items;
            self.current_page += 1;
        }
    }

    pub fn go_top(&mut self) {
        self.state.select(Some(0));
    }

    pub fn go_bottom(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }
}

fn get_sessions_from_database(
    connection: &Connection,
    current_page: usize,
    items_per_page: usize,
) -> Result<Option<Vec<Session>>, Box<dyn Error>> {
    let mut stmt = connection.prepare("SELECT * FROM sessions INNER JOIN events ON sessions.id = events.session_id WHERE sessions.id IN (SELECT id FROM sessions ORDER BY id LIMIT ?, ?) ORDER BY sessions.id")?;
    let mut found = false;

    let mut sessions_map = HashMap::new();

    let rows = stmt.query_map(
        params![current_page * items_per_page, items_per_page],
        |row| {
            found = true;
            let session_id: u32 = row.get(0)?;
            let session_type: String = row.get(1)?;
            let event = Event {
                id: row.get(2)?,
                session_id,
                timestamp: row.get(4)?,
                capacity: row.get(5)?,
                power_draw: row.get(5)?,
            };
            let session = sessions_map.entry(session_id).or_insert_with(|| Session {
                id: session_id,
                session_type: SessionType::from_string(session_type.as_str())
                    .unwrap_or(SessionType::Discharging),
                events: Vec::new(),
            });

            session.events.push(event);

            Ok(())
        },
    )?;

    for _ in rows {}

    if !found {
        return Ok(None);
    }

    Ok(Some(sessions_map.into_values().collect()))
}
