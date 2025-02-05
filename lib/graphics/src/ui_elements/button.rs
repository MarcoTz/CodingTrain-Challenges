use crate::{rectangle, Color, Drawable, DrawingContext, Graphics};
use math::vec2d::Vec2D;
use piston::{Button as PisButton, ButtonArgs, ButtonState, MouseButton};
use piston_window::{text, DrawState};

pub struct Button {
    pub width: f64,
    pub height: f64,
    pub pos: Vec2D,
    pub color: Color,
    pub text: String,
    pub text_color: Color,
    pub font_size: u32,
}

impl Button {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        color: Color,
        text: &str,
        text_color: Color,
        font_size: u32,
    ) -> Button {
        Button {
            width,
            height,
            pos: Vec2D::new(x, y),
            color,
            text: text.to_owned(),
            text_color,
            font_size,
        }
    }

    pub fn clicked(&self, mouse_pos: [f64; 2], args: &ButtonArgs) -> bool {
        if args.button != PisButton::Mouse(MouseButton::Left) || args.state != ButtonState::Release
        {
            return false;
        }
        mouse_pos[0] >= self.pos.x
            && mouse_pos[0] <= self.pos.x + self.width
            && mouse_pos[1] >= self.pos.y
            && mouse_pos[1] <= self.pos.y + self.height
    }
}

impl Drawable for Button {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            self.color,
            [self.pos.x, self.pos.y, self.width, self.height],
            transform,
            gl,
        );

        text::Text::new_color(self.text_color, self.font_size)
            .draw_pos(
                &self.text,
                [self.pos.x, self.pos.y + self.font_size as f64],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
    }
}
