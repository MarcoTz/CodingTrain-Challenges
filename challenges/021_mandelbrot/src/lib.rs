use graphics::{
    rectangle, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    SetupContext, Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, Key, ResizeArgs};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MAX_ITER: u64 = 50;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 0.5;
const MIN_Y: f64 = -2.0;
const MAX_Y: f64 = 2.0;

const MOVE_X: f64 = 0.1;
const MOVE_Y: f64 = 0.1;
const ZOOM_FAC: f64 = 0.01;

pub struct Mandelbrot {
    computed: Vec<Vec<i64>>,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl Mandelbrot {
    pub fn new() -> Mandelbrot {
        Mandelbrot {
            computed: vec![],
            min_x: MIN_X,
            max_x: MAX_X,
            min_y: MIN_Y,
            max_y: MAX_Y,
        }
    }

    fn inside(&self, pt: Vec2D) -> i64 {
        let mut next = Vec2D { x: 0.0, y: 0.0 };
        for i in 0..MAX_ITER {
            next = Vec2D {
                x: next.x * next.x - next.y * next.y,
                y: 2.0 * next.x * next.y,
            };
            next += pt;

            if next.abs() > 2.0 {
                return i as i64;
            }
        }
        -1
    }

    fn compute(&mut self, window_width: f64, window_height: f64) {
        let step_x = (self.max_x - self.min_x) / window_width;
        let step_y = (self.max_y - self.min_y) / window_height;

        self.computed.clear();

        for i in 0..=window_width.ceil() as usize {
            self.computed.push(vec![]);
            for j in 0..=window_height.ceil() as usize {
                let x = self.min_x + i as f64 * step_x;
                let y = self.min_y + j as f64 * step_y;
                let inside = self.inside(Vec2D { x, y });
                self.computed[i].push(inside);
            }
        }
    }
}

impl Drawable for Mandelbrot {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();

        for x in 0..(ctx.args.window_size[0].ceil() as usize) {
            for y in 0..(ctx.args.window_size[1].ceil() as usize) {
                if self.computed[x][y] == -1 {
                    continue;
                }

                let color = [1.0 / (self.computed[x][y] as f32), 1.0, 1.0, 1.0];

                rectangle(color, [x as f64, y as f64, 1.0, 1.0], transform, gl);
            }
        }
    }
}

impl Updatable for Mandelbrot {
    fn update(&mut self, _: &UpdateContext) {}
}

impl EventHandler for Mandelbrot {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.compute(ctx.window_size[0], ctx.window_size[1]);
    }

    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release {
            return;
        }

        let key = if let Button::Keyboard(key) = ctx.args.button {
            key
        } else {
            return;
        };

        match key {
            Key::Left => {
                self.min_x -= MOVE_X;
                self.max_x -= MOVE_X;
            }
            Key::Right => {
                self.min_x += MOVE_X;
                self.max_x += MOVE_X;
            }
            Key::Up => {
                self.min_y += MOVE_Y;
                self.max_y += MOVE_Y;
            }
            Key::Down => {
                self.min_y -= MOVE_Y;
                self.max_y -= MOVE_Y;
            }
            Key::Z => {
                self.min_y *= ZOOM_FAC;
                self.max_y *= ZOOM_FAC;
                self.min_x *= ZOOM_FAC;
                self.max_x *= ZOOM_FAC;
            }
            Key::Y => {
                self.min_y /= ZOOM_FAC;
                self.max_y /= ZOOM_FAC;
                self.min_x /= ZOOM_FAC;
                self.max_x /= ZOOM_FAC;
            }
            _ => return,
        }
        self.compute(ctx.window_width, ctx.window_height);
    }
}

impl Runnable for Mandelbrot {
    fn setup(&mut self, ctx: &SetupContext) {
        self.compute(ctx.window_width, ctx.window_height);
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Mandelbrot".to_owned(),
        }
    }
}
