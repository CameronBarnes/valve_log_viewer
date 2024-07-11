use std::{
    ops::Deref,
    sync::{Arc, Mutex},
};

use chrono::NaiveDateTime;
use itertools::Itertools;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use anyhow::{anyhow, Result};

use crate::term::app::Dir;

pub type SharedLog = Arc<Mutex<Log>>;

fn log_level_to_span(level: &Arc<str>) -> Span {
    let span = level.to_span();
    match level.deref() {
        " Warning " => span.black().on_light_yellow(),
        "  Error  " => span.black().on_light_red(),
        _ => span.on_dark_gray(),
    }
}

#[derive(Debug)]
pub struct Entry {
    timestamp: NaiveDateTime,
    level: Arc<str>,
    data: String,
}

impl Entry {
    pub fn timestamp(&self) -> &NaiveDateTime {
        &self.timestamp
    }

    pub fn log_level(&self) -> &str {
        self.level.deref()
    }

    pub fn log_data(&self) -> &str {
        &self.data
    }

    pub(crate) fn new(timestamp: NaiveDateTime, level: Arc<str>, data: &str) -> Self {
        Self {
            timestamp,
            level,
            data: data.to_string(),
        }
    }

    pub fn as_list_item(&self) -> ListItem {
        let mut data = self.data.lines();
        let mut out_lines = Vec::with_capacity(data.clone().count());
        out_lines.push(Line::from(vec![
            self.timestamp().to_span().black().on_dark_gray(),
            log_level_to_span(&self.level),
            Span::from(data.next().unwrap()),
        ]));

        data.for_each(|line| out_lines.push(Line::from(line)));

        ListItem::new(out_lines)
    }
}

#[derive(Debug)]
pub struct Log {
    name: String,
    entries: Vec<Entry>,
    list_state: ListState,
}

impl Log {
    pub fn new<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
            list_state: ListState::default().with_selected(Some(0)),
        }
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

    pub fn append_last(&mut self, input: &str) -> Result<()> {
        let data = &mut self
            .entries
            .last_mut()
            .ok_or(anyhow!("Log must have previous entry to append to"))?
            .data;
        data.push('\n');
        data.push_str(input);
        Ok(())
    }

    pub fn get_list(&self, cursor: Dir) -> List {
        let style = match cursor {
            Dir::Left => Style::new().reversed().dim(),
            Dir::Right => Style::new().reversed(),
        };

        List::new(self.entries().iter().map(Entry::as_list_item).collect_vec())
            .block(
                Block::new()
                    .borders(Borders::all())
                    .title("Log")
                    .title_alignment(ratatui::layout::Alignment::Center),
            )
            .highlight_style(style)
    }

    pub fn as_list_item(&self) -> ListItem {
        ListItem::new(self.name())
    }
}
