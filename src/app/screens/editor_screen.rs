use std::path::PathBuf;

use ratatui::{
    crossterm::event::KeyEvent,
    layout::Alignment,
    widgets::{ block::Title, Block },
    Frame,
};

use crate::app::{ screens::Screen, App };

pub struct EditorScreen {}

impl EditorScreen {
    pub fn new() -> EditorScreen {
        EditorScreen {}
    }
}

impl Screen for EditorScreen {
    fn ui(&mut self, app: &App, frame: &mut Frame) -> () {
        let file_name = app.selected_file.as_ref();

        let title = Title::from(file_name.unwrap().to_str().unwrap());
        let block = Block::bordered().title(title).title_alignment(Alignment::Center);

        frame.render_widget(block, frame.area());
    }

    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> () {
        ()
    }
}
