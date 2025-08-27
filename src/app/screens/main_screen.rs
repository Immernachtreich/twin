use std::rc::Rc;

use ratatui::{
    crossterm::event::KeyCode,
    layout::{ Constraint, Direction, Layout, Rect },
    style::Style,
    widgets::{ Block, Paragraph },
    Frame,
};

use crate::app::{ screens::Screen, App };

#[derive(Default)]
pub struct MainScreen {
    pwd: String,
}

impl Screen for MainScreen {
    fn ui(&mut self, app: &App, frame: &mut Frame) -> () {
        let layout: Rc<[Rect]> = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());

        let block_1: Block = Block::bordered().style(Style::default());
        let block_2: Block = Block::bordered().style(Style::default());

        frame.render_widget(block_1, layout[0]);
        frame.render_widget(block_2, layout[1]);
    }

    fn handle_key_event(&mut self, app: &mut App, key_code: KeyCode) -> () {
        ()
    }
}
