use crate::{gfx::render::RenderDriver, data::{Direction, TextRow}};
use std::{io::{Error, BufReader, BufRead}, fs::File};

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
            Direction::Down => self.render.set_cursor(false, self.render.cy + 1),
            Direction::Up => self.render.set_cursor(false, self.render.cy - 1),
            Direction::Left if self.render.cx != 0 => self.render.set_cursor(true, self.render.cx - 1),
            Direction::Right if self.render.cx != (self.render.cols - 1).try_into().unwrap() => self.render.set_cursor(true, self.render.cx + 1),
            _ => (),
        }
    }

    pub fn queue_scroll(&mut self, d: Direction) {
        match d {
            Direction::Up => self.render.set_cursor(false, 0),
            Direction::Down => self.render.set_cursor(false, self.render.rows.try_into().unwrap()),
            Direction::Left => self.render.set_cursor(true, 0),
            Direction::Right => self.render.set_cursor(true, self.render.cols.try_into().unwrap()),
            _ => (),
        }
    }

    pub fn read_file(&mut self, s: &str) {
        let file = File::open(s).expect("File not found at the given location.");
        let buf = BufReader::new(file);

        let mut vec: Vec<String> = Vec::new();
        for line in buf.lines() {
            vec.push(line.unwrap());
        }
        self.queue_text_upload(&vec);
    }

    pub fn queue_text_upload(&mut self, vec: &Vec<String>) {
        let mut output: Vec<TextRow> = Vec::new();
        for text in vec {
            let str = String::from(text.trim());
            let row = TextRow::new(str);
            output.push(row);
        }
        self.render.set_text(output)
    }

    pub fn exit(&mut self) {
        self.render.exit();
    }

    pub fn tick_screen(&mut self) -> Result<(), Error> {
        self.render.tick_screen()
    }

}