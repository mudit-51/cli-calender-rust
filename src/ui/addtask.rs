use ratatui::{
    self,
    prelude::{Constraint, CrosstermBackend, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(f: &mut ratatui::Frame<'_, CrosstermBackend<std::io::Stderr>>, app: &mut App) {
    let inner = Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(f.size());
    f.render_widget(
        Paragraph::new("Add task page")
            .alignment(ratatui::prelude::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).fg(Color::LightGreen)),
        inner[0],
    );
    f.render_widget(
        Paragraph::new(app.get_page_text())
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).fg(Color::LightCyan)),
        inner[1],
    );
}
