use super::GRID_SQUARE;
use graphics::{rectangle, types::Color};
use graphics_lib::{
    Drawable, DrawingContext, EventHandler, Graphics, InputContext, Updatable, UpdateContext,
};
use piston::{Button, Key};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Debug)]
struct BodySegment {
    x: u64,
    y: u64,
    full: bool,
    color: Color,
}

impl BodySegment {
    fn new(x: u64, y: u64, color: Color) -> BodySegment {
        BodySegment {
            x,
            y,
            full: false,
            color,
        }
    }
}

pub struct Snake {
    tail: Vec<BodySegment>,
    speed: f64,
    dir: Dir,
    tick: f64,
}

impl Snake {
    pub fn new(center_x: u64, center_y: u64) -> Snake {
        Snake {
            dir: Dir::Right,
            tail: vec![BodySegment::new(center_x, center_y, [0.3, 0.3, 1.0, 1.0])],
            speed: 5.0,
            tick: 0.0,
        }
    }

    pub fn pos(&self) -> (u64, u64) {
        let head = self.tail.first().unwrap();
        (head.x, head.y)
    }

    pub fn grow(&mut self) {
        let head = self.tail.get_mut(0).unwrap();
        head.full = true;
    }

    pub fn check_death(&self, window_width: f64, window_height: f64) -> bool {
        let head = self.tail.first().unwrap();
        head.x == 0
            || head.y == 0
            || (head.x as f64) * GRID_SQUARE > window_width
            || (head.y as f64) * GRID_SQUARE > window_height
            || self
                .tail
                .iter()
                .enumerate()
                .any(|(ind, seg)| ind != 0 && seg.x == head.x && seg.y == head.y)
    }
}

impl Drawable for BodySegment {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        let mut x = (self.x as f64 + 0.1) * GRID_SQUARE;
        let mut y = (self.y as f64 + 0.1) * GRID_SQUARE;
        let transform = ctx.id_trans();
        let mut size = 0.8 * GRID_SQUARE;
        if self.full {
            size = GRID_SQUARE;
            x -= 0.1 * GRID_SQUARE;
            y -= 0.1 * GRID_SQUARE;
        }

        rectangle(self.color, [x, y, size, size], transform, gl);
    }
}

impl Drawable for Snake {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        for seg in self.tail.iter() {
            seg.draw(ctx, gl);
        }
    }
}

impl Updatable for Snake {
    fn update(&mut self, ctx: &UpdateContext) {
        self.tick += ctx.args.dt * self.speed;
        if self.tick < 1.0 {
            return;
        }
        self.tick -= 1.0;

        let mut new_head = self.tail.first().unwrap().clone();
        new_head.full = false;

        match self.dir {
            Dir::Left => new_head.x -= 1,
            Dir::Right => new_head.x += 1,
            Dir::Up => new_head.y -= 1,
            Dir::Down => new_head.y += 1,
        }

        let len = self.tail.len();
        if self.tail[len - 1].full {
            self.tail[len - 1].full = false;
            self.tail.insert(0, new_head);
        } else {
            self.tail.rotate_right(1);
            self.tail[0] = new_head;
        }
    }
}

impl EventHandler for Snake {
    fn handle_input(&mut self, ctx: &InputContext) {
        let key = if let Button::Keyboard(key) = ctx.args.button {
            key
        } else {
            return;
        };

        match key {
            Key::W | Key::Up if self.dir != Dir::Down => self.dir = Dir::Up,
            Key::A | Key::Left if self.dir != Dir::Right => self.dir = Dir::Left,
            Key::S | Key::Down if self.dir != Dir::Up => self.dir = Dir::Down,
            Key::D | Key::Right if self.dir != Dir::Left => self.dir = Dir::Right,
            _ => (),
        }
    }
}
