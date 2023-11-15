use crate::update;

use super::{app::App, Result};

use ratatui::{prelude::CrosstermBackend, Terminal};
mod addtask;
mod page1;

//Depending on the state of page field in app struct, this function diplays the corresponding page.
//It displays either the main page (page==0), or the add task(page==1) page.
//In case of any error, main page (page==0) is displyed. 
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
        
        //If get_status==true, then the loop breaks, and therefore the app shuts down.  
        if app.get_status() {
            break;
        }
    }
    Ok(())
}
