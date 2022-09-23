use crate::{gfx::render::RenderDriver, data::{Direction, TextRow}};
use std::io::Error;

pub struct RenderController {
    render: RenderDriver,
}

impl RenderController {
    pub fn new() -> Self {
        Self {
            render: RenderDriver::new()
        }
    }

    pub fn queue_move(&mut self, d: Direction) {
        match d {
            Direction::Down if self.render.cy != self.render.rows - 1 => self.render.set_cursor(false, self.render.cy + 1),
            Direction::Up if self.render.cy != 0 => self.render.set_cursor(false, self.render.cy - 1),
            Direction::Left if self.render.cx != 0 => self.render.set_cursor(true, self.render.cx - 1),
            Direction::Right if self.render.cx != self.render.cols - 1 => self.render.set_cursor(true, self.render.cx + 1),
            _ => (),
        }
    }

    pub fn queue_scroll(&mut self, d: Direction) {
        match d {
            Direction::Up => self.render.set_cursor(false, 0),
            Direction::Down => self.render.set_cursor(false, self.render.rows),
            Direction::Left => self.render.set_cursor(true, 0),
            Direction::Right => self.render.set_cursor(true, self.render.cols),
            _ => (),
        }
    }

    pub fn queue_text_upload(&mut self) {
        let test_str = String::from("Hello, world!");
        let test_row = TextRow::new(test_str);
        self.render.set_text(vec![test_row])
    }

    pub fn exit(&mut self) {
        self.render.exit();
    }

    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

}