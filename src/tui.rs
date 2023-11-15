use crossterm::terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use crossterm::execute;

use crossterm::terminal::enable_raw_mode;

use super::Result;

//Crossterm startup
pub fn startup() -> Result<()> {
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen)?;
    Ok(())
}

//Crossterm shutdown
pub fn shutdown() -> Result<()> {
    execute!(std::io::stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
