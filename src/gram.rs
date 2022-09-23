use std::io::Error;
use crate::data::InputEvent;
use crate::gfx::controller::RenderController;
use crate::input;

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
