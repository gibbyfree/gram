use std::time::Instant;
use std::fmt;

// Cursor State. Represents the state of the CursorHandler at a moment in time.
// Contains essential CursorHandler fields, for use by the renderer.
#[derive(Copy, Clone)]
pub struct CursorState {
    pub cx: i16,
    pub cy: i16,
    pub row_offset: i16,
    pub col_offset: i16,
}

impl CursorState {
    // Upon construction, a CursorState is zeroed out.
    pub fn new() -> Self {
        Self {
            cx: 0,
            cy: 0,
            row_offset: 0,
            col_offset: 0,
        }
    }

    // Returns the 'line number' of this current cursor state.
    // Stringified version of current cy.
    pub fn line_num(self) -> String {
        (self.cy + 1).to_string()
    }

    // Update the values of this CursorState and return the updated CursorState.
    pub fn update(&mut self, new_cx: i16, new_cy: i16, new_roffset: i16, new_coffset: i16) -> CursorState {
        self.cx = new_cx;
        self.cy = new_cy;
        self.row_offset = new_roffset;
        self.col_offset = new_coffset;
        *self
    }
}


// Status Message. Represents the most recent status message displayed by the renderer.
// Contains the text of the message, and a timestamp representing when the message was fired.
// Choosing to use an Instant here instead of SystemTime, as all we really need is a way to compare to Instant::now() on render.
pub struct StatusMessage {
    pub content: String,
    pub last_sent: Option<Instant>,
}

impl StatusMessage {
    // Set to a keybind help message to begin with.
    // last_sent is set to right now
    pub fn new() -> Self {
        Self {
            content: "".to_string(),
            last_sent: None,
        }
    }

    // Update the content of the latest status message.
    // We also update last_sent here.
    // Technically we could wait to update the timestamp until the renderer actually draws the message.
    // Since we're working with seconds instead of milliseconds here, I think it's fine to set the timestamp here (for now).
    pub fn set_content(&mut self, content: String) {
        self.content = content;
        self.last_sent = Some(Instant::now());
    }

    // Returns whether or not the renderer should print this status message.
    // We print a status message if it's been live for less than 5 seconds.
    pub fn should_print(&mut self) -> bool {
        match self.last_sent {
            None => false,
            Some(t) => t.elapsed().as_secs() <= 5
        }
    }
}

impl fmt::Display for StatusMessage {
    // We display a StatusMessage by printing out its content.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

// Represents the Dirty flag status for the renderer.
// Contains the dirty flag, and # of consecutive force quit inputs.
pub struct DirtyStatus {
    pub dirty: bool,
    pub quit_count: i16,
}

impl DirtyStatus {
    pub fn new() -> Self {
        Self {
            dirty: false,
            quit_count: 0,
        }
    }

    // Various (probably pointless) 'utility methods' for messing around with DirtyStatus fields.
    pub fn redirty(&mut self) {
        self.dirty = true;
        self.quit_count = 0;
    }

    pub fn reset(&mut self) {
        self.quit_count = 0;
    }

    pub fn clean(&mut self) {
        self.dirty = false;
        self.quit_count = 0;
    }
}