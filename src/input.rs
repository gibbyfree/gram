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
        Key::Ctrl('q') => return Some(InputEvent::Quit),
        Key::Ctrl('s') => return Some(InputEvent::Save),
        Key::Ctrl('h') => return Some(InputEvent::Delete(Direction::Left)),
        Key::Ctrl('f') => return Some(InputEvent::Find),
        Key::Char('/') => return Some(InputEvent::Cancel), // easier testing on my little keyboards 
        Key::Up => return Some(InputEvent::Move(Direction::Up)),
        Key::Left => return Some(InputEvent::Move(Direction::Left)),
        Key::Down => return Some(InputEvent::Move(Direction::Down)),
        Key::Right => return Some(InputEvent::Move(Direction::Right)),
        Key::PageUp => return Some(InputEvent::Page(Direction::Up)),
        Key::PageDown => return Some(InputEvent::Page(Direction::Down)),
        Key::Home => return Some(InputEvent::Page(Direction::Left)),
        Key::End => return Some(InputEvent::Page(Direction::Right)),
        Key::Backspace => return Some(InputEvent::Delete(Direction::Left)),
        Key::Delete => return Some(InputEvent::Delete(Direction::Right)),
        Key::Char(char) => return Some(InputEvent::Write(char)),
        _ => return None,
    };
}
