use std::{ error::Error, io::{ stdout, Stdout } };

use ratatui::{
    crossterm::{
        execute,
        terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    },
    prelude::CrosstermBackend,
    Terminal,
};

use crate::app::{ cli::{ config::{ Config } }, App };

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let config: Config = Config::load().expect("Failed to load config");

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();

    let mut app = App::new(config);
    app.run_app(&mut terminal)?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
