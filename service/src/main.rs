use std::{thread::sleep, time::Duration};

use batt_log::{create_event, create_session, initialize_tables, Power};
use rusqlite::Connection;

const POLLING_INTERVAL: u8 = 60; // seconds
const DB_PATH: &str = "/home/marco/.cache/batt.db";

fn main() {
    let conn = Connection::open(DB_PATH).expect("Failed to connect to database");

    initialize_tables(&conn).unwrap_or_else(|e| {
        eprintln!("Failed to initialize tables: {}", e);
        std::process::exit(1);
    });

    let mut power: Power = Power::default();
    power.update().unwrap_or_else(|e| {
        eprintln!("Failed to get power levels: {}", e);
        std::process::exit(1);
    });

    let mut session = match create_session(&power, &conn) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to create new session: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        let curr = power.status.clone();
        if let Err(e) = power.update() {
            eprintln!("Error: {}", e);
            sleep(Duration::from_secs(POLLING_INTERVAL.into()));
            continue;
        }

        if curr != power.status {
            if let Ok(s) = create_session(&power, &conn) {
                session = s;
            } else {
                eprintln!("Failed to create new session");
            }
        }

        if let Err(e) = create_event(&power, &conn, &session) {
            eprintln!("Failed to create event: {}", e);
        }

        sleep(Duration::from_secs(POLLING_INTERVAL.into()));
    }
}
