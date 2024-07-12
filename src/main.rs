use std::{fs::File, io::Read, path::PathBuf, thread::spawn};

use anyhow::Result;
use clap::Parser;
use logwatcher::LogWatcher;
use parser::{parse_file_path, parse_line};
use ratatui::{backend::CrosstermBackend, Terminal};
use term::{
    app::App,
    event::EventHandler,
    tui::Tui,
    update::{handle_keys, handle_mouse},
};
use types::SharedLog;

mod parser;
mod term;
mod types;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "txt")]
    extension: String,
    #[arg(num_args = 1..)]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.files.is_empty() {
        println!("Path to log files is required");
        return Ok(());
    }

    let (logs, paths): (Vec<SharedLog>, Vec<PathBuf>) = args
        .files
        .iter()
        .flat_map(|path| parse_file_path(path, &args.extension).unwrap())
        .map(|(log, path)| {
            // Read the existing contents of the file into the log
            let log_new = log.clone();
            let mut log_mut = log_new.lock().unwrap();
            let mut output = String::new();
            File::open(&path)
                .unwrap()
                .read_to_string(&mut output)
                .unwrap();
            output
                .lines()
                .for_each(|line| parse_line(&mut log_mut, line).unwrap());
            (log, path)
        })
        .unzip();

    // Creating new threads to handle the log watching
    paths
        .into_iter()
        .zip(logs.iter().cloned())
        .for_each(|(path, log)| {
            spawn(|| {
                let mut watcher = LogWatcher::register(path).unwrap();
                watcher.watch(&mut move |line: String| {
                    parse_line(&mut log.lock().unwrap(), &line).unwrap();
                    logwatcher::LogWatcherAction::None
                });
            });
        });

    // Init term ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(30);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    let mut app = App::new(logs);

    // Do main program loop
    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next().unwrap() {
            term::event::Event::Key(key_event) => handle_keys(&mut app, key_event),
            term::event::Event::Mouse(mouse_event) => handle_mouse(&mut app, mouse_event),
            _ => {}
        }
    }

    // Close down the term ui stuff cleanly
    tui.exit().unwrap();

    // We dont want to join these because they wont ever close if we wait for them
    /*handles.into_iter().for_each(|handle| {
        handle.join().unwrap();
    });*/

    Ok(())
}
