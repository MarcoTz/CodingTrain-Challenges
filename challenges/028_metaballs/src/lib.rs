use ::image::{Rgba, RgbaImage};
use graphics::{
    image, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable, Updatable,
    UpdateContext, WindowConfig,
};
use math::{rand_between, vec2d::Vec2D};
use piston::{Button, ButtonState, MouseButton, ResizeArgs};
use piston_window::{G2dTexture, TextureSettings};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MIN_R: f64 = 10.0;
const MAX_R: f64 = 80.0;
const BALL_SPEED: f64 = 150.0;

struct Metaball {
    pos: Vec2D,
    r: f64,
    vel: Vec2D,
}

pub struct Metaballs {
    balls: Vec<Metaball>,
    image: RgbaImage,
}

impl Metaballs {
    pub fn new() -> Metaballs {
        Metaballs {
            balls: vec![],
            image: RgbaImage::new(WIDTH as u32 + 1, HEIGHT as u32 + 1),
        }
    }
}

impl Drawable for Metaballs {
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

impl Updatable for Metaballs {
    fn update(&mut self, ctx: &mut UpdateContext) {
        for ball in self.balls.iter_mut() {
            ball.update(ctx);
        }

        for x in 0..(ctx.window_width.ceil() as usize) {
            for y in 0..(ctx.window_height.ceil() as usize) {
                let pos = Vec2D::new(x as f64, y as f64);
                let mut trans = 0.0;
                for ball in self.balls.iter() {
                    let dist = pos.dist(&ball.pos);
                    trans += ball.r / dist;
                }
                self.image.put_pixel(
                    x as u32,
                    y as u32,
                    Rgba([255, 255, 255, (trans * 128.0).round() as u8]),
                );
            }
        }
    }
}

impl Updatable for Metaball {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.pos += self.vel * ctx.args.dt;
    }
}

impl EventHandler for Metaballs {
    fn handle_resize(&mut self, ctx: &ResizeArgs) {
        self.image = RgbaImage::new(
            ctx.window_size[0].ceil() as u32 + 1,
            ctx.window_size[1].ceil() as u32 + 1,
        );
    }

    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state == ButtonState::Release
            && ctx.args.button == Button::Mouse(MouseButton::Left)
        {
            self.balls.push(Metaball {
                pos: ctx.mouse_pos.into(),
                r: rand_between(MIN_R, MAX_R),
                vel: Vec2D::rand_unit() * BALL_SPEED,
            })
        }
    }
}

impl Runnable for Metaballs {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            height: HEIGHT,
            width: WIDTH,
            title: "Metaballs".to_owned(),
        }
    }
}
