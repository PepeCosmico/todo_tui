use ratatui::{prelude::*, widgets::*};

use crate::tabline::Tabline;

pub fn draw_tabline<B: Backend>(frame: &mut Frame<B>, tabline: &Tabline, chunk: Rect) {
    let titles = tabline
        .get_tabs()
        .iter()
        .map(|tab| Line::from(Span::styled(&tab.name, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(tabline.get_index())
        .highlight_style(Style::default().fg(Color::Yellow));
    frame.render_widget(tabs, chunk);
}
