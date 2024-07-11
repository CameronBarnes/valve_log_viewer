use std::sync::Arc;

use ratatui::widgets::ListState;

use crate::{parser::get_levels, types::{Entry, SharedLog}};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    logs: Vec<SharedLog>,
    pub list_state: ListState,
    cursor: Dir,
    pub level_filter_popup: Option<ListState>,
    filter_list: Vec<Arc<str>>,
}

impl App {
    pub fn new(logs: Vec<SharedLog>) -> Self {
        Self {
            should_quit: false,
            logs,
            list_state: ListState::default().with_selected(Some(0)),
            cursor: Dir::Left,
            level_filter_popup: None,
            filter_list: Vec::default(),
        }
    }

    pub fn tick(&self) {}

    pub fn logs(&self) -> Vec<SharedLog> {
        self.logs.clone()
    }

    pub fn right(&mut self) {
        self.cursor = Dir::Right;
    }

    pub fn left(&mut self) {
        self.cursor = Dir::Left;
    }

    pub fn up(&mut self) {
        if let Some(list_state) = &mut self.level_filter_popup {
            list_state.select_previous();
        } else {
            match self.cursor {
                Dir::Left => {
                    self.list_state.select_previous();
                }
                Dir::Right => {
                    self.logs[self
                        .list_state
                        .selected()
                        .unwrap_or(0)
                        .min(self.logs.len().saturating_sub(1))]
                    .lock()
                    .unwrap()
                    .list_state_mut()
                    .select_previous();
                }
            }
        }
    }

    pub fn down(&mut self) {
        if let Some(list_state) = &mut self.level_filter_popup {
            list_state.select_next();
        } else {
            match self.cursor {
                Dir::Left => {
                    self.list_state.select_next();
                }
                Dir::Right => {
                    self.logs[self
                        .list_state
                        .selected()
                        .unwrap_or(0)
                        .min(self.logs.len().saturating_sub(1))]
                    .lock()
                    .unwrap()
                    .list_state_mut()
                    .select_next();
                }
            }
        }
    }

    pub fn enter(&mut self) {
        if self.level_filter_popup.is_some() {
            let selected = self.level_filter_popup.as_ref().unwrap().selected().unwrap();
            let level = get_levels()[selected].clone();
            if self.filter_list.contains(&level) {
                self.filter_list.retain(|entry| entry.ne(&level));
            } else {
                self.filter_list.push(level);
            }
        }
    }

    pub fn home(&mut self) {
        match self.cursor {
            Dir::Left => {
                self.list_state.select_first();
            }
            Dir::Right => {
                self.logs[self
                    .list_state
                    .selected()
                    .unwrap_or(0)
                    .min(self.logs.len().saturating_sub(1))]
                .lock()
                .unwrap()
                .list_state_mut()
                .select_first();
            }
        }
    }

    pub fn end(&mut self) {
        match self.cursor {
            Dir::Left => {
                self.list_state.select_last();
            }
            Dir::Right => {
                self.logs[self
                    .list_state
                    .selected()
                    .unwrap_or(0)
                    .min(self.logs.len().saturating_sub(1))]
                .lock()
                .unwrap()
                .list_state_mut()
                .select_last();
            }
        }
    }

    pub(crate) fn cursor(&self) -> Dir {
        self.cursor
    }

    pub fn filter_list_mut(&mut self) -> &mut Vec<Arc<str>> {
        &mut self.filter_list
    }

    pub fn filter(&self, entry: &Entry) -> bool {
        !self.filter_list.contains(entry.log_level())
    }
}
