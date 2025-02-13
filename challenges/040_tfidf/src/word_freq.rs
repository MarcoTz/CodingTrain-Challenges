use super::{FONT_SIZE, FRICTION, SIZE_MULT};
use graphics::{
    colors::Rgba, ellipse, Drawable, DrawingContext, Graphics, Transformed, Updatable,
    UpdateContext,
};
use math::vec2d::Vec2D;
use piston_window::text::Text;

pub struct WordFreq {
    pub word: String,
    pub cnt: u64,
    pub tf: f64,
    color: Rgba,
    text_color: Rgba,
    pub pos: Vec2D,
    pub vel: Vec2D,
    pub acc: Vec2D,
}

impl WordFreq {
    pub fn new(word: String, x: f64, y: f64) -> WordFreq {
        WordFreq {
            word,
            cnt: 1,
            tf: 0.0,
            color: Rgba::random(),
            text_color: Rgba::random(),
            pos: Vec2D::new(x, y),
            vel: Vec2D::default(),
            acc: Vec2D::default(),
        }
    }

    pub fn radius(&self) -> f64 {
        self.tf as f64 * SIZE_MULT / 2.0
    }

    pub fn collides(&self, other: &WordFreq) -> bool {
        self.pos.dist(&other.pos) <= self.radius() + other.radius()
    }

    fn borders(&mut self, window_width: f64, window_height: f64) {
        let radius = self.radius();
        if self.pos.x < radius {
            self.pos.x = radius;
            self.vel.x *= -1.0;
        }
        if self.pos.x > window_width - radius {
            self.pos.x = window_width - radius;
            self.vel.x *= -1.0;
        }

        if self.pos.y < radius {
            self.pos.y = radius;
            self.vel.y *= -1.0;
        }
        if self.pos.y > window_height - radius {
            self.pos.y = window_height - radius;
            self.vel.y *= -1.0;
        }
    }
}

impl Drawable for WordFreq {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.context.transform.trans(self.pos.x, self.pos.y);
        let radius = self.radius();
        ellipse(
            self.color.into(),
            [-radius, -radius, 2.0 * radius, 2.0 * radius],
            transform,
            gl,
        );
        Text::new_color(self.text_color.into(), FONT_SIZE)
            .draw_pos(
                self.word.as_str(),
                [-(FONT_SIZE as f64 * self.word.len() as f64) / 2.0, 0.0],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
    }
}

impl Updatable for WordFreq {
    fn update(&mut self, ctx: &mut UpdateContext) {
        self.borders(ctx.window_width, ctx.window_height);
        self.pos += self.vel * ctx.args.dt;
        self.vel += self.acc * ctx.args.dt;
        self.vel *= FRICTION;
    }
}
