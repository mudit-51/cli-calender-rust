use ratatui::{
    self,
    prelude::{Constraint, CrosstermBackend, Layout},
    style::{Color, Modifier, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render(f: &mut ratatui::Frame<'_, CrosstermBackend<std::io::Stderr>>, app: &mut App) {
    let outer = Layout::default()
        .direction(ratatui::prelude::Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
            Constraint::Percentage(14),
        ])
        .split(f.size());

    for i in 0..7 {
        
        let inner = Layout::default()
            .direction(ratatui::prelude::Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
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
                    Paragraph::new(x.get_date_string().as_str())
                        .alignment(ratatui::prelude::Alignment::Center),
                    inner[0],
                )
            }
            None => panic!("Day not found"),
        }
    }
}

fn _render_day(day: &str, i: i32, n: i32) -> Paragraph<'_> {
    let x = Paragraph::new(
        Line::styled(day, Style::default().add_modifier(Modifier::UNDERLINED))
            .alignment(ratatui::prelude::Alignment::Center),
    )
    .block(Block::default().borders(Borders::ALL));

    if i == n {
        x.fg(Color::LightGreen)
    } else {
        x
    }
}
