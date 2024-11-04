use crate::data::enums::{Direction, InputEvent};
use std::io::stdin;
use termion::event::Key;
use termion::input::TermRead;

// Functions for reading and processing key inputs.

// Read a key and return its Key value. If there is no keypress on this tick, return a Null keypress.
fn read_key() -> Key {
    if let Some(b) = stdin().keys().next() {
        return b.unwrap();
    }
    Key::Null
}

// Process a read key into various InputEvents.
// If the key does not match a known InputEvent, nothing happens.
pub fn proc_key() -> Option<InputEvent> {
    let k = read_key();

    match k {
        Key::Ctrl('q') => Some(InputEvent::Quit),
        Key::Ctrl('s') => Some(InputEvent::Save),
        Key::Ctrl('h') => Some(InputEvent::Delete(Direction::Left)),
        Key::Ctrl('f') => Some(InputEvent::Find),
        Key::Esc => Some(InputEvent::Cancel),
        Key::Up => Some(InputEvent::Move(Direction::Up)),
        Key::Left => Some(InputEvent::Move(Direction::Left)),
        Key::Down => Some(InputEvent::Move(Direction::Down)),
        Key::Right => Some(InputEvent::Move(Direction::Right)),
        Key::PageUp => Some(InputEvent::Page(Direction::Up)),
        Key::PageDown => Some(InputEvent::Page(Direction::Down)),
        Key::Home => Some(InputEvent::Page(Direction::Left)),
        Key::End => Some(InputEvent::Page(Direction::Right)),
        Key::Backspace => Some(InputEvent::Delete(Direction::Left)),
        Key::Delete => Some(InputEvent::Delete(Direction::Right)),
        Key::Char(char) => Some(InputEvent::Write(char)),
        _ => None,
    }
}
