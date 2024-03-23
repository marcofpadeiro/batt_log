use rusqlite::Connection;

const CAPACITY_PATH: &str = "/sys/class/power_supply/BAT0/capacity";
const STATUS_PATH: &str = "/sys/class/power_supply/BAT0/status";
const POWERDRAW_PATH: &str = "/sys/class/power_supply/BAT0/power_now";

#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Charging,
    Discharging,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Charging => write!(f, "Charging"),
            Status::Discharging => write!(f, "Discharging"),
        }
    }
}

pub struct Power {
    pub capacity: u32,
    pub status: Status,
    pub power_draw: u32,
}

impl Power {
    pub fn default() -> Self {
        Self {
            capacity: 0,
            status: Status::Discharging,
            power_draw: 0,
        }
    }

    pub fn update(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let capacity = std::fs::read_to_string(CAPACITY_PATH)?.trim().parse()?;
        let status = match std::fs::read_to_string(STATUS_PATH).unwrap().trim() {
            "Charging" => Status::Charging,
            "Discharging" => Status::Discharging,
            _ => return Err("Unknown status".into()),
        };
        let power_draw = std::fs::read_to_string(POWERDRAW_PATH)?.trim().parse()?;

        self.capacity = capacity;
        self.status = status;
        self.power_draw = power_draw;

        Ok(())
    }
}

pub fn create_session(power: &Power, conn: &Connection) -> Result<usize, rusqlite::Error> {
    let query = format!(
        "INSERT INTO session (session_type) VALUES ('{}')",
        power.status.to_string()
    );

    conn.execute(&query, [])?;

    Ok(conn.last_insert_rowid() as usize)
}

pub fn create_event(
    power: &Power,
    conn: &Connection,
    session_id: &usize,
) -> Result<usize, rusqlite::Error> {
    let query = format!(
        "INSERT INTO event (session_id, capacity, power_draw) VALUES ({}, {}, {})",
        session_id, power.capacity, power.power_draw
    );

    conn.execute(&query, [])?;

    Ok(conn.last_insert_rowid() as usize)
}

pub fn initialize_tables(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS session (
            id INTEGER PRIMARY KEY,
            session_type TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS event (
            id INTEGER PRIMARY KEY,
            session_id INTEGER,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            capacity INTEGER,
            power_draw INTEGER
        )",
        [],
    )?;

    Ok(())
}
