use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, Terminal};

pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn std::error::Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}
