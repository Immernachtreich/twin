use std::{ fs::{ self, DirEntry }, io::{ self }, path::PathBuf };

use ratatui::{
    crossterm::event::{ KeyCode, KeyEvent, KeyEventKind },
    layout::{ Alignment, Constraint },
    style::{ palette::tailwind::SLATE, Color, Modifier, Style, Stylize },
    text::Line,
    widgets::{ block::Title, Block, HighlightSpacing, List, ListItem, ListState },
    Frame,
};

use crate::app::{ app::ScreenCode, screens::Screen, util::ui, App };

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
        let folder_name = self.pwd
            .file_name()
            .and_then(|s| Some(s.to_string_lossy().to_string()))
            .unwrap_or(String::from(""));

        // let title: Title = Title::from(folder_name);

        let block: Block = Block::bordered()
            .style(Style::default())
            .title_top(Line::from(folder_name).left_aligned())
            .title_top(
                Line::from("TWIN").style(Style::default().fg(Color::Blue)).bold().centered()
            );

        let file_list: Vec<ListItem> = self.dir_entries
            .iter()
            .filter(|entry: &&DirEntry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "txt")
            })
            .map(|entry: &DirEntry| {
                ListItem::new(entry.file_name().to_string_lossy().to_string())
            })
            .collect();

        let list: List = List::new(file_list)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        let centered_rect = ui::center(
            frame.area(),
            Constraint::Percentage(60),
            Constraint::Percentage(60)
        );

        frame.render_stateful_widget(list, centered_rect, &mut self.list_state);
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
