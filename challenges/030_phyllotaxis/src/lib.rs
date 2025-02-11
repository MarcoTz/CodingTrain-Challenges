use graphics::{
    colors::{Hsv, Rgba},
    ellipse, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable, UpdateContext,
    WindowConfig,
};
use math::vec2d::Vec2D;
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;
const GROW_TIME: f64 = 0.01;
const PETAL_SIZE: f64 = 10.0;
const PHI: f64 = 2.0 * PI * 138.0 / 360.0;
const SCALING: f64 = PETAL_SIZE;

pub struct Petal {
    pos: Vec2D,
    color: Rgba,
}

impl Petal {
    pub fn new(pos: Vec2D, color: Rgba) -> Petal {
        Petal { pos, color }
    }
}

pub struct Phyllotaxis {
    last_growth: f64,
    last_color: Hsv,
    petals: Vec<Petal>,
}

impl Phyllotaxis {
    pub fn new() -> Phyllotaxis {
        let start_color = Hsv::new(0.0, 1.0, 1.0).unwrap();
        Phyllotaxis {
            last_growth: 0.0,
            last_color: start_color,
            petals: vec![Petal::new(Vec2D::default(), start_color.into())],
        }
    }

    pub fn grow(&mut self) {
        let n = self.petals.len() as f64;
        let next_phi = n * PHI;
        let next_r = SCALING * n.sqrt();
        self.last_color.increase_hue(1.0);
        let next_rgb: Rgba = self.last_color.into();
        self.petals
            .push(Petal::new(Vec2D::from_polar(next_r, next_phi), next_rgb))
    }
}

impl Drawable for Phyllotaxis {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for petal in self.petals.iter() {
            petal.draw(ctx, gl)
        }
    }
}

impl Drawable for Petal {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();
        ellipse(
            self.color.into(),
            [self.pos.x, self.pos.y, PETAL_SIZE, PETAL_SIZE],
            transform,
            gl,
        )
    }
}

impl Updatable for Phyllotaxis {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.last_growth += ctx.args.dt;
        if self.last_growth >= GROW_TIME {
            self.grow();
            self.last_growth = 0.0;
        }
    }
}

impl EventHandler for Phyllotaxis {}

impl Runnable for Phyllotaxis {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Phyllotaxis".to_owned(),
        }
    }
}
