use std::sync::{Arc, Mutex};

use chrono::NaiveDateTime;
use ratatui::widgets::ListState;

use anyhow::{anyhow, Result};

pub type SharedLog = Arc<Mutex<Log>>;

pub struct Entry {
    timestamp: NaiveDateTime,
    level: Arc<str>,
    data: String,
}

impl Entry {
    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    pub fn log_level(&self) -> Arc<str> {
        self.level.clone()
    }

    pub fn log_data(&self) -> &str {
        &self.data
    }

    pub(crate) fn new(timestamp: NaiveDateTime, level: Arc<str>, data: &str) -> Self {
        Self { timestamp, level, data: data.to_string() }
    }
}

pub struct Log {
    name: String,
    entries: Vec<Entry>,
    list_state: ListState,
}

impl Log {
    pub fn new<T: ToString>(name: T) -> Self {
        Self { name: name.to_string(), entries: Vec::new(), list_state: ListState::default()}
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub fn list_state_mut(&mut self) -> &mut ListState {
        &mut self.list_state
    }

    pub fn append_last(&mut self, data: &str) -> Result<()> {
        self.entries
            .last_mut()
            .ok_or(anyhow!("Log must have previous entry to append to"))?
            .data
            .push_str(data);
        Ok(())
    }
}
