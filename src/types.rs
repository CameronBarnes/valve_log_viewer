use std::sync::Arc;

use chrono::NaiveDateTime;
use ratatui::widgets::ListState;

pub struct Entry {
    timestamp: NaiveDateTime,
    level: Arc<str>,
    data: String,
}

pub struct Log {
    name: String,
    entries: Vec<Entry>,
    list_state: ListState,
}
