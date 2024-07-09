use crossterm::event::{KeyCode, KeyEvent};

use super::app::App;

pub fn handle_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
            app.should_quit = true;
        },
        _ => {},
    }
}
