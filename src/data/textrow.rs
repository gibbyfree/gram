use std::fmt;

#[derive(Default)]
pub struct TextRow {
    raw_text: String,
    text: String,
}

impl TextRow {
    pub fn new(text: String) -> Self {
        let copy = text.clone();
        Self {
            raw_text: text,
            text: copy,
        }
    }

    pub fn truncate(&mut self, l: u16) -> &mut TextRow {
        self.text.truncate(l.into());
        self
    }

    pub fn substring(&mut self, start: i16) -> &mut TextRow {
        let len = self.raw_text.len();
        self.text = self.raw_text.chars().skip(start.try_into().unwrap()).take(len).collect();
        self
    }

    pub fn has_more(&mut self, start: i16, cap: u16) -> bool {
        let maybe_later = self.substring(start);
        return maybe_later.text.len() > cap.into();
    }
}

impl fmt::Display for TextRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
