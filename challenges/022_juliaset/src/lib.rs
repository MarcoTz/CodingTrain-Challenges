use ::image::{Rgba, RgbaImage};
use graphics::{
    image, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable, SetupContext,
    Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, Key, ResizeArgs};
use piston_window::{G2dTexture, TextureSettings};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MAX_ITER: u64 = 255;
const MAX_RADIUS: f64 = 5.0;

const MIN_X: f64 = -2.0;
const MAX_X: f64 = 0.5;
const MIN_Y: f64 = -2.0;
const MAX_Y: f64 = 2.0;

const MOVE_X: f64 = 0.1;
const MOVE_Y: f64 = 0.1;
const ZOOM_FAC: f64 = 0.5;

pub struct JuliaSet {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    c: Vec2D,
    image: RgbaImage,
}

impl JuliaSet {
    pub fn new() -> JuliaSet {
        JuliaSet {
            min_x: MIN_X,
            max_x: MAX_X,
            min_y: MIN_Y,
            max_y: MAX_Y,
            c: Vec2D::new(0.285, 0.0001),
            image: RgbaImage::new(WIDTH as u32 + 1, HEIGHT as u32 + 1),
        }
    }
    fn inside(&self, pt: Vec2D) -> u8 {
        let mut next = pt;
        for i in 0..MAX_ITER {
            next = Vec2D {
                x: next.x * next.x - next.y * next.y,
                y: 2.0 * next.x * next.y,
            };
            next += self.c;

            if next.abs() > MAX_RADIUS {
                return i as u8;
            }
        }
        0
    }

    fn compute(&mut self, window_width: f64, window_height: f64) {
        let step_x = (self.max_x - self.min_x) / window_width;
        let step_y = (self.max_y - self.min_y) / window_height;

        for i in 0..self.image.width() as u32 {
            for j in 0..self.image.height() as u32 {
                let x = self.min_x + i as f64 * step_x;
                let y = self.min_y + j as f64 * step_y;
                let inside = self.inside(Vec2D { x, y });
                self.image.put_pixel(i, j, Rgba([255, 255, 255, inside]));
            }
        }
    }
}

impl Drawable for JuliaSet {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        let texture = G2dTexture::from_image(
            &mut ctx.texture_context,
            &self.image,
            &TextureSettings::new(),
        )
        .unwrap();
        image(&texture, transform, gl);
    }
}

impl Updatable for JuliaSet {
    fn update(&mut self, _: &mut UpdateContext) {}
}

impl EventHandler for JuliaSet {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.image = RgbaImage::new(
            ctx.window_size[0].ceil() as u32 + 1,
            ctx.window_size[1].ceil() as u32 + 1,
        );
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
                if self.min_y < 0.0 {
                    self.min_y += ZOOM_FAC
                } else {
                    self.min_y -= ZOOM_FAC
                }

                if self.max_y < 0.0 {
                    self.max_y += ZOOM_FAC
                } else {
                    self.max_y -= ZOOM_FAC
                }

                if self.min_x < 0.0 {
                    self.min_x += ZOOM_FAC
                } else {
                    self.min_x -= ZOOM_FAC
                }

                if self.max_x < 0.0 {
                    self.max_x += ZOOM_FAC
                } else {
                    self.max_x -= ZOOM_FAC
                }
            }
            Key::Y => {
                if self.min_y < 0.0 {
                    self.min_y -= ZOOM_FAC
                } else {
                    self.min_y += ZOOM_FAC
                }

                if self.max_y < 0.0 {
                    self.max_y -= ZOOM_FAC
                } else {
                    self.max_y += ZOOM_FAC
                }

                if self.min_x < 0.0 {
                    self.min_x -= ZOOM_FAC
                } else {
                    self.min_x += ZOOM_FAC
                }

                if self.max_x < 0.0 {
                    self.max_x -= ZOOM_FAC
                } else {
                    self.max_x += ZOOM_FAC
                }
            }
            _ => return,
        }
        self.compute(ctx.window_width, ctx.window_height);
    }
}

impl Runnable for JuliaSet {
    fn setup(&mut self, ctx: &mut SetupContext) {
        self.compute(ctx.window_width, ctx.window_height);
    }

    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "JuliaSet".to_owned(),
        }
    }
}
