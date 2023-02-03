mod history;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use history::History;

const SIZE: (u32, u32) = (500, 500);

type Field = fn(x: f64, y: f64) -> (f64, f64);

fn main() {
    // Change this to OpenGL::V2_1 if not working
    let opengl = OpenGL::V3_2;

    // Create a Glutin window
    let mut window: Window = WindowSettings::new("visualization", SIZE)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new visualization and run it
    let mut app = App::new(GlGraphics::new(opengl), |_, _| (1.0, 0.0));
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

pub struct App {
    field: Field,   // Field to visualize
    gl: GlGraphics, // OpenGL drawing backend
    history: Vec<History<(f64, f64), 50>>,
}

impl App {
    fn new(gl: GlGraphics, field: Field) -> App {
        App {
            field,
            gl,
            history: Vec::from_iter((0..100).map(|_| {
                let mut l = History::new();
                l.push((
                    rand::random::<f64>() * 2.0 - 1.0,
                    rand::random::<f64>() * 2.0 - 1.0,
                ));
                l
            })),
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        const BACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
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

            self.history
                .last()
                .iter()
                .map(|&(x, y)| (x, y, (self.field)(x, y).0, (self.field)(x, y).1))
                .map(|(x0, y0, x1, y1)| (x0, y0, x0 + x1, y0 + y1))
                .for_each(|(x0, y0, x1, y1)| line(RED, RADIUS, [x0, y0, x1, y1], transform, gl));

            line(RED, RADIUS, [0.0, 0.0, 1.0, 0.0], transform, gl);
            line(GREEN, RADIUS, [0.0, 0.0, 0.0, 1.0], transform, gl);
            line(BLUE, RADIUS, [1.0, 0.0, 1.0, 1.0], transform, gl);
            line(BLUE, RADIUS, [0.0, 1.0, 1.0, 1.0], transform, gl);
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let dt = _args.dt;
        for p in self.history.iter_mut() {
            let &(x, y) = p.last().unwrap();
            let (dx, dy) = (self.field)(x, y);
            p.push((x + dx * dt, y + dy * dt));
        }
    }
}
