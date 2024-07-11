use ratatui::widgets::ListState;

use crate::types::SharedLog;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
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
            list_state: ListState::default(),
            cursor: Dir::Left,
        }
    }

    pub fn tick(&self) {}

    pub fn logs(&self) -> Vec<SharedLog> {
        self.logs.clone()
    }

}
