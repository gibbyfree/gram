use std::io::Error;

use crate::{gfx::render::RenderDriver, data::{textrow::TextRow, payload::CursorState}};

pub struct OperationsHandler {
    render: RenderDriver,
}

impl OperationsHandler {
    pub fn new(render: RenderDriver) -> Self {
        Self {
            render,
        }
    }

    // WRAPPER METHODS //
    pub fn get_text(&mut self) -> &Vec<TextRow> {
        self.render.get_text()
    }

    pub fn set_text(&mut self, text: Vec<TextRow>) {
        self.render.set_text(text);
    }

    pub fn update_cursor_state(&mut self, state: CursorState) {
        self.render.update_cursor_state(state);
    }

    pub fn set_file_name(&mut self, name: &str) {
        self.render.set_file_name(name);
    }

    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

    pub fn exit(&mut self) {
        self.render.exit();
    }
    // END OF WRAPPER METHODS //
}