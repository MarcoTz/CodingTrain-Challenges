use graphics::{
    colors::Rgba, line, Drawable, DrawingContext, EventHandler, Graphics, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;
const RADIUS: f64 = 200.0;
const RES: usize = 800;
const WOBBLE: f64 = 50.0;
const NUM_GRADIENTS: usize = 20;

pub struct Blobby {
    color: Rgba,
    radii: Vec<f64>,
}

impl Blobby {
    pub fn new() -> Blobby {
        Blobby {
            color: Rgba::random(),
            radii: Self::generate_radii(),
        }
    }

    fn generate_radii() -> Vec<f64> {
        let mut gradients = vec![];
        for _ in 0..NUM_GRADIENTS {
            gradients.push(rand_between(-1.0, 1.0));
        }
        gradients.push(gradients[0]);

        let mut radii = vec![];
        let angle_step = 2.0 * PI / RES as f64;
        let gradient_step = 2.0 * PI / NUM_GRADIENTS as f64;
        for i in 0..RES {
            let circle_pos = i as f64 * angle_step;
            let mut ind = 0;
            while circle_pos >= ind as f64 * gradient_step {
                ind += 1;
            }

            let prev_dist = ((ind - 1) as f64 * gradient_step - circle_pos).abs();
            let prev_gradient = gradients[ind - 1];
            let next_dist = ((ind) as f64 * gradient_step - circle_pos).abs();
            let next_gradient = gradients[ind];
            let prev_dot = prev_dist * prev_gradient;
            let next_dot = next_dist * next_gradient;
            let t = prev_dist / gradient_step;
            let smooth_step = |x: f64| 3.0 * x * x - 2.0 * x * x * x;
            let next_radius =
                RADIUS + 2.0 * WOBBLE * (prev_dot + smooth_step(t) * (next_dot - prev_dot));
            radii.push(next_radius)
        }

        radii
    }
}

impl Drawable for Blobby {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.center_trans();

        let angle_step = 2.0 * PI / RES as f64;
        for (i, r) in self.radii.iter().enumerate() {
            let next_phi = i as f64 * angle_step;
            let next_pos = Vec2D::from_polar(*r, next_phi);
            line(
                self.color.into(),
                2.0,
                [0.0, 0.0, next_pos.x, next_pos.y],
                transform,
                gl,
            );
        }
    }
}

impl Updatable for Blobby {
    fn update(&mut self, _: &mut UpdateContext) {}
}

impl EventHandler for Blobby {}

impl Runnable for Blobby {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Blobby".to_owned(),
        }
    }
}
