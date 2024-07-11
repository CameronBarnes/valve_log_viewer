use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    widgets::{Block, Borders, List},
    Frame,
};

use super::app::App;

pub fn render(app: &mut App, f: &mut Frame) {
    let vertical = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(f.size());
    let horizontal = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(20), Constraint::Percentage(80)],
    )
    .split(vertical[2]);

    let style = match app.cursor() {
        super::app::Dir::Left => Style::new().reversed(),
        super::app::Dir::Right => Style::new().reversed().dim(),
    };
    let arcs = app.logs();
    let mut log_files = arcs.iter().map(|file| file.lock().unwrap()).collect_vec();
    let list = List::new(
        log_files
            .iter()
            .map(|file| file.as_list_item())
            .collect_vec(),
    )
    .block(
        Block::new()
            .borders(Borders::all())
            .title("Files")
            .title_alignment(ratatui::layout::Alignment::Center),
    )
    .highlight_style(style);
    f.render_stateful_widget(list, horizontal[0], &mut app.list_state);

    let selected = app.list_state.selected().unwrap_or(0).min(log_files.len());
    let mut list_state = log_files[selected].list_state_mut().to_owned();
    let list = log_files[selected].get_list(app.cursor());
    f.render_stateful_widget(list, horizontal[1], &mut list_state);
    *log_files[selected].list_state_mut() = list_state;
}
