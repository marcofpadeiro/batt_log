pub mod app;
pub mod render;
pub mod terminal;

use std::error::Error;

use app::App;
use common::Config;
use rusqlite::Connection;
use terminal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::new();

    let conn = Connection::open(&config.log_path)
        .expect("Failed to connect to database. Check permissions.");

    let terminal = init_terminal()?;

    App::new(conn).run(terminal)?;

    restore_terminal()?;

    Ok(())
}
