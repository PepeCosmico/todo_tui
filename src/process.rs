use crossterm::event::{self, Event, KeyCode};
use ratatui::prelude::*;
use std::{io::Stdout, time::Duration};

use crate::{app::App, ui::draw};

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    mut app: App,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(loop {
        // Drawing
        terminal.draw(|frame| draw(frame, &app))?;

        // Processing inputs
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Tab => app.tabline.next(),
                    KeyCode::BackTab => app.tabline.previous(),
                    _ => (),
                };
            }
        }
    })
}
