use ratatui::widgets::ListState;

use crate::types::SharedLog;

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
}

impl App {
    pub fn new(logs: Vec<SharedLog>) -> Self {
        Self {
            should_quit: false,
            logs,
            list_state: ListState::default().with_selected(Some(0)),
            cursor: Dir::Left,
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
        match self.cursor {
            Dir::Left => {
                self.list_state.select_previous();
            }
            Dir::Right => {
                self.logs[self.list_state.selected().unwrap_or(0).min(self.logs.len())]
                    .lock()
                    .unwrap()
                    .list_state_mut()
                    .select_previous();
            }
        }
    }

    pub fn down(&mut self) {
        match self.cursor {
            Dir::Left => {
                self.list_state.select_next();
            }
            Dir::Right => {
                self.logs[self.list_state.selected().unwrap_or(0).min(self.logs.len())]
                    .lock()
                    .unwrap()
                    .list_state_mut()
                    .select_next();
            }
        }
    }

    pub fn home(&mut self) {
        match self.cursor {
            Dir::Left => {
                self.list_state.select_first();
            }
            Dir::Right => {
                self.logs[self.list_state.selected().unwrap_or(0).min(self.logs.len())]
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
                self.logs[self.list_state.selected().unwrap_or(0).min(self.logs.len())]
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
}
