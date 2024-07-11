use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::widgets::ListState;

use super::app::App;

pub fn handle_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
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
        KeyCode::Char('F') => {
            if app.level_filter_popup.is_some() {
                app.level_filter_popup = None;
            } else {
                app.level_filter_popup = Some(ListState::default().with_selected(Some(0)));
            }
        }
        _ => {}
    }
}
