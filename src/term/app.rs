use crate::types::SharedLog;

enum Dir {
    Left,
    Right,
}

pub struct App {
    pub should_quit: bool,
    logs: Vec<SharedLog>,
    cursor: Dir,
}

impl App {
    pub fn new(logs: Vec<SharedLog>) -> Self {
        Self {
            should_quit: false,
            logs,
            cursor: Dir::Left,
        }
    }

    pub fn tick(&self) {}
}
