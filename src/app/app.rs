use std::{ collections::HashMap, io::Result, path::PathBuf };

use ratatui::{
    crossterm::{ event::{ self, Event, KeyCode, KeyEventKind } },
    prelude::Backend,
    Frame,
    Terminal,
};

use crate::app::{ cli::config::Config, screens::{ EditorScreen, MainScreen, Screen } };

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum ScreenCode {
    Main,
    Editor,
}

pub struct App {
    pub config: Config,
    pub screens: HashMap<ScreenCode, Box<dyn Screen>>,
    pub current_screen: ScreenCode,
    pub should_exit: bool,
    pub selected_file: Option<PathBuf>,
}

impl App {
    pub fn new(config: Config) -> App {
        let mut screens: HashMap<ScreenCode, Box<dyn Screen>> = HashMap::with_capacity(2);

        screens.insert(ScreenCode::Main, Box::new(MainScreen::new(&config.storage_path)));
        screens.insert(ScreenCode::Editor, Box::new(EditorScreen::new()));

        App {
            config,
            current_screen: ScreenCode::Main,
            screens,
            should_exit: false,
            selected_file: None,
        }
    }

    pub fn switch_screen(&mut self, screen: ScreenCode) -> () {
        self.current_screen = screen;
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame: &mut Frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                    break Ok(());
                }

                // Move the screen out, leaving `None`
                if let Some(mut current_screen) = self.screens.remove(&self.current_screen) {
                    let old_key = self.current_screen.clone();

                    // Now we can hand &mut self to the screen safely
                    current_screen.handle_key_event(self, key);

                    // Put the screen back
                    self.screens.insert(old_key, current_screen);
                }
            }

            if self.should_exit {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let key = self.current_screen.clone();

        if let Some(mut current_screen) = self.screens.remove(&key) {
            current_screen.ui(self, frame);

            self.screens.insert(key, current_screen);
        }
    }
}
