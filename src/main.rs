extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate palette;
extern crate piston;

mod app;
mod field;
mod history;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use crate::app::App;
use crate::field::FIELDS;

const SIZE: (u32, u32) = (500, 500);
const N: usize = 50;

fn main() {
    let opengl = OpenGL::V3_2;

    // Create a new visualization
    let mut window: Window = WindowSettings::new("visualization", SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut field_idx: usize = 0;
    let mut app: App<N> = App::new(GlGraphics::new(opengl), FIELDS[field_idx]);
    let mut events = Events::new(EventSettings::new());

    // Run visualization
    while let Some(e) = events.next(&mut window) {
        // Change field being visualized
        use piston::input::ButtonEvent;
        if let Some(args) = e.button_args() {
            use piston::input::Button::Keyboard;
            use piston::keyboard::Key;
            use piston::ButtonState;

            if args.state == ButtonState::Release {
                if let Keyboard(key) = args.button {
                    match key {
                        Key::Right => {
                            field_idx = (field_idx + 1) % FIELDS.len();
                            app = app.renew(FIELDS[field_idx]);
                        }
                        Key::Left => {
                            field_idx = (field_idx + FIELDS.len() - 1) % FIELDS.len();
                            app = app.renew(FIELDS[field_idx]);
                        }
                        _ => (),
                    }
                }
            }
        }

        // Hangle event
        app.handle_envent(&e);
    }
}
