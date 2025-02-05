use graphics::{
    ellipse, Color, Drawable, DrawingContext, EventHandler, Graphics, InputContext, Runnable,
    Updatable, UpdateContext, WindowConfig,
};
use math::vec2d::Vec2D;
use piston::{Button, ButtonState, MouseButton};

const WIDTH: f64 = 800.0;
const HEIGHT: f64 = 900.0;

const MAX_R: f64 = 100.0;
const MIN_R: f64 = 50.0;
const MAX_SPEED: f64 = 25.0;
const START_CELLS: usize = 50;

struct Cell {
    color: Color,
    pos: Vec2D,
    radius: f64,
    velocity: Vec2D,
}

impl Cell {
    pub fn new(window_width: f64, window_height: f64) -> Cell {
        let radius = MIN_R + (rand::random::<f64>() * (MAX_R - MIN_R));
        Cell {
            color: [rand::random(), rand::random(), rand::random(), 0.5],
            pos: Vec2D {
                x: rand::random::<f64>() * (window_width - 2.0 * radius),
                y: rand::random::<f64>() * (window_height - 2.0 * radius),
            },
            radius,
            velocity: MAX_SPEED * Vec2D::rand_unit(),
        }
    }

    pub fn from_pos_radius(pos: Vec2D, radius: f64) -> Cell {
        Cell {
            pos,
            radius,
            color: [rand::random(), rand::random(), rand::random(), 0.5],
            velocity: MAX_SPEED * Vec2D::rand_unit(),
        }
    }

    pub fn inside(&self, pt: &Vec2D) -> bool {
        let center = Vec2D {
            x: self.pos.x + self.radius,
            y: self.pos.y + self.radius,
        };
        let dist = pt.dist(&center);
        dist <= self.radius
    }
}

#[derive(Default)]
pub struct Mitosis {
    cells: Vec<Cell>,
}

impl Mitosis {
    pub fn new() -> Mitosis {
        Mitosis { cells: vec![] }
    }
}

impl Drawable for Mitosis {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        for cell in self.cells.iter() {
            cell.draw(ctx, gl);
        }
    }
}

impl Drawable for Cell {
    fn draw(&self, ctx: &mut DrawingContext, gl: &mut Graphics) {
        let transform = ctx.id_trans();
        ellipse(
            self.color,
            [self.pos.x, self.pos.y, 2.0 * self.radius, 2.0 * self.radius],
            transform,
            gl,
        );
    }
}

impl Updatable for Mitosis {
    fn update(&mut self, ctx: &UpdateContext) {
        if self.cells.is_empty() {
            for _ in 0..START_CELLS {
                self.cells
                    .push(Cell::new(ctx.window_width, ctx.window_height));
            }
        }
        for cell in self.cells.iter_mut() {
            cell.update(ctx)
        }
    }
}

impl Updatable for Cell {
    fn update(&mut self, ctx: &UpdateContext) {
        self.pos += self.velocity * ctx.args.dt;
        if self.pos.x <= 0.0 || self.pos.x + 2.0 * self.radius >= ctx.window_width {
            self.velocity.x *= -1.0;
        }

        if self.pos.y <= 0.0 || self.pos.y + 2.0 * self.radius >= ctx.window_height {
            self.velocity.y *= -1.0;
        }
    }
}

impl EventHandler for Mitosis {
    fn handle_input(&mut self, ctx: &InputContext) {
        if ctx.args.state != ButtonState::Release
            || ctx.args.button != Button::Mouse(MouseButton::Left)
        {
            return;
        }
        let mouse_pos = Vec2D {
            x: ctx.mouse_pos[0],
            y: ctx.mouse_pos[1],
        };
        let mut clicked: Vec<usize> = self
            .cells
            .iter()
            .enumerate()
            .filter_map(|(ind, cell)| cell.inside(&mouse_pos).then_some(ind))
            .collect();
        clicked.sort();
        clicked.reverse();
        for ind in clicked {
            let cell = self.cells.remove(ind);
            let new_left = Cell::from_pos_radius(mouse_pos, cell.radius / 2.0);
            let new_right = Cell::from_pos_radius(mouse_pos, cell.radius / 2.0);
            self.cells.push(new_left);
            self.cells.push(new_right);
        }
    }
}

impl Runnable for Mitosis {
    fn config(&self) -> WindowConfig {
        WindowConfig {
            width: WIDTH,
            height: HEIGHT,
            title: "Mitosis".to_owned(),
        }
    }
}
