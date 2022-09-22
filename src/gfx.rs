use std::io::{BufWriter, Stdout, Write, stdout};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct GfxDriver {
    rows: u16,
    cols: u16,
    buf: BufWriter<RawTerminal<Stdout>>,
}

impl GfxDriver {
    pub fn new() -> Self {
        let size_rc = get_window_size();
        Self {
            rows: size_rc.0,
            cols: size_rc.1,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
        }
    }

    fn draw_lhs_edge(&mut self) {
        for n in 1..self.rows {
            write!(self.buf, "~").expect(WRITE_ERR_MSG);

            if n < self.rows - 1 {
                writeln!(self.buf, "\r").expect(WRITE_ERR_MSG);
            }
        }
        self.buf.flush().unwrap();
    }

    pub fn exit(&mut self) {
        write!(self.buf, "{}", termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    pub fn tick_screen(&mut self) -> u8 {
        write!(self.buf, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).expect(WRITE_ERR_MSG);
        self.draw_lhs_edge();
        write!(self.buf, "{}", termion::cursor::Goto(1, 1)).expect(WRITE_ERR_MSG);

        let res = self.buf.flush();
        match res {
            Ok(_) => return 1,
            Err(_) => return 0,
        };
    }
}

fn get_window_size() -> (u16, u16) {
    termion::terminal_size().unwrap()
}

const WRITE_ERR_MSG: &'static str = "Failed to write to console.";
