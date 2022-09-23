use crate::data::InputEvent;
use crate::gfx::GfxDriver;
use crate::input;

pub struct Gram {
    gfx: GfxDriver,
}

impl Gram {
    pub fn new() -> Self {
        Self {
            gfx: GfxDriver::new(),
        }
    }

    pub fn tick(&mut self) {
        let mut err = 1;
        let mut evt: Option<InputEvent>;

        while err != 0 {
            err = self.gfx.tick_screen();
            evt = input::proc_key();

            match evt {
                Some(InputEvent::Quit) => break,
                Some(InputEvent::Move(d)) => self.gfx.queue_move(d),
                Some(_) => (),
                None => (),
            }
        }

        self.gfx.exit();
    }
}
