use battery::State;
use rusqlite::Connection;

pub fn create_session(current_state: &State, conn: &Connection) -> Result<usize, rusqlite::Error> {
    let query = format!(
        "INSERT INTO sessions (session_type) VALUES ('{}')",
        current_state.to_string()
    );

    conn.execute(&query, [])?;

    Ok(conn.last_insert_rowid() as usize)
}

pub fn create_event(
    capacity: &u32,
    power_draw: &u32,
    session_id: &usize,
    conn: &Connection,
) -> Result<usize, rusqlite::Error> {
    let query = format!(
        "INSERT INTO events (session_id, capacity, power_draw) VALUES ({}, {}, {})",
        session_id, capacity, power_draw
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
            power_draw FLOAT
        )",
        [],
    )
}
