use super::{
    ASTEROID_SPEED, AST_MAX_BASE_R, AST_MAX_R_OFF, AST_MAX_VERTS, AST_MIN_BASE_R, AST_MIN_R_OFF,
    AST_MIN_SPLIT, AST_MIN_VERTS,
};
use graphics::{
    colors::Rgba, poly_outline::PolyOutline, Drawable, DrawingContext, Graphics, Updatable,
    UpdateContext,
};
use math::{rand_between, vec2d::Vec2D};
use std::f64::consts::PI;

enum ScreenSide {
    Top,
    Bottom,
    Left,
    Right,
}

impl ScreenSide {
    fn random() -> ScreenSide {
        match rand::random::<usize>() % 4 {
            0 => ScreenSide::Top,
            1 => ScreenSide::Bottom,
            2 => ScreenSide::Left,
            3 => ScreenSide::Right,
            _ => panic!("invalid result mod 4"),
        }
    }
}

pub struct Asteroid {
    pub shape: PolyOutline,
    vel: Vec2D,
    size_mult: f64,
}

impl Asteroid {
    pub fn new(window_width: f64, window_height: f64) -> Asteroid {
        let side = ScreenSide::random();
        let (min_x, max_x, min_y, max_y) = match side {
            ScreenSide::Top => (0.0, window_width, -window_height, 0.0),
            ScreenSide::Bottom => (0.0, window_width, window_height, 2.0 * window_height),
            ScreenSide::Left => (-window_width, 0.0, 0.0, window_height),
            ScreenSide::Right => (window_width, 2.0 * window_width, 0.0, window_height),
        };
        let self_center = Vec2D::new(rand_between(min_x, max_x), rand_between(min_y, max_y));
        let mut vel = Vec2D::new(window_width / 2.0, window_height / 2.0) - self_center;
        vel.set_abs(ASTEROID_SPEED);
        Self::from_center_vel(self_center, vel, 1.0)
    }

    pub fn from_center_vel(center: Vec2D, vel: Vec2D, size_mult: f64) -> Asteroid {
        let verts = Self::rand_verts(size_mult);
        let mut ast = Asteroid {
            shape: PolyOutline::new(center, verts, Rgba::random()),
            vel: Vec2D::default(),
            size_mult,
        };
        ast.vel = vel;
        ast
    }

    fn rand_verts(size_mult: f64) -> Vec<Vec2D> {
        let num_verts = (rand::random::<usize>() % (AST_MIN_VERTS + AST_MAX_VERTS)) + AST_MIN_VERTS;
        let angle_diff = 2.0 * PI / num_verts as f64;
        let mut verts = vec![];
        let base_r = rand_between(size_mult * AST_MIN_BASE_R, size_mult * AST_MAX_BASE_R);
        for i in 0..num_verts {
            let next_angle = i as f64 * angle_diff;
            let next_r =
                base_r + rand_between(size_mult * AST_MIN_R_OFF, size_mult * AST_MAX_R_OFF);
            verts.push(Vec2D::from_polar(next_r, next_angle));
        }
        verts
    }

    pub fn split(self) -> Option<[Asteroid; 2]> {
        if self.shape.diameter() < AST_MIN_SPLIT {
            return None;
        }
        let vel_abs = self.vel.abs();
        let mut new_vel = self.vel.tangent();
        new_vel.set_abs(vel_abs / 2.0);
        let center = self.shape.center();

        let left = Asteroid::from_center_vel(center, new_vel, self.size_mult / 2.0);
        let right = Asteroid::from_center_vel(center, -new_vel, self.size_mult / 2.0);
        Some([left, right])
    }
}

impl Drawable for Asteroid {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.shape.draw(ctx, gl);
    }
}

impl Updatable for Asteroid {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.shape
            .set_center(self.shape.center() + ctx.args.dt * self.vel);
    }
}
