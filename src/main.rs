use std::{ error::Error, io::{ stdout, Stdout } };

use ratatui::{
    crossterm::{
        execute,
        terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    },
    prelude::CrosstermBackend,
    Terminal,
};

use crate::app::App;

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();

    let mut app = App::new();
    app.run_app(&mut terminal)?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
