use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::app::App;

pub fn handle_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q' | 'Q') => {
            app.should_quit = true;
        },
        KeyCode::Char('c' | 'C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.should_quit = true;
            }
        }
        _ => {},
    }
}
