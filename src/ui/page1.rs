use chrono::Datelike;
use ratatui::{
    self,
    prelude::{Constraint, CrosstermBackend, Layout},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

pub fn render(f: &mut ratatui::Frame<'_, CrosstermBackend<std::io::Stderr>>, app: &mut App) {
    let outer = Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(15),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .split(f.size());

    for i in 0..7 {
        let inner = Layout::default()
            .direction(ratatui::prelude::Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(5),
                Constraint::Percentage(10),
                Constraint::Percentage(85),
            ])
            .horizontal_margin(1)
            .vertical_margin(1)
            .split(outer[i]);

        let temp = vec![Line::from("--Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1 Line 1"),Line::from("--Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2 Line 2"),Line::from("--Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3 Line 3"),Line::from("--Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4 Line 4"),Line::from("--Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5 Line 5"),Line::from("--Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6 Line 6"),Line::from("--Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7 Line 7"),Line::from("--Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8 Line 8")];
        match app.get_days().get(i) {
            Some(x) => {
                let block = Block::default().borders(Borders::ALL);

                if x.is_today() {
                    f.render_widget(block.fg(Color::LightMagenta), outer[i]);
                } else {
                    f.render_widget(block, outer[i]);
                }

                f.render_widget(
                    Paragraph::new(x.get_date().weekday().to_string())
                        .alignment(ratatui::prelude::Alignment::Center),
                    inner[0],
                );
                f.render_widget(
                    Paragraph::new(x.get_date().to_string())
                        .alignment(ratatui::prelude::Alignment::Center)
                        .block(Block::default().borders(Borders::BOTTOM)),
                    inner[1],
                );
                f.render_widget(
                    Paragraph::new(temp)
                        .wrap(Wrap { trim: false })
                        .scroll(app.get_scroll()),
                    inner[2],
                );
            }
            None => panic!("Day not found"),
        }
    }
}
