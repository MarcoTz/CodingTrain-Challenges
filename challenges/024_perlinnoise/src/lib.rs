use graphics::{
    rectangle, Drawable, DrawingContext, EventHandler, Graphics, Runnable, SetupContext, Updatable,
    UpdateContext, WindowConfig,
};
use math::{grid::Grid, rand_between, vec2d::Vec2D};
use std::f64::consts::PI;

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const ROWS: usize = 20;
const COLS: usize = 20;

const NUM_SAMPLES_ROW: usize = 20;
const NUM_SAMPLES_COL: usize = 20;

pub struct PerlinNoise {
    grid: Grid<Vec2D>,
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        PerlinNoise {
            grid: Grid::new(COLS, ROWS),
        }
    }
}

impl Drawable for PerlinNoise {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let cell_width = ctx.args.window_size[0] / (COLS as f64 - 1.0);
        let cell_height = ctx.args.window_size[1] / (ROWS as f64 - 1.0);
        let transform = ctx.id_trans();
        for grid_y in 0..ROWS - 1 {
            for grid_x in 0..COLS - 1 {
                let top_left = Vec2D::new(grid_x as f64 * cell_width, grid_y as f64 * cell_height);

                for num_row in 0..NUM_SAMPLES_ROW {
                    for num_col in 0..NUM_SAMPLES_COL {
                        let sample_width = 1.0 / NUM_SAMPLES_COL as f64;
                        let sample_height = 1.0 / NUM_SAMPLES_ROW as f64;
                        let sample = Vec2D::new(
                            num_col as f64 * sample_width + sample_width / 2.0,
                            num_row as f64 * sample_height + sample_height / 2.0,
                        );

                        let offset_top_left = sample;
                        let offset_top_right = sample - Vec2D::new(1.0, 0.0);
                        let offset_bottom_left = sample - Vec2D::new(0.0, 1.0);
                        let offset_bottom_right = sample - Vec2D::new(1.0, 1.0);

                        let inf_top_left = offset_top_left.dot(self.grid[(grid_x, grid_y)]);
                        let inf_top_right = offset_top_right.dot(self.grid[(grid_x + 1, grid_y)]);
                        let inf_bottom_left =
                            offset_bottom_left.dot(self.grid[(grid_x, grid_y + 1)]);
                        let inf_bottom_right =
                            offset_bottom_right.dot(self.grid[(grid_x + 1, grid_y + 1)]);

                        let smoothing =
                            |t: f64| 6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3);
                        let smooth_x = smoothing(sample.x);
                        let smooth_y = smoothing(sample.y);

                        let avg_top = (1.0 - smooth_x) * inf_top_left + smooth_x * inf_top_right;
                        let avg_bot =
                            (1.0 - smooth_x) * inf_bottom_left + smooth_x * inf_bottom_right;
                        let avg = (1.0 - smooth_y) * avg_top + smooth_y * avg_bot;

                        let x = top_left.x + num_col as f64 * (cell_width / NUM_SAMPLES_COL as f64);
                        let y =
                            top_left.y + num_row as f64 * (cell_height / NUM_SAMPLES_ROW as f64);
                        let w = cell_width / NUM_SAMPLES_COL as f64;
                        let h = cell_height / NUM_SAMPLES_ROW as f64;

                        rectangle(
                            [1.0, 1.0, 1.0, (avg + 0.5) as f32],
                            [x, y, w, h],
                            transform,
                            gl,
                        );
                    }
                }
            }
        }
    }
}

impl Updatable for PerlinNoise {
    fn update(&mut self, _: &mut UpdateContext) {
        let rand_dir = rand_between(0.0, PI / 8.0);
        for grid_x in 0..COLS {
            for grid_y in 0..ROWS {
                let old_dir = self.grid[(grid_x, grid_y)].arg();
                let new_dir = old_dir + rand_dir;
                self.grid[(grid_x, grid_y)].set_arg(new_dir);
            }
        }
    }
}

impl EventHandler for PerlinNoise {}

impl Runnable for PerlinNoise {
    fn setup(&mut self, _: &mut SetupContext) {
        for y in 0..ROWS {
            for x in 0..COLS {
                self.grid.insert(x, y, Vec2D::rand_unit());
            }
        }
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Perlin Noise".to_owned(),
        }
    }
}
