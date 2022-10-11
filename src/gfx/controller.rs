use crate::{gfx::{render::RenderDriver, cursor::CursorHandler}, data::{enums::Direction, textrow::TextRow}, utils};
use std::{io::{Error, BufReader, BufRead}, fs::File};

pub struct RenderController {
    render: RenderDriver,
    cursor: CursorHandler,
    rows: u16,
    cols: u16,
}

impl RenderController {
    pub fn new() -> Self {
        let window_size = utils::get_window_size();
        let mut cursor = CursorHandler::new(window_size.rows, window_size.cols);
        let state = cursor.get_state();
        Self {
            render: RenderDriver::new(state),
            cursor,
            rows: window_size.rows,
            cols: window_size.cols,
        }
    }

    pub fn queue_move(&mut self, d: Direction) {
        let data = self.render.get_text();
        match d {
            Direction::Down => self.cursor.handle_cursor(false, self.cursor.cy + 1, data),
            Direction::Up => self.cursor.handle_cursor(false, self.cursor.cy - 1, data),
            Direction::Left => self.cursor.handle_cursor(true, self.cursor.cx - 1, data),
            Direction::Right => self.cursor.handle_cursor(true, self.cursor.cx + 1, data),
            _ => (),
        }
        self.render.update_cursor_state(self.cursor.get_state());
    }

    pub fn queue_scroll(&mut self, d: Direction) {
        let data = self.render.get_text();
        match d {
            Direction::Up => self.cursor.handle_cursor(false, 0, data),
            Direction::Down => self.cursor.handle_cursor(false, self.rows.try_into().unwrap(), data),
            Direction::Left => self.cursor.handle_cursor(true, 0, data),
            Direction::Right => self.cursor.handle_cursor(true, self.cols.try_into().unwrap(), data),
            _ => (),
        }
        self.render.update_cursor_state(self.cursor.get_state());
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