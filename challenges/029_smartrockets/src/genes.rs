use super::{MUTATION_RATE, NUM_GENES, STEERING_FORCE};
use core::array;
use math::{rand_between, vec2d::Vec2D};
use std::f64::consts::PI;

#[derive(Clone)]
pub struct Genes {
    steering: [Vec2D; NUM_GENES],
    current_gene: usize,
}

impl Genes {
    pub fn new() -> Genes {
        Genes {
            current_gene: 0,
            steering: array::from_fn(|_| Vec2D::rand_unit() * STEERING_FORCE),
        }
    }

    pub fn mutate(&mut self) {
        if rand::random::<f64>() >= MUTATION_RATE {
            return;
        }
        let ind = rand::random::<usize>() % NUM_GENES;
        self.steering[ind] = STEERING_FORCE * Vec2D::rand_unit();
    }

    pub fn cross(&self, other: &Genes) -> Genes {
        let new_genes = array::from_fn(|ind| {
            let between = rand::random::<f64>();
            between * self.steering[ind] + (1.0 - between) * other.steering[ind]
        });
        Genes {
            current_gene: 0,
            steering: new_genes,
        }
    }
}

impl Iterator for Genes {
    type Item = Vec2D;
    fn next(&mut self) -> Option<Self::Item> {
        let next = Some(self.steering[self.current_gene]);
        self.current_gene += 1;
        self.current_gene = self.current_gene % NUM_GENES;
        next
    }
}
