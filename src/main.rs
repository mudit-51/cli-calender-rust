use std::io::stderr;
use app::App;
use ratatui::{prelude::CrosstermBackend, Terminal};

mod tui;
mod ui;
mod update;
pub mod app;

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

fn main() -> Result<()> {
    tui::startup()?;

    let terminal = Terminal::new(CrosstermBackend::new(stderr())).unwrap();
    let mut app = App::new(); 
    
    ui::ui(terminal, &mut app)?;
    tui::shutdown()?;
    Ok(())
}
