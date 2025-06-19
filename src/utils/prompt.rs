use crossterm::event::{self, Event, KeyCode};
use std::io::{Write, stdout};
//TODO: move this to a new mod so we can have both tui and cli
//Ask user to confirm
pub fn confirm(msg: &str) -> bool {
    println!("{}\npress y to confirm:", msg);

    stdout().flush().unwrap();

    if let Ok(Event::Key(key_event)) = event::read() {
        return key_event.code == KeyCode::Char('y');
    }
    false
}
