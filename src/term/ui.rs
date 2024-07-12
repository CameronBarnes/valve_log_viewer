use std::ops::Deref;

use itertools::Itertools;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Style, Stylize},
    text::ToLine,
    widgets::{Block, Borders, Clear, List, Paragraph, Wrap},
    Frame,
};

use crate::parser::get_levels;

use super::app::{App, InputMode};

pub fn render(app: &mut App, f: &mut Frame) {
    let vertical = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Min(0),
        ],
    )
    .split(f.size());
    let horizontal_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(20), Constraint::Percentage(80)],
    );
    let upper_horizontal = horizontal_layout.split(vertical[1]);
    let lower_horizontal = horizontal_layout.split(vertical[2]);

    // Text filter input area
    f.render_widget(app.filter_widget(), upper_horizontal[1]);
    // Display the cursor when we're using the filter widget
    if app.input_mode == InputMode::Text {
        f.set_cursor(
            upper_horizontal[1].x + 1 + app.input.cursor() as u16,
            upper_horizontal[1].y + 1,
        );
    }

    // Help text
    let help_text = Paragraph::new(
        "Home to move to the top. End to move to bottom. Right and left to select between the log and file menus. CTRL-F to search. SHIFT-F to filter by log level."
        ).wrap(Wrap{ trim: true }).bold();
    f.render_widget(help_text, upper_horizontal[0]);

    // Render list of log files
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
            .title_style(Style::new().bold()),
    )
    .highlight_style(style);
    f.render_stateful_widget(list, lower_horizontal[0], &mut app.list_state);

    // Render log file entries
    let selected = app
        .list_state
        .selected()
        .unwrap_or(0)
        .min(log_files.len() - 1);
    let mut list_state = log_files[selected].list_state_mut().to_owned();
    let list = log_files[selected].get_list(app);
    f.render_stateful_widget(list, lower_horizontal[1], &mut list_state);
    *log_files[selected].list_state_mut() = list_state;

    // Render title
    f.render_widget(
        Paragraph::new("Valve Log Viewer").bold().centered(),
        vertical[0],
    );

    // Log level filter popup
    if app.level_filter_popup.is_some() {
        let size_x = 30;
        let size_y = 8;
        let spare_x = f.size().width.saturating_sub(size_x);
        let spare_y = f.size().height.saturating_sub(size_y);
        let vertical = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(spare_y.saturating_div(2)),
                Constraint::Min(size_y),
                Constraint::Length(spare_y.saturating_div(2)),
            ],
        )
        .split(f.size());
        let horizontal = Layout::new(
            Direction::Horizontal,
            [
                Constraint::Length(spare_x.saturating_div(2)),
                Constraint::Min(size_x),
                Constraint::Length(spare_x.saturating_div(2)),
            ],
        )
        .split(vertical[1]);
        let levels = get_levels();
        let items = levels
            .iter()
            .map(|level| {
                let span = level.to_line().centered().black();
                let span = match (*level).deref() {
                    " Warning " => span.on_light_yellow(),
                    "  Error  " => span.on_light_red(),
                    _ => span.on_gray(),
                };
                if app.filter_list_mut().contains(level) {
                    span.reversed()
                } else {
                    span
                }
            })
            .collect_vec();
        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::all())
                    .title("Filter")
                    .title_style(Style::new().bold())
                    .title_alignment(ratatui::layout::Alignment::Center)
                    .green(),
            )
            .highlight_symbol(">> ");

        f.render_widget(Clear, horizontal[1]);
        f.render_stateful_widget(
            list,
            horizontal[1],
            app.level_filter_popup.as_mut().unwrap(),
        );
    }
}
