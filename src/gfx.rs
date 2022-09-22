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
        print!("{:?}", size_rc);
        Self {
            rows: size_rc.0,
            cols: size_rc.1,
            buf: BufWriter::new(stdout().into_raw_mode().unwrap()),
        }
    }

    fn draw_welcome(&mut self) {
        let welcome = format!("Gram editor -- v{}\r", VERSION);
        let mut padding = (self.cols - welcome.len() as u16) / 2;
        write!(self.buf, "~").expect(WRITE_ERR_MSG);
        while padding > 0 {
            write!(self.buf, " ").expect(WRITE_ERR_MSG);
            padding = padding - 1;
        }
        write!(self.buf, "Gram editor -- v{}\r\n", VERSION).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    fn set_screen(&mut self) {
        for n in 1..self.rows - 1 {
            write!(self.buf, "{}", termion::clear::CurrentLine).expect(WRITE_ERR_MSG);
            if n == self.rows - 2 {
                self.draw_welcome();
            } else {
                writeln!(self.buf, "~\r").expect(WRITE_ERR_MSG);
            }
        }
        self.buf.flush().unwrap();
    }

    pub fn exit(&mut self) {
        write!(self.buf, "{}", termion::clear::All).expect(WRITE_ERR_MSG);
        self.buf.flush().unwrap();
    }

    pub fn tick_screen(&mut self) -> u8 {
        write!(self.buf, "{}", termion::cursor::Hide).expect(WRITE_ERR_MSG);
        write!(self.buf, "{}", termion::cursor::Goto(1, 1)).expect(WRITE_ERR_MSG);
        self.set_screen();
        write!(self.buf, "{}{}", termion::cursor::Show, termion::cursor::Goto(1, 1)).expect(WRITE_ERR_MSG);

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
const VERSION: &'static str = "0.1";