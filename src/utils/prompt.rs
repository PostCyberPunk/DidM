use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::io::{Write, stdout};
//TODO: move this to a new mod so we can have both tui and cli
//Ask user to confirm
pub fn confirm(msg: &str) -> bool {
    println!("{}\npress y to confirm:", msg);

    if stdout().flush().is_err() {
        return false;
    }
    if enable_raw_mode().is_err() {
        return false;
    }
    let result = match event::read() {
        Ok(Event::Key(key_event)) => key_event.code == KeyCode::Char('y'),
        _ => false,
    };

    disable_raw_mode().unwrap();
    result
}
