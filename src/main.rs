use app::App;

use crate::{
    process::run,
    utils::terminal::{restore_terminal, setup_terminal},
};

mod app;
mod db;
mod error;
mod models;
mod process;
mod tabline;
mod ui;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let app = App::init()?;
    run(&mut terminal, app)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}
