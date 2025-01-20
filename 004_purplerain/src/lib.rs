use graphics::{line, types::Color};
use graphics_lib::{
    point::Point, Drawable, DrawingContext, InputHandler, Runnable, Updatable, UpdateContext,
};
use opengl_graphics::GlGraphics;
use window::Size;

const WIDTH: f64 = 600.0;
const HEIGHT: f64 = 600.0;
const MAX_SPEED: f64 = 200.0;
const MAX_LEN: f64 = 80.0;
const MAX_THICK: f64 = 1.0;
const PURPLE: Color = [0.8, 0.0, 0.8, 1.0];
const SPAWN_RATE: f64 = 0.96;

pub struct RainCloud {
    drops: Vec<RainDrop>,
}

impl RainCloud {
    pub fn new() -> RainCloud {
        RainCloud { drops: vec![] }
    }
}

struct RainDrop {
    pos: Point,
    velocity: f64,
    thickness: f64,
    length: f64,
}

impl RainDrop {
    pub fn new(max_x: f64) -> RainDrop {
        RainDrop {
            pos: Point {
                x: rand::random::<f64>() * max_x,
                y: 0.0,
            },
            velocity: rand::random::<f64>() * MAX_SPEED,
            length: rand::random::<f64>() * MAX_LEN,
            thickness: rand::random::<f64>() * MAX_THICK,
        }
    }
}

impl Updatable for RainCloud {
    fn update(&mut self, ctx: &UpdateContext) {
        let mut to_remove = vec![];
        for (ind, drop) in self.drops.iter_mut().enumerate() {
            drop.update(ctx);
            if drop.pos.y > ctx.window_height {
                to_remove.push(ind);
            }
        }

        to_remove.sort();
        for ind in to_remove.into_iter().rev() {
            self.drops.remove(ind);
        }

        if rand::random::<f64>() < SPAWN_RATE {
            self.drops.push(RainDrop::new(ctx.window_width))
        }
    }
}

impl Updatable for RainDrop {
    fn update(&mut self, ctx: &UpdateContext) {
        self.pos.y += self.velocity * ctx.args.dt;
    }
}

impl Drawable for RainDrop {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        let transform = ctx.id_trans();
        line(
            PURPLE,
            self.thickness,
            [self.pos.x, self.pos.y, self.pos.x, self.pos.y + self.length],
            transform,
            gl,
        )
    }
}

impl Drawable for RainCloud {
    fn draw(&self, ctx: &DrawingContext, gl: &mut GlGraphics) {
        for drop in self.drops.iter() {
            drop.draw(ctx, gl);
        }
    }
}

impl InputHandler for RainCloud {}

impl Runnable for RainCloud {
    fn window_size(&self) -> Size {
        Size {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

impl Default for RainCloud {
    fn default() -> RainCloud {
        RainCloud::new()
    }
}
