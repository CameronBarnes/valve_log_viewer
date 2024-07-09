use ratatui::widgets::ListState;


pub struct Log {
    name: String,
    entries: Vec<String>,
    list_state: ListState,
}
