extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate palette;
extern crate piston;

mod field;
mod history;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use palette::{Gradient, LinSrgba, Pixel};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use field::{Field, FIELDS};
use history::History;

const SIZE: (u32, u32) = (500, 500);
const N: usize = 50;

fn main() {
    // Change this to OpenGL::V2_1 if not working
    let opengl = OpenGL::V3_2;

    // Create a Glutin window
    let mut window: Window = WindowSettings::new("visualization", SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut field_idx: usize = 0;
    // Create a new visualization and run it
    let mut app = App::new(GlGraphics::new(opengl), FIELDS[field_idx]);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }

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
    }
}

struct App {
    field: Field, // Field to visualize
    time: f64,
    gl: GlGraphics, // OpenGL drawing backend
    history: History<N>,
    gradient: Vec<[f32; 4]>,
}

impl App {
    fn new(gl: GlGraphics, field: Field) -> Self {
        let mut history = History::new();
        history.spawn();

        let red = LinSrgba::new(1.0, 0.0, 0.0, 1.0);
        let white = LinSrgba::new(1.0, 1.0, 1.0, 1.0);

        let gradient = Gradient::new([red, white]);
        let gradient: Vec<[f32; 4]> = gradient
            .take(N)
            .map(|srgba| srgba.into_format().into_raw())
            .collect();

        Self {
            field,
            time: 0.0,
            gl,
            history,
            gradient,
        }
    }

    fn renew(self, field: Field) -> Self {
        Self {
            field,
            time: 0.0,
            gl: self.gl,
            history: History::new(),
            gradient: self.gradient,
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const BACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RADIUS: f64 = 0.005;

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            // Clear the screen
            clear(BACK, gl);

            let transform = c
                .transform
                .scale(SIZE.0 as f64, SIZE.1 as f64)
                .scale(0.5, -0.5)
                .trans(1.0, -1.0);

            for (i, gen) in self.history.gen_iter().enumerate() {
                for l in gen {
                    line(self.gradient[i], RADIUS, l, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        let dt = args.dt;

        self.history.spawn();
        self.history.expires(200);

        for p in self.history.data_iter_mut() {
            let &(x, y) = p.last().unwrap();
            let (dx, dy) = (self.field)(x, y, self.time);
            p.push((x + dx * dt, y + dy * dt));
        }
        self.time += dt;
    }
}
