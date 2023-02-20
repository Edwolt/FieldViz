use opengl_graphics::GlGraphics;
use palette::{Gradient, LinSrgba, Pixel};
use piston::input::{RenderArgs, RenderEvent, UpdateEvent};
use piston::Event;

use crate::field::Field;
use crate::history::History;

pub struct App<const HISTORY_LEN: usize> {
    /// Field to visualize
    field: Field,
    time: f64,

    /// OpenGL drawing backend
    gl: GlGraphics,
    history: History<HISTORY_LEN>,
    pub gradient: Vec<[f32; 4]>,
}

impl<const HISTORY_LEN: usize> App<HISTORY_LEN> {
    pub fn new(gl: GlGraphics, field: Field) -> Self {
        let mut history = History::new();
        history.spawn();

        let red = LinSrgba::new(1.0, 0.0, 0.0, 1.0);
        let white = LinSrgba::new(1.0, 1.0, 1.0, 1.0);

        let gradient = Gradient::new([red, white]);
        let gradient: Vec<[f32; 4]> = gradient
            .take(HISTORY_LEN)
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

    pub fn renew(self, field: Field) -> Self {
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
        const RADIUS: f64 = 0.002;

        let [width, height] = args.window_size;
        let side = width.max(height);

        self.gl.draw(args.viewport(), |c, gl| {
            use graphics::*;
            // Clear the screen
            clear(BACK, gl);

            let transform = c
                .transform
                .trans(width / 2.0, height / 2.0)
                .scale(side, side)
                .scale(0.5, -0.5);

            for (i, gen) in self.history.gen_iter().enumerate() {
                for l in gen {
                    line(self.gradient[i], RADIUS, l, transform, gl);
                }
            }
        });
    }

    fn update(&mut self, dt: f64) {
        const NUMBER_OF_SPAWNS: usize = 10;
        const EXPIRATION_DATE: u32 = 150;

        (0..NUMBER_OF_SPAWNS).for_each(|_| self.history.spawn());
        self.history.expires(EXPIRATION_DATE);

        for p in self.history.data_iter_mut() {
            let &(x, y) = p.last().unwrap();
            let (dx, dy) = (self.field)(x, y, self.time);
            p.push((x + dx * dt, y + dy * dt));
        }
        self.time += dt;
    }

    pub fn handle_envent(&mut self, event: &Event) {
        if let Some(args) = event.render_args() {
            self.render(&args);
        }

        if let Some(args) = event.update_args() {
            let dt = args.dt;
            self.update(dt);
        }
    }
}
