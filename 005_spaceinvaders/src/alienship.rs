use super::{
    alien::Alien, ALIEN_HEIGHT, ALIEN_WIDTH, ALIEN_YSPEED, COL_DIST, ROW_DIST, ROW_OFFSET,
};
use core::array;
use graphics::{Drawable, DrawingContext, Graphics, Updatable, UpdateContext};

pub struct AlienShip<const N: usize, const M: usize> {
    pub alien_rows: [[Alien; M]; N],
}

impl<const N: usize, const M: usize> AlienShip<N, M> {
    pub fn new() -> AlienShip<N, M> {
        AlienShip {
            alien_rows: array::from_fn(|i| {
                let y = i as f64 * (ALIEN_HEIGHT + ROW_DIST) + ROW_DIST;
                let row_offset = if i % 2 == 0 { ROW_OFFSET } else { 0.0 };
                array::from_fn(|j| {
                    let x = j as f64 * (ALIEN_WIDTH + COL_DIST) + COL_DIST + row_offset;
                    Alien::new(x, y)
                })
            }),
        }
    }

    fn reverse_dir(&mut self, even: bool) {
        for (num, row) in self.alien_rows.iter_mut().enumerate() {
            for alien in row.iter_mut() {
                if (num % 2 == 0) == even {
                    alien.x_speed *= -1.0;
                }

                alien.pos.y += ALIEN_YSPEED;
            }
        }
    }
}

impl<const N: usize, const M: usize> Drawable for AlienShip<N, M> {
    fn draw(&self, ctx: &DrawingContext, gl: &mut Graphics) {
        for row in self.alien_rows.iter() {
            for alien in row.iter() {
                alien.draw(ctx, gl);
            }
        }
    }
}

impl<const N: usize, const M: usize> Updatable for AlienShip<N, M> {
    fn update(&mut self, ctx: &UpdateContext) {
        for row in self.alien_rows.iter_mut() {
            for alien in row.iter_mut() {
                alien.update(ctx)
            }
        }
        let rightmost_even = &self.alien_rows[0][M - 1];
        let leftmost_even = &self.alien_rows[0][0];
        if leftmost_even.pos.x <= 0.0 || rightmost_even.pos.x >= ctx.window_width - ALIEN_WIDTH {
            self.reverse_dir(true);
        }
        let rightmost_odd = &self.alien_rows[1][M - 1];
        let leftmost_odd = &self.alien_rows[1][0];
        if leftmost_odd.pos.x <= 0.0 || rightmost_odd.pos.x >= ctx.window_width - ALIEN_WIDTH {
            self.reverse_dir(false);
        }
    }
}
