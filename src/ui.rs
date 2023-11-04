use crate::update;

use super::{app::App, Result};

use ratatui::{prelude::CrosstermBackend, Terminal};
mod addtask;
mod page1;

pub fn ui(mut terminal: Terminal<CrosstermBackend<std::io::Stderr>>, app: &mut App) -> Result<()> {
    loop {
        match app.get_page() {
            0 => {
                terminal.draw(|f| page1::render(f, app))?;
                update::udpate_0(app);
            }
            1 => {
                terminal.draw(|f| addtask::render(f, app))?;
                update::update_1(app);
            }
            _ => {
                terminal.draw(|f| page1::render(f, app))?;
            }
        }
        if app.get_status() {
            break;
        }
    }
    Ok(())
}
