use std::io::Error;

use unicode_segmentation::UnicodeSegmentation;

use crate::{
    data::{payload::CursorState, textrow::TextRow},
    gfx::render::RenderDriver,
};

// OperationsHandler. Its purpose in life is to manipulate the fields of a RenderDriver.
pub struct OperationsHandler {
    render: RenderDriver,
    file_name: String,
}

impl OperationsHandler {
    pub fn new(render: RenderDriver) -> Self {
        Self {
            render,
            file_name: "[Untitled]".to_string(),
        }
    }

    pub fn process_write(&mut self, cursor: CursorState, c: char) {
        let data: &Vec<TextRow> = self.render.get_text();
        let idx = cursor.cy as usize;
        let mut row_text: &String = &"".to_string();
        if data.len() > idx.try_into().unwrap() {
            row_text = &data[idx].raw_text;
        }

        // break into graphemes, insert char, put back to string
        let mut g = row_text.graphemes(true).collect::<Vec<&str>>();
        let mut tmp = [0u8; 4];
        g.insert(cursor.cx as usize, c.encode_utf8(&mut tmp));
        let updated: String = g.into_iter().map(String::from).collect();

        self.render.set_text_at_index(idx, updated);
    }

    pub fn write_file(&mut self) {}

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
    // We'll also save this file_name to a field here, for use when saving files.
    pub fn set_file_name(&mut self, name: &str) {
        self.render.set_file_name(name);
        self.file_name = name.to_string();
    }

    // Wrapper around RenderDriver's tick_screen.
    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

    // Wrapper around RenderDriver's exit.
    pub fn exit(&mut self) {
        self.render.exit();
    }

    // Wrapper around RenderDriver's complete_init.
    pub fn complete_init(&mut self) {
        self.render.complete_init();
    }
    // END OF WRAPPER METHODS //
}