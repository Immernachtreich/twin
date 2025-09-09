use ratatui::{
    layout::{ Constraint, Flex, Layout, Rect },
    style::{ Style, Stylize },
    text::{ Line, Span },
};

pub enum Direction {
    Left,
    Right,
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal]).flex(Flex::Center).areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn marginized_title(
    title: &str,
    margin: usize,
    direction: Direction,
    title_style: Style
) -> Line {
    let margin = Span::raw("─".repeat(margin));
    let title = Span::styled(format!(" {title} "), title_style).bold();

    let spans = match direction {
        Direction::Left => vec![margin, title],
        Direction::Right => vec![margin, title],
    };

    Line::from(spans)
}
