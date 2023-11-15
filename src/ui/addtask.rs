use ratatui::{
    self,
    prelude::{Constraint, CrosstermBackend, Layout},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::App;

//This function creates the display for the add page part of the application

pub fn render(f: &mut ratatui::Frame<'_, CrosstermBackend<std::io::Stderr>>, app: &mut App) {
    //Creates the layout, by splitting the entire page vertically into two parts, one for title and instruction,
    //and other for the user to enter their text input
    let inner = Layout::default()
        .direction(ratatui::prelude::Direction::Vertical)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(f.size());
    
    //Title and intrctions
    f.render_widget(
        Paragraph::new("Add task page\nStart entering text below\nPress 'Enter' to finally add")
            .alignment(ratatui::prelude::Alignment::Center)
            .block(Block::default().borders(Borders::ALL).fg(Color::LightGreen)),
        inner[0],
    );

    //Area for user to enter text input
    f.render_widget(
        Paragraph::new(app.get_page_text())
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL).fg(Color::LightCyan)),
        inner[1],
    );
}
