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
    // let field: Fiedl = |_x, _y| (1.0, 0.0);
    let field: Field = |x, y| (y, x);
    // let field: Field = |x, y| (y/(x*x+y*y).sqrt(), -x/(x*x+y*y).sqrt());
    let mut app = App::new(GlGraphics::new(opengl), field);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        println!("render");
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        println!("update");
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        println!("ok");
    }
}

pub struct App {
    field: Field,   // Field to visualize
    gl: GlGraphics, // OpenGL drawing backend
    history: History<50>,
}

impl App {
    fn new(gl: GlGraphics, field: Field) -> App {
        let mut history = History::new();
        for _ in 0..100 {
            history.spawn();
        }

        App { field, gl, history }
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

            // TODO Draw from the oldest to the newest
            // let iterators: Vec<_> = self.history
            //     .iter()
            //     .map(|h| h.iter().rev().windows(2).map(|w| (w[1], w[0])).fuse()).collect();
            //
            // loop {
            //     let ok: bool = false;
            //     for i in iterators {
            //         if let Some((x0, y0), (x1, y1)) = i.next() {
            //             ok = true;
            //             line(RED, RADIUS, [x0, y0, x1, y1], transform, gl);
            //         }
            //     }
            //     if !ok {
            //         break;
            //     }
            // }

            // TODO it's rendering non-valid points
            for (i, gen) in self.history.gen_iter().enumerate() {
                println!("{}", i);
                for l in gen {
                    line(RED, RADIUS, l, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, _args: &UpdateArgs) {
        let dt = _args.dt;
        for p in self.history.data_iter_mut() {
            let (x, y) = p.last().unwrap();
            let (dx, dy) = (self.field)(x, y);
            p.push((x + dx * dt, y + dy * dt));
        }
    }
}
