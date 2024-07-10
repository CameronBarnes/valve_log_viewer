use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use term::{app::App, event::EventHandler, tui::Tui, update::handle_keys};

mod parser;
mod term;
mod types;

fn main() -> Result<()> {
    println!("Hello, world!");

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

    Ok(())
}
