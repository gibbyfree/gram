mod gfx;
mod input;
mod data;

use std::env::args;
use std::io::Error;
use crate::data::InputEvent;
use crate::gfx::controller::RenderController;

fn main() {
    let mut editor = Gram::new();
    editor.tick();
}

pub struct Gram {
    ctrl: RenderController,
}

impl Gram {
    pub fn new() -> Self {
        Self {
            ctrl: RenderController::new(),
        }
    }

    pub fn tick(&mut self) {
        let mut err: Result<(), Error>;
        let mut evt: Option<InputEvent>;

        let file_name = args().nth(1);
        match file_name {
            None => (),
            Some(str) => self.ctrl.read_file(&str),
        };

        loop {
            err = self.ctrl.tick_screen();
            evt = input::proc_key();

            match err {
                Err(_) => break,
                Ok(_) => (),
            }

            match evt {
                Some(InputEvent::Quit) => break,
                Some(InputEvent::Move(d)) => self.ctrl.queue_move(d),
                Some(InputEvent::Page(d)) => self.ctrl.queue_scroll(d),
                None => (),
            }
        }

        self.ctrl.exit();
    }
}
