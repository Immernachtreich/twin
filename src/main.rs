use std::{ error::Error, io::{ stdout, Stdout }, path::PathBuf };

use clap::Parser;
use ratatui::{
    crossterm::{
        execute,
        terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    },
    prelude::CrosstermBackend,
    Terminal,
};

use crate::app::{ cli::{ config::{ self, Config }, Args }, App };

mod app;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config: Config = config::Config::default();

    config.load_config().expect("Failed to parse config");

    if config.storage_path.is_none() {
        let args: Args = Args::try_parse().expect(
            "No storage path selected. Please run the application with --path to set a path to your notes"
        );

        config.config_path = PathBuf::from(args.storage_path);
        config.save().expect("Failed to save file path into config");
    }

    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();

    let mut app = App::new();
    app.run_app(&mut terminal)?;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    Ok(())
}
