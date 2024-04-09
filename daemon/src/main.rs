pub mod database;

use battery::{Battery, Manager};
use daemon::*;
use common::Config;
use database::{create_event, create_session, initialize_tables};
use rusqlite::Connection;
use std::thread::sleep;

fn main() -> Result<(), battery::Error> {
    let config = Config::new();

    let conn = Connection::open(&config.log_path)
        .expect("Failed to connect to database. Check permissions.");

    initialize_tables(&conn).expect("Failed to initialize tables");

    let manager = Manager::new().expect("Failed to create battery manager");
    let batteries = manager
        .batteries()
        .expect("Failed to get batteries")
        .map(|b| b.expect("Failed to get battery"))
        .collect::<Vec<Battery>>();

    let current_status = batteries.get(0).unwrap().state();
    let session = create_session(&current_status, &conn).expect("Failed to create initial session");

    main_loop(manager, batteries, conn, session, &config)
}

fn main_loop(
    mut manager: Manager,
    mut batteries: Vec<Battery>,
    conn: Connection,
    mut session: usize,
    config: &Config,
) -> Result<(), battery::Error> {
    loop {
        let current_status = batteries.get(0).unwrap().state();

        if let Ok(()) = refresh_battery_info(&mut manager, &mut batteries) {
            let updated_status = batteries.get(0).unwrap().state();

            if current_status != updated_status {
                if let Ok(s) = create_session(&updated_status, &conn) {
                    session = s;
                } else {
                    eprintln!("Failed to create new session");
                }
            }

            let total_capacity = get_current_battery(&batteries);
            println!("Total capacity: {}", total_capacity);
            let powerdraw = get_powerdraw(&batteries);
            if let Err(e) = create_event(&total_capacity, &powerdraw, &session, &conn) {
                eprintln!("Failed to create event: {}", e);
            }
        }

        sleep(config.polling_interval);
    }
}
