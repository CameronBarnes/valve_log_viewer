use std::sync::Arc;

use fuzzy_matcher::skim::SkimMatcherV2;
use once_cell::sync::Lazy;
use ratatui::{
    style::Stylize,
    widgets::{Block, Borders, ListState, Paragraph},
};
use regex::Regex;
use tui_input::Input;

use crate::{
    parser::get_levels,
    types::{Entry, SharedLog},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
pub enum FilterMode {
    Exact,
    Fuzzy,
    Regex(Option<Regex>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Text,
}

#[derive(Debug)]
pub struct App {
    pub should_quit: bool,
    logs: Vec<SharedLog>,
    pub list_state: ListState,
    cursor: Dir,
    pub level_filter_popup: Option<ListState>,
    filter_list: Vec<Arc<str>>,
    pub filter_mode: FilterMode,
    pub input_mode: InputMode,
    pub input: Input,
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
            filter_mode: FilterMode::Exact,
            input_mode: InputMode::Normal,
            input: Input::default(),
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
            let selected = self
                .level_filter_popup
                .as_ref()
                .unwrap()
                .selected()
                .unwrap();
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
        static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().smart_case());
        !self.filter_list.contains(entry.log_level())
            && (self.input.value().is_empty()
                || match &self.filter_mode {
                    FilterMode::Exact => entry.log_data().contains(self.input.value()),
                    FilterMode::Fuzzy => MATCHER
                        .fuzzy(entry.log_data(), self.input.value(), true)
                        .is_some(),
                    FilterMode::Regex(re) => {
                        if let Some(re) = re {
                            re.is_match(entry.log_data())
                        } else {
                            true
                        }
                    }
                })
    }

    pub fn filter_widget(&self) -> Paragraph {
        let title = match self.filter_mode {
            FilterMode::Exact => "Filter - Exact",
            FilterMode::Fuzzy => "Filter - Fuzzy",
            FilterMode::Regex(_) => "Filter - Regex",
        };

        let block = Block::default().borders(Borders::all()).title(title);
        let block = match &self.filter_mode {
            FilterMode::Regex(val) => {
                if val.is_some() {
                    block.green()
                } else {
                    block.red()
                }
            }
            _ => block,
        };
        let block = match &self.input_mode {
            InputMode::Normal => block.dim(),
            InputMode::Text => block.bold(),
        };
        Paragraph::new(self.input.value()).block(block)
    }

    pub fn update_regex(&mut self) {
        if matches!(self.filter_mode, FilterMode::Regex(_)) {
            if !self.input.value().is_empty() {
                self.filter_mode = FilterMode::Regex(None);
            } else {
                self.filter_mode = FilterMode::Regex(Regex::new(self.input.value()).ok());
            }
        }
    }
}
