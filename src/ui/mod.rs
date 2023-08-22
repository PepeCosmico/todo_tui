use ratatui::{prelude::*, widgets::*};

use crate::app::App;

use self::tabline::draw_tabline;

mod tabline;

pub fn draw<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let size = frame.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    draw_tabline(frame, &app.tabline, chunks[0]);

    let inner = match app.tabline.get_index() {
        0 => Block::default().title("Inner main").borders(Borders::ALL),
        1 => Block::default().title("Inner tab1").borders(Borders::ALL),
        2 => Block::default().title("Inner tab2").borders(Borders::ALL),
        _ => Block::default().title("TODO").borders(Borders::ALL),
    };
    frame.render_widget(inner, chunks[1]);
}
