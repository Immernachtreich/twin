use std::{ fs, io::Result, path::Path };

use ratatui::{
    crossterm::event::{ KeyCode, KeyEvent, KeyModifiers },
    layout::Alignment,
    widgets::{ block::Title, Block },
    Frame,
};
use tui_textarea::TextArea;

use crate::app::{ app::ScreenCode, screens::Screen, App };

pub struct EditorScreen {
    text_area: TextArea<'static>,
}

impl EditorScreen {
    pub fn new() -> EditorScreen {
        let text_area = TextArea::default();

        EditorScreen {
            text_area,
        }
    }

    pub fn from_file(path: &Path) -> Result<EditorScreen> {
        let file = fs::read_to_string(path)?;
        let file_contents: Vec<String> = file
            .lines()
            .map(|line| line.to_string())
            .collect();
        let text_area = TextArea::from(file_contents);

        Ok(EditorScreen { text_area })
    }
}

impl Screen for EditorScreen {
    fn ui(&mut self, app: &App, frame: &mut Frame) -> () {
        let file_name = app.selected_file.as_ref();

        let title = Title::from(file_name.unwrap().to_str().unwrap().to_string());
        let block = Block::bordered().title(title).title_alignment(Alignment::Center);

        self.text_area.set_block(block);

        frame.render_widget(&self.text_area, frame.area());
    }

    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> () {
        if
            (key_event.code == KeyCode::Char('s') || key_event.code == KeyCode::Char('S')) &&
            key_event.modifiers == KeyModifiers::CONTROL
        {
            fs::write(
                app.selected_file.as_ref().unwrap(),
                self.text_area.lines().join("\n")
            ).expect("Failed to write File");

            app.switch_screen(ScreenCode::Main);
            return;
        }

        self.text_area.input(key_event);
    }
}
