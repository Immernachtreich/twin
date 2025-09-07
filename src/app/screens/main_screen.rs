use std::{ fs::{ self, DirEntry }, io::{ self }, path::PathBuf };

use ratatui::{
    crossterm::event::{ KeyCode, KeyEvent, KeyEventKind },
    layout::Alignment,
    style::{ palette::tailwind::SLATE, Modifier, Style, Stylize },
    widgets::{ block::Title, Block, HighlightSpacing, List, ListItem, ListState },
    Frame,
};

use crate::app::{ app::ScreenCode, screens::Screen, App };

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct MainScreen {
    pwd: PathBuf,
    dir_entries: Vec<DirEntry>,
    list_state: ListState,
}

impl MainScreen {
    pub fn new(pwd: &PathBuf) -> MainScreen {
        let dir_entries = MainScreen::get_notes(pwd);
        MainScreen { pwd: pwd.to_path_buf(), dir_entries, list_state: ListState::default() }
    }

    fn get_notes(pwd: &PathBuf) -> Vec<DirEntry> {
        fs::read_dir(pwd).unwrap().collect::<io::Result<Vec<DirEntry>>>().unwrap()
    }
}

impl Screen for MainScreen {
    fn ui(&mut self, _app: &App, frame: &mut Frame) -> () {
        let title: Title = Title::from(self.pwd.to_str().unwrap());

        let block: Block = Block::bordered()
            .style(Style::default())
            .title(title)
            .title_alignment(Alignment::Center)
            .title_style(Style::default().bold());

        let file_list: Vec<ListItem> = self.dir_entries
            .iter()
            .filter(|entry| {
                let is_folder: bool = !entry.file_type().unwrap().is_file();

                if is_folder {
                    return true;
                }

                let path = entry.path();
                let extention = path.extension().and_then(|ext| ext.to_str());

                match extention {
                    None => false,
                    Some(ext) => ext == "txt",
                }
            })
            .map(|entry| {
                let file_name: String = entry
                    .path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_string();

                ListItem::from(file_name)
            })
            .collect();

        let list: List = List::new(file_list)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }

    fn handle_key_event(&mut self, app: &mut App, key_event: KeyEvent) -> () {
        if key_event.kind != KeyEventKind::Press {
            return;
        }

        match key_event.code {
            KeyCode::Up => self.list_state.select_previous(),
            KeyCode::Down => self.list_state.select_next(),
            KeyCode::Enter => {
                let selected_index = self.list_state.selected();

                if selected_index.is_none() {
                    return;
                }

                app.selected_file = self.dir_entries
                    .get(selected_index.unwrap())
                    .and_then(|entry| Some(entry.path()));

                app.switch_screen(ScreenCode::Editor);
            }
            _ => (),
        }
    }
}
