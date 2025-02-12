use crate::{
    colors::{rgba, Rgba},
    rectangle, Drawable, DrawingContext, EventHandler, Graphics, InputContext,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, Key, MouseButton};
use piston_window::text::Text;

pub struct TextField {
    pos: Vec2D,
    width: f64,
    height: f64,
    background: Rgba,
    active: bool,
    shift: bool,
    text: String,
    text_color: Rgba,
    font_size: u32,
}
impl TextField {
    pub fn new(x: f64, y: f64, w: f64, h: f64, font_size: u32) -> TextField {
        TextField {
            pos: Vec2D::new(x, y),
            width: w,
            height: h,
            background: rgba::WHITE,
            active: false,
            shift: false,
            text: "".to_owned(),
            text_color: rgba::BLACK,
            font_size,
        }
    }

    pub fn set_text_color(&mut self, col: Rgba) {
        self.text_color = col;
    }

    pub fn set_bg_color(&mut self, col: Rgba) {
        self.background = col;
    }

    pub fn set_pos(&mut self, x: f64, y: f64) {
        self.pos = Vec2D::new(x, y);
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn text(&self) -> String {
        self.text.clone()
    }

    pub fn size(&self) -> [f64; 2] {
        [self.width, self.height]
    }

    pub fn pos(&self) -> Vec2D {
        self.pos
    }

    pub fn active(&self) -> bool {
        self.active
    }

    fn inside(&self, pos: &Vec2D) -> bool {
        pos.x >= self.pos.x
            && pos.x <= self.pos.x + self.width
            && pos.y >= self.pos.y
            && pos.y <= self.pos.y + self.height
    }

    fn check_shift(&mut self, st: &ButtonState, bt: &Button) {
        if *bt != Button::Keyboard(Key::LShift) && *bt != Button::Keyboard(Key::RShift) {
            return;
        }
        self.shift = *st == ButtonState::Press;
    }

    fn check_active(&mut self, bt: &Button, mouse_pos: &Vec2D) {
        if *bt != Button::Mouse(MouseButton::Left) {
            return;
        }

        self.active = self.inside(mouse_pos)
    }

    fn check_input(&mut self, bt: &Button) {
        let key = if let Button::Keyboard(key) = *bt {
            key
        } else {
            return;
        };

        if key == Key::Backspace {
            self.text.pop();
            return;
        }

        match key_to_char(key) {
            None => (),
            Some(ch) => {
                if self.shift {
                    self.text.push(ch.to_ascii_uppercase())
                } else {
                    self.text.push(ch)
                }
            }
        }
    }
}

impl Drawable for TextField {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        rectangle(
            self.background.into(),
            [self.pos.x, self.pos.y, self.width, self.height],
            transform,
            gl,
        );

        let text = Text::new_color(self.text_color.into(), self.font_size);

        let mut next_y = self.pos.y + self.font_size as f64;
        let mut next_line = "".to_owned();
        let mut current_size = 0.0;
        for ch in self.text.chars() {
            next_line.push(ch);
            let next_size = ctx
                .glyphs
                .opt_character(self.font_size, ch)
                .map(|ch| ch.advance_width())
                .unwrap_or(self.font_size as f64);
            current_size += next_size;
            if current_size + next_size > self.width {
                text.draw_pos(
                    &next_line,
                    [self.pos.x, next_y],
                    ctx.glyphs,
                    &ctx.context.draw_state,
                    transform,
                    gl,
                )
                .unwrap();
                next_line.clear();
                next_y += self.font_size as f64;
                current_size = 0.0;
                if next_y > self.pos.y + self.height {
                    break;
                }
            }
        }

        if !next_line.is_empty() {
            text.draw_pos(
                &next_line,
                [self.pos.x, next_y],
                ctx.glyphs,
                &ctx.context.draw_state,
                transform,
                gl,
            )
            .unwrap();
        }
    }
}

impl EventHandler for TextField {
    fn handle_input(&mut self, ctx: &InputContext) {
        self.check_shift(&ctx.args.state, &ctx.args.button);

        if ctx.args.state != ButtonState::Release {
            return;
        }

        self.check_active(&ctx.args.button, &ctx.mouse_pos.into());
        if self.active {
            self.check_input(&ctx.args.button);
        }
    }
}

fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::A => Some('a'),
        Key::B => Some('b'),
        Key::C => Some('c'),
        Key::D => Some('d'),
        Key::E => Some('e'),
        Key::F => Some('f'),
        Key::G => Some('g'),
        Key::H => Some('h'),
        Key::I => Some('i'),
        Key::J => Some('j'),
        Key::K => Some('k'),
        Key::L => Some('l'),
        Key::M => Some('m'),
        Key::N => Some('n'),
        Key::O => Some('o'),
        Key::P => Some('p'),
        Key::Q => Some('q'),
        Key::R => Some('r'),
        Key::S => Some('s'),
        Key::T => Some('t'),
        Key::U => Some('u'),
        Key::V => Some('v'),
        Key::W => Some('w'),
        Key::X => Some('x'),
        Key::Y => Some('y'),
        Key::Z => Some('z'),
        Key::Space => Some(' '),
        _ => None,
    }
}
