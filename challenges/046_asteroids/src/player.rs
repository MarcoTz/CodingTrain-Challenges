use super::{bullet::Bullet, FRICTION, LINE_THICK, PLAYER_ACC, PLAYER_ROT, PLAYER_SIZE};
use graphics::{
    colors::Rgba, poly_outline::PolyOutline, Drawable, DrawingContext, EventHandler, Graphics,
    InputContext, Updatable, UpdateContext,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, Key};

enum RotationState {
    No,
    Left,
    Right,
}

#[derive(Debug)]
enum MoveState {
    No,
    Forward,
    Backward,
}

pub struct Player {
    pub shape: PolyOutline,
    rotating: RotationState,
    mov: MoveState,
    vel: Vec2D,
    acc: Vec2D,
}

impl Player {
    pub fn new() -> Player {
        let mut player = Player {
            shape: PolyOutline::new(
                Vec2D::default(),
                vec![
                    Vec2D::new(-PLAYER_SIZE / 2.0, 0.0),
                    Vec2D::new(PLAYER_SIZE / 2.0, PLAYER_SIZE / 2.0),
                    Vec2D::new(PLAYER_SIZE / 2.0, -PLAYER_SIZE / 2.0),
                ],
                Rgba::random(),
            ),
            rotating: RotationState::No,
            mov: MoveState::No,
            vel: Vec2D::default(),
            acc: Vec2D::default(),
        };
        player.shape.set_line(LINE_THICK);
        player
    }

    pub fn center(&mut self, window_width: f64, window_height: f64) {
        self.shape
            .set_center(Vec2D::new(window_width / 2.0, window_height / 2.0));
    }

    pub fn heading(&self) -> Vec2D {
        let center = self.shape.center();
        let front = center + self.shape.vertices()[0];
        let mut head = front - center;
        head.set_abs(1.0);
        head
    }

    pub fn rotate(&mut self, angle: f64) {
        let rot_mat = [[angle.cos(), -angle.sin()], [angle.sin(), angle.cos()]];
        let old_verts = self.shape.vertices();
        let mut new_verts = vec![];
        for vert in old_verts {
            let new_x = vert.x * rot_mat[0][0] + vert.y * rot_mat[0][1];
            let new_y = vert.x * rot_mat[1][0] + vert.y * rot_mat[1][1];
            new_verts.push(Vec2D::new(new_x, new_y));
        }
        self.shape.set_vertices(new_verts);
    }

    pub fn shoot(&self) -> Bullet {
        let pos = self.shape.center();
        Bullet::new(pos.x, pos.y, self.heading())
    }
}

impl Drawable for Player {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        self.shape.draw(ctx, gl);
    }
}

impl Updatable for Player {
    fn update(&mut self, ctx: &mut UpdateContext) {
        match self.rotating {
            RotationState::No => (),
            RotationState::Left => self.rotate(-PLAYER_ROT * ctx.args.dt),
            RotationState::Right => self.rotate(PLAYER_ROT * ctx.args.dt),
        };

        match self.mov {
            MoveState::No => self.acc = Vec2D::default(),
            MoveState::Forward => self.acc = PLAYER_ACC * self.heading(),
            MoveState::Backward => self.acc = -PLAYER_ACC * self.heading(),
        };

        self.shape
            .set_center(self.shape.center() + ctx.args.dt * self.vel);
        self.vel += self.acc * ctx.args.dt;
        self.vel *= FRICTION;
    }
}

impl EventHandler for Player {
    fn handle_input(&mut self, ctx: &InputContext) {
        match (ctx.args.state, ctx.args.button) {
            (ButtonState::Release, Button::Keyboard(key))
                if matches!(key, Key::A | Key::D | Key::Left | Key::Right) =>
            {
                self.rotating = RotationState::No;
            }
            (ButtonState::Release, Button::Keyboard(key))
                if matches!(key, Key::W | Key::S | Key::Up | Key::Down) =>
            {
                self.mov = MoveState::No
            }
            (ButtonState::Press, Button::Keyboard(key)) if matches!(key, Key::A | Key::Left) => {
                self.rotating = RotationState::Left
            }
            (ButtonState::Press, Button::Keyboard(key)) if matches!(key, Key::D | Key::Right) => {
                self.rotating = RotationState::Right
            }
            (ButtonState::Press, Button::Keyboard(key)) if matches!(key, Key::W | Key::Up) => {
                self.mov = MoveState::Forward
            }
            (ButtonState::Press, Button::Keyboard(key)) if matches!(key, Key::S | Key::Down) => {
                self.mov = MoveState::Backward
            }
            _ => (),
        }
    }
}
