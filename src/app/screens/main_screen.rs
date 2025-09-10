use std::{ fs::{ self, DirEntry }, io::{ self }, path::PathBuf };

use chrono::{ DateTime, Local };
use ratatui::{
    crossterm::event::{ KeyCode, KeyEvent, KeyEventKind },
    layout::{ Constraint, Direction, Layout },
    style::{ palette::tailwind::SLATE, Color, Modifier, Style },
    widgets::{ Block, HighlightSpacing, List, ListItem, ListState, Padding },
    Frame,
};

use crate::app::{ app::ScreenCode, screens::Screen, util::ui::{ self, marginized_title }, App };

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
        let layout = Layout::default()
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .direction(Direction::Vertical)
            .split(frame.area());

        let folder_name = self.pwd
            .file_name()
            .and_then(|s| Some(s.to_string_lossy().to_string()))
            .unwrap_or(String::from(""))
            .to_ascii_uppercase();

        let title = marginized_title(
            &folder_name,
            0,
            ui::Direction::Left,
            Style::default().fg(Color::White).bg(Color::Blue)
        );

        let block: Block = Block::bordered()
            .style(Style::default())
            .title_top(title.centered())
            .padding(Padding::top(1));

        let file_list: Vec<ListItem> = self.dir_entries
            .iter()
            .filter(|entry: &&DirEntry| {
                entry
                    .path()
                    .extension()
                    .map_or(false, |ext| ext == "txt")
            })
            .enumerate()
            .map(|(index, entry)| {
                let metadata = entry.metadata().unwrap();

                let file_size = metadata.len();
                let updated_at = metadata
                    .modified()
                    .and_then(|time| {
                        let datetime: DateTime<Local> = time.into();
                        let formatted_date = datetime.format("%Y-%m-%d %H:%M:%S");

                        Ok(formatted_date.to_string())
                    })
                    .unwrap_or("---".to_string());
                let file_stem = entry
                    .path()
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();

                let formatted_name = format!(
                    "{}. {file_stem} | ({} bytes) | {updated_at}",
                    index + 1,
                    file_size
                );

                ListItem::new(formatted_name)
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
