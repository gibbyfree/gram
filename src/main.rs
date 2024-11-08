use std::env::args;
use std::io::Error;

mod backend;
mod data;
mod gfx;
mod input;
mod utils;

use crate::data::enums::InputEvent;
use crate::gfx::controller::RenderController;

// Driver function.
fn main() {
    let mut editor = Gram::new();
    editor.tick();
}

// Represents an initialized editor. Only contains a controller.
pub struct Gram {
    ctrl: RenderController,
}

impl Default for Gram {
    fn default() -> Self {
        Self::new()
    }
}

impl Gram {
    pub fn new() -> Self {
        Self {
            ctrl: RenderController::new(),
        }
    }

    // Main function.
    // Read file contents if a path is provided.
    // Until the program exits, enter a loop of ticking the screen and processing key inputs. Any key events will be passed to the controller.
    pub fn tick(&mut self) {
        let mut err: Result<(), Error>;
        let mut evt: Option<InputEvent>;

        let file_name = args().nth(1);
        match file_name {
            None => self.ctrl.finish_early(),
            Some(str) => self.ctrl.read_file(&str),
        };

        loop {
            err = self.ctrl.tick_screen();
            evt = input::proc_key();

            if err.is_err() {
                break;
            }

            match evt {
                Some(InputEvent::Quit) => {
                    let shutdown = self.ctrl.exit();
                    if shutdown {
                        break;
                    }
                }
                Some(InputEvent::Save) => self.ctrl.write_file(),
                Some(InputEvent::Move(d)) => self.ctrl.queue_move(d),
                Some(InputEvent::Page(d)) => self.ctrl.queue_scroll(d),
                Some(InputEvent::Write(c)) => self.ctrl.queue_write(c),
                Some(InputEvent::Delete(d)) => self.ctrl.queue_delete(d),
                Some(InputEvent::Cancel) => self.ctrl.exit_prompt(),
                Some(InputEvent::Find) => self.ctrl.start_prompt(InputEvent::Find),
                None => (),
            }
        }

        self.ctrl.exit();
    }
}
