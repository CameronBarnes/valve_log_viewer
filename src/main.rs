use std::{
    path::PathBuf,
    thread::spawn,
};

use anyhow::Result;
use clap::Parser;
use itertools::Itertools;
use logwatcher::LogWatcher;
use parser::{parse_file_path, parse_line};
use ratatui::{backend::CrosstermBackend, Terminal};
use term::{app::App, event::EventHandler, tui::Tui, update::handle_keys};
use types::SharedLog;

mod parser;
mod term;
mod types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "txt")]
    extension: String,
    #[arg(value_delimiter = ' ', num_args = 1..)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let (logs, paths): (Vec<SharedLog>, Vec<PathBuf>) = args
        .files
        .iter()
        .flat_map(|path| parse_file_path(path, "txt"))
        .flatten()
        .unzip();

    let handles = paths
        .into_iter()
        .zip(logs.iter().cloned())
        .map(|(path, log)| {
            spawn(|| {
                let mut watcher = LogWatcher::register(path).unwrap();
                watcher.watch(&mut move |line: String| {
                    parse_line(log.clone(), &line).unwrap();
                    logwatcher::LogWatcherAction::None
                });
            })
        })
        .collect_vec();

    // Init term ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(30);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app = App::new(Vec::new());

    // Do main program loop
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            term::event::Event::Key(key_event) => handle_keys(&mut app, key_event),
            term::event::Event::Tick => app.tick(),
            _ => {}
        }
    }

    // Close down the term ui stuff cleanly
    tui.exit()?;

    handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });

    Ok(())
}
