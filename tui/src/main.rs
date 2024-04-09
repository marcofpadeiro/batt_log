pub mod app;
pub mod modules;
pub mod terminal;
use std::error::Error;

use app::App;
use terminal::*;

fn main() -> Result<(), Box<dyn Error>> {
    let terminal = init_terminal()?;

    App::new().run(terminal)?;

    restore_terminal()?;

    Ok(())
}
