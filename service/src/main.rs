pub mod database;

use batt_log::Power;
use database::{create_event, create_session, initialize_tables};
use rusqlite::Connection;
use std::{thread::sleep, time::Duration};

const POLLING_INTERVAL: Duration = Duration::from_secs(60);
const DB_PATH: &str = "/home/marco/.cache/batt.db";

fn main() {
    let conn = Connection::open(DB_PATH).expect("Failed to connect to database");

    initialize_tables(&conn).expect("Failed to initialize tables");

    let mut power: Power = Power::default();

    power.update().unwrap_or_else(|e| {
        eprintln!("Failed to initialize power: {}", e);
        std::process::exit(1);
    });

    let session = create_session(&power, &conn).expect("Failed to create initial session");

    main_loop(power, conn, session)
}

fn main_loop(mut power: Power, conn: Connection, mut session: usize) -> ! {
    loop {
        let curr = power.status.clone();

        if let Ok(_) = power.update() {
            change_session_if_status_changed(curr, &power, &conn, &mut session);

            if let Err(e) = create_event(&power, &conn, &session) {
                eprintln!("Failed to create event: {}", e);
            }
        }

        sleep(POLLING_INTERVAL);
    }
}

fn change_session_if_status_changed(
    curr: batt_log::Status,
    power: &Power,
    conn: &Connection,
    session: &mut usize,
) {
    if curr != power.status {
        if let Ok(s) = create_session(power, conn) {
            *session = s;
        } else {
            eprintln!("Failed to create new session");
        }
    }
}
