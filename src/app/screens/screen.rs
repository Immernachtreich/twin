use ratatui::{ crossterm::event::{ KeyEvent }, Frame };

use crate::app::App;

pub trait Screen {
    fn ui(&mut self, app: &App, frame: &mut Frame) -> ();
    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> ();
}
