use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent};
use ratatui::{layout::Position, widgets::ListState};
use tui_input::backend::crossterm::EventHandler;

use super::app::{App, FilterMode, InputMode};

pub fn handle_keys(app: &mut App, key_event: KeyEvent) {
    match app.input_mode {
        super::app::InputMode::Normal => match key_event.code {
            KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
                if app.level_filter_popup.is_some() {
                    app.level_filter_popup = None;
                } else {
                    app.should_quit = true;
                }
            }
            KeyCode::Char('c' | 'C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.should_quit = true;
                }
            }
            KeyCode::Up => app.up(),
            KeyCode::Down => app.down(),
            KeyCode::Left => app.left(),
            KeyCode::Right => app.right(),
            KeyCode::Home => app.home(),
            KeyCode::End => app.end(),
            KeyCode::Enter | KeyCode::Char(' ') => app.enter(),
            KeyCode::Char('f') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    app.input_mode = InputMode::Text;
                }
            }
            KeyCode::Char('F') => {
                if app.level_filter_popup.is_some() {
                    app.level_filter_popup = None;
                } else {
                    app.level_filter_popup = Some(ListState::default().with_selected(Some(0)));
                }
            }
            _ => {}
        },
        super::app::InputMode::Text => match key_event.code {
            KeyCode::Esc => {
                app.input_mode = InputMode::Normal;
            }
            KeyCode::Tab => {
                match app.filter_mode {
                    FilterMode::Exact => app.filter_mode = FilterMode::Fuzzy,
                    FilterMode::Fuzzy => app.filter_mode = FilterMode::Regex(None),
                    FilterMode::Regex(_) => app.filter_mode = FilterMode::Exact,
                }
                app.update_regex();
            }
            _ => {
                app.input.handle_event(&Event::Key(key_event));
                app.update_regex();
            }
        },
    }
}

pub fn handle_mouse(app: &mut App, mouse_event: MouseEvent) {
    let position = Position {
        x: mouse_event.column,
        y: mouse_event.row,
    };
    match mouse_event.kind {
        crossterm::event::MouseEventKind::Down(button) => {
            if button == MouseButton::Left {
                if app.left_zone.contains(position) {
                    app.input_mode = InputMode::Normal;
                    app.left();
                } else if app.right_zone.contains(position) {
                    app.input_mode = InputMode::Normal;
                    app.right();
                } else if app.filter_zone.contains(position) {
                    app.input_mode = InputMode::Text;
                }
            }
        }
        crossterm::event::MouseEventKind::ScrollDown => {
            if matches!(app.input_mode, InputMode::Normal) {
                app.down();
            } else {
                match app.filter_mode {
                    FilterMode::Exact => app.filter_mode = FilterMode::Fuzzy,
                    FilterMode::Fuzzy => app.filter_mode = FilterMode::Regex(None),
                    FilterMode::Regex(_) => app.filter_mode = FilterMode::Exact,
                }
                app.update_regex();
            }
        }
        crossterm::event::MouseEventKind::ScrollUp => {
            if matches!(app.input_mode, InputMode::Normal) {
                app.up();
            } else {
                match app.filter_mode {
                    FilterMode::Exact => app.filter_mode = FilterMode::Regex(None),
                    FilterMode::Fuzzy => app.filter_mode = FilterMode::Exact,
                    FilterMode::Regex(_) => app.filter_mode = FilterMode::Fuzzy,
                }
                app.update_regex();
            }
        }
        _ => {}
    }
}
