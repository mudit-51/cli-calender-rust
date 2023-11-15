use chrono::Datelike;
use ratatui::{
    self,
    prelude::*,
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

//This fucntion creates the display for the main page of the application

pub fn render(f: &mut ratatui::Frame<'_, CrosstermBackend<std::io::Stderr>>, app: &mut App) {
    
    //Splits the entire display available horizontally into 7 roughly equal parts
    let outer = Layout::default()
        .direction(Direction::Horizontal)
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

    //Splits each of the blocks created above vertically, creating space to display the day and data, followed by tasks    
    for i in 0..7 {
        let inner = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(12),
                Constraint::Percentage(88),
            ])
            .horizontal_margin(1)
            .vertical_margin(1)
            .split(outer[i]);

        if let Some(x) = app.get_days().get(i) {
            //Creates a block to store all the content that follows inside             
            let block = Block::default().borders(Borders::ALL);

            //If the day's date matches the date of the present day, then the block is colored green.
            if x.is_today() {
                f.render_widget(block.fg(Color::Green), outer[i]);
            } else {
                f.render_widget(block, outer[i]);
            }

            //Displays the day name and date in two separate lines. 
            f.render_widget(
                Paragraph::new(format!(
                    "{}\n{}",
                    x.get_date().weekday().to_string(),
                    x.get_date().to_string()
                ))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::BOTTOM)),
                inner[0],
            );
            //If the day struct contains tasks inside, then each task is processed, prefixed with (• charecter) and displayed
            //Else a simple line saying no tasks scheduled is displayed. 
            if let Some(task) = x.get_tasks() {
                let task_text = task.as_array().unwrap();
                let mut z: Vec<Line> = Vec::new();
                for i in task_text {
                    let mut temp_str = i.to_string();
                    temp_str = temp_str.trim_matches('"').to_string();
                    temp_str.insert(0, '•');
                    z.push(Line::from(temp_str));
                }
                f.render_widget(
                    Paragraph::new(z)
                        .wrap(Wrap { trim: false })
                        .scroll(app.get_scroll()),
                    inner[1],
                );
            } else {
                f.render_widget(
                    Paragraph::new("No tasks scheduled")
                        .wrap(Wrap { trim: false })
                        .scroll(app.get_scroll()),
                    inner[1],
                );
            }
        } else {
            panic!("Day not found")
        }
    }
}
