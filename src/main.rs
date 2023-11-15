use std::io::stderr;
use app::App;
use ratatui::{prelude::CrosstermBackend, Terminal};

mod tui;
mod ui;
mod update;
pub mod app;

type Err = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Err>;

//This entire application is built using the MVC architecture.

fn main() -> Result<()> {
    //Crossterm start
    tui::startup()?;

    //Creates a new crossterm terminal instance. 
    let terminal = Terminal::new(CrosstermBackend::new(stderr())).unwrap();
    
    //Initializes a struct that behaves as the "model" of the application. All data and methods
    //necessary for the working of the app are associated with this struct. 
    let mut app = App::new(); 
    
    //Passes the struct into the "view" part of the architecture
    ui::ui(terminal, &mut app)?;
    
    //Crossterm end
    tui::shutdown()?;
    Ok(())
}
