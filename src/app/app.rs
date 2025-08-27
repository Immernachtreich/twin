use std::{ collections::HashMap, io::Result };

use ratatui::{
    crossterm::{ event::{ self, Event, KeyCode, KeyEventKind } },
    prelude::Backend,
    Frame,
    Terminal,
};

use crate::app::screens::{ MainScreen, Screen };

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub enum ScreenCode {
    Main,
}

pub struct App {
    pub screens: HashMap<ScreenCode, Box<dyn Screen>>,
    pub current_screen: ScreenCode,
    pub should_exit: bool,
}

impl App {
    pub fn new() -> App {
        let mut screens: HashMap<ScreenCode, Box<dyn Screen>> = HashMap::with_capacity(1);

        screens.insert(ScreenCode::Main, Box::new(MainScreen::default()));

        App {
            current_screen: ScreenCode::Main,
            screens,
            should_exit: false,
        }
    }

    pub fn run_app<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        loop {
            terminal.draw(|frame: &mut Frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                // Skip events that are not press events
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                if key.code == KeyCode::Esc {
                    break Ok(());
                }

                // Move the screen out, leaving `None`
                if let Some(mut current_screen) = self.screens.remove(&self.current_screen) {
                    let old_key = self.current_screen.clone();

                    // Now we can hand &mut self to the screen safely
                    current_screen.handle_key_event(self, key.code);

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
