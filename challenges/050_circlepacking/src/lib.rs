use graphics::{
    Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MIN_R: f64 = 10.0;
const MAX_R: f64 = 100.0;
const R_GROW: f64 = 0.1;

const MIN_VERTS: usize = 4;
const MAX_VERTS: usize = 10;

mod area;
mod circle;
use area::Area;
use circle::Circle;

pub struct CirclePacking {
    circles: Vec<Circle>,
    fill_area: Area,
}

impl CirclePacking {
    pub fn new() -> CirclePacking {
        CirclePacking {
            circles: vec![],
            fill_area: Area::new(Vec2D::default(), vec![]),
        }
    }

    fn collides(&self, pos: &Vec2D, r: f64) -> bool {
        self.circles.iter().any(|circ| circ.collides(pos, r))
    }
}

impl Drawable for CirclePacking {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for circle in self.circles.iter() {
            circle.draw(ctx, gl)
        }
        self.fill_area.draw(ctx, gl);
    }
}

impl Updatable for CirclePacking {
    fn update(&mut self, ctx: &mut UpdateContext) {
        let mut new_pos = Vec2D::default();
        while !self.fill_area.inside(&new_pos, MIN_R) || self.collides(&new_pos, MIN_R) {
            new_pos = Vec2D::new(
                rand_between(MAX_R, ctx.window_width - MAX_R),
                rand_between(MAX_R, ctx.window_height - MAX_R),
            );
        }
        let mut r = MIN_R;
        while !self.collides(&new_pos, r) && r <= MAX_R && self.fill_area.inside(&new_pos, r) {
            r += R_GROW;
        }
        r -= R_GROW;

        self.circles.push(Circle::new(new_pos, r));
    }
}

impl EventHandler for CirclePacking {}

impl Runnable for CirclePacking {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.fill_area
            .set_center(Vec2D::new(ctx.window_width / 2.0, ctx.window_height / 2.0));

        let num_verts = (rand::random::<usize>() % (MAX_VERTS - MIN_VERTS)) + MIN_VERTS;
        let angle_diff = 2.0 * PI / num_verts as f64;
        let max_r = (ctx.window_width / 2.0).min(ctx.window_height / 2.0);
        let mut new_verts = vec![];
        for i in 0..num_verts {
            let next_angle = angle_diff * i as f64;
            let next_r = rand_between(0.0, max_r);
            let next_vert = Vec2D::from_polar(next_r, next_angle);
            new_verts.push(next_vert);
        }
        self.fill_area.set_vertices(new_verts);
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "CirclePacking".to_owned(),
        }
    }
}
