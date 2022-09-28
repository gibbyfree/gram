use std::fmt;

pub enum InputEvent {
    Quit,
    Move(Direction),
    Page(Direction),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// TextRow 
#[derive(Default)]
pub struct TextRow {
    text: String,
}

impl TextRow {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn truncate(&mut self, l: u16) -> &mut TextRow {
        self.text.truncate(l.into());
        self
    }
}

impl fmt::Display for TextRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}