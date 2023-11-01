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
                match x.get_tasks() {
                    Some(task) => {
                        let task_text = task.as_array().unwrap();
                        let mut z: Vec<Line> = Vec::new();
                        for i in task_text {
                            z.push(Line::from(i.to_string()));
                        }
                        f.render_widget(
                            Paragraph::new(z)
                                .wrap(Wrap { trim: false })
                                .scroll(app.get_scroll()),
                            inner[2],
                        );
                    }
                    None => {
                        f.render_widget(
                            Paragraph::new("No tasks scheduled")
                                .wrap(Wrap { trim: false })
                                .scroll(app.get_scroll()),
                            inner[2],
                        );
                    }
                }
            }
            None => panic!("Day not found"),
        }
    }
}
