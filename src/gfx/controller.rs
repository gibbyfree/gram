use crate::{gfx::render::RenderDriver, data::Direction};
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

    pub fn exit(&mut self) {
        self.render.exit();
    }

    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

}