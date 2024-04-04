use batt_log::Power;
use rusqlite::Connection;

pub fn create_session(power: &Power, conn: &Connection) -> Result<usize, rusqlite::Error> {
    let query = format!(
        "INSERT INTO sessions (session_type) VALUES ('{}')",
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
        "INSERT INTO events (session_id, capacity, power_draw) VALUES ({}, {}, {})",
        session_id, power.capacity, power.power_draw
    );

    conn.execute(&query, [])?;

    Ok(conn.last_insert_rowid() as usize)
}

pub fn initialize_tables(conn: &Connection) -> Result<usize, rusqlite::Error> {
    initialize_session_table(conn)?;
    initialize_event_table(conn)
}

fn initialize_session_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sessions (
            id INTEGER PRIMARY KEY,
            session_type TEXT
        )",
        [],
    )
}

fn initialize_event_table(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS events (
            id INTEGER PRIMARY KEY,
            session_id INTEGER,
            timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            capacity INTEGER,
            power_draw INTEGER
        )",
        [],
    )
}
