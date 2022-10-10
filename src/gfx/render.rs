use std::io::{BufWriter, Stdout, Write, Error, stdout};
use termion::raw::{IntoRawMode, RawTerminal};
use termsize::Size;
use crate::data::textrow::TextRow;

pub struct RenderDriver {
    pub(in crate::gfx) rows: u16,
    pub(in crate::gfx) cols: u16,
    pub(in crate::gfx) cx: i16,
    pub(in crate::gfx) cy: i16,
    buf: BufWriter<RawTerminal<Stdout>>,
    text: Vec<TextRow>,
    row_offset: i16,
    col_offset: i16,
}

impl RenderDriver {
    pub fn new() -> Self {
        let size_rc = get_window_size();
        Self {
            rows: size_rc.rows,
            cols: size_rc.cols,
            cx: 0,
            cy: 0,
            row_offset: 0,
            col_offset: 0,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
            text: vec![TextRow::default()],
        }
    }

    fn draw_footer(&mut self) {
        let welcome = format!("Gram editor -- v{}\r", VERSION);
        let mut padding = (self.cols - welcome.len() as u16) / 2;
        write!(self.buf, "~").expect(WRITE_ERR_MSG);
        while padding > 0 {
            write!(self.buf, " ").expect(WRITE_ERR_MSG);
            padding = padding - 1;
        }
        write!(self.buf, "{}", welcome).expect(WRITE_ERR_MSG);
    }

    fn set_screen(&mut self) {
        for n in 0..self.rows - 1 {
            // clear the current line
            write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
            let row_idx = n.wrapping_add(self.row_offset as u16);
            // render text if necessary, else render edge
            if row_idx < self.text.len() as u16 {
                let render_str = self.text[row_idx as usize].substring(self.col_offset);
                writeln!(self.buf, "{}\r", render_str.truncate(self.cols)).expect(WRITE_ERR_MSG);
            } else {
                writeln!(self.buf, "~\r").expect(WRITE_ERR_MSG);
            }
        }
        self.draw_footer();
    }

    fn handle_y_move(&mut self, val: i16) {
        if val == -1 && self.row_offset > 0 {
            self.row_offset = self.row_offset - 1;
        }
        if val == (self.rows - 1).try_into().unwrap() && (self.row_offset + self.cy) < self.text.len().try_into().unwrap() {
            self.row_offset = self.row_offset + 1;
        }
        if val != -1 && val != (self.rows - 1).try_into().unwrap() {
            self.cy = val;
        }

        // correct cx if we just skipped to a shorter line
        if self.cy + 1 <= self.text.len().try_into().unwrap() && self.cx > self.text[self.cy as usize].length() {
            self.cx = self.text[self.cy as usize].length()
        }
    }

    fn handle_x_move(&mut self, val: i16) {
        if val == -1 && self.col_offset > 0 {
            self.col_offset = self.col_offset - 1;
        }
        if val == (self.cols - 1).try_into().unwrap() && self.text[self.cy as usize].length() >= val + self.col_offset {
            self.col_offset = self.col_offset + 1;
        }
        if val != -1 && val != (self.cols - 1).try_into().unwrap() {
            self.cx = val;
        }
    }

    pub(in crate::gfx) fn set_text(&mut self, text: Vec<TextRow>) {
        self.text = text;
    }

    pub(in crate::gfx) fn set_cursor(&mut self, x: bool, val: i16) {
        if x {
            self.handle_x_move(val);
        } else {
            self.handle_y_move(val);
        }
    }

    pub(in crate::gfx) fn exit(&mut self) {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    pub(in crate::gfx) fn tick_screen(&mut self) -> Result<(), Error> {
        write!(self.buf, "{}{}", termion::cursor::Goto(1, 1), termion::cursor::Hide).expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(self.buf, "{}{}", termion::cursor::Goto((self.cx + 1).try_into().unwrap(), (self.cy + 1).try_into().unwrap()), termion::cursor::Show).expect(WRITE_ERR_MSG); 

        self.buf.flush()
    }
}

fn get_window_size() -> Size {
    termsize::get().unwrap()
}

const WRITE_ERR_MSG: &'static str = "Failed to write to console.";
const VERSION: &'static str = "0.1";