use crate::gfx::GfxDriver;
use crate::input;

#[derive(Debug)]
pub struct Gram {
    gfx: GfxDriver,
}

impl Gram {
    pub fn new() -> Self {
        Self {
            gfx: GfxDriver::new(),
        }
    }

    pub fn tick(&self) {
        let (mut exit, mut err) = (1, 1);

        while exit != 0 && err != 0 {
            err = self.gfx.tick_screen();
            exit = input::proc_key();
        }

        print!("{}", termion::clear::All);
    }
}
