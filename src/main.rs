pub mod config;
pub mod database;

use batt_log::Power;
use config::Config;
use database::{create_event, create_session, initialize_tables};
use rusqlite::Connection;
use std::{thread::sleep, time::Duration};

fn main() {
    let config = Config::new();

    let conn = Connection::open(&config.db_path)
        .expect("Failed to connect to database. Check permissions.");

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

        sleep(Duration::from_secs(1));
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
