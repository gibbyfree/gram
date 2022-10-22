use std::io::Error;

use unicode_segmentation::UnicodeSegmentation;

use crate::{gfx::render::RenderDriver, data::{textrow::TextRow, payload::CursorState}};

// OperationsHandler. Its purpose in life is to manipulate the fields of a RenderDriver.
pub struct OperationsHandler {
    render: RenderDriver,
}

impl OperationsHandler {
    pub fn new(render: RenderDriver) -> Self {
        Self {
            render,
        }
    }

    pub fn process_write(&mut self, cursor: CursorState, c: char) {
        let data: &Vec<TextRow> = self.render.get_text();
        let idx = cursor.cy as usize;
        let row_text = &data[idx].raw_text;

        // break into graphemes, insert char, put back to string
        let mut g = row_text.graphemes(true).collect::<Vec<&str>>();
        let mut tmp = [0u8; 4];
        g.insert(cursor.cx as usize, c.encode_utf8(&mut tmp));
        let updated: String = g.into_iter().map(String::from).collect();

        self.render.set_text_at_index(idx, updated);
    }

    // WRAPPER METHODS //
    // Wrapper around RenderDriver's get_text.
    pub fn get_text(&mut self) -> &Vec<TextRow> {
        self.render.get_text()
    }

    // Wrapper around RenderDriver's set_text.
    pub fn set_text(&mut self, text: Vec<TextRow>) {
        self.render.set_text(text);
    }

    // Wrapper around RenderDriver's update_cursor_state.
    pub fn update_cursor_state(&mut self, state: CursorState) {
        self.render.update_cursor_state(state);
    }

    // Wrapper around RenderDriver's set_file_name.
    pub fn set_file_name(&mut self, name: &str) {
        self.render.set_file_name(name);
    }

    // Wrapper around RenderDriver's tick_screen.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

    // Wrapper around RenderDriver's exit.
    pub fn exit(&mut self) {
        self.render.exit();
    }
    // END OF WRAPPER METHODS //
}