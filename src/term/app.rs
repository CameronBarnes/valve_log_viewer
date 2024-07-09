use crate::types::Log;

enum Dir {
    Left,
    Right,
}

pub struct App {
    pub should_quit: bool,
    logs: Vec<Log>,
    cursor: Dir,
}

impl App {
    pub fn new(logs: Vec<Log>) -> Self {
        Self { should_quit: false, logs, cursor: Dir::Left }
    }
}
