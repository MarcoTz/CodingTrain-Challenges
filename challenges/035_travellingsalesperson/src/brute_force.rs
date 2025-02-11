use super::{graph::Graph, Path, Solver, NUM_VERTS};

#[allow(unused)]
pub fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        return vec![vec![0]];
    }

    let mut perms = vec![];
    let prev_perm = permutations(n - 1);
    for perm in prev_perm {
        for i in 0..perm.len() {
            let mut next = perm.clone();
            next.insert(i, n - 1);
            perms.push(next);
        }

        let mut last_fixed = perm;
        last_fixed.push(n - 1);
        perms.push(last_fixed);
    }

    perms
}

pub struct BruteForceSolver {
    candidate_paths: Vec<Path>,
    best_path: Path,
    best_weight: f64,
}

#[allow(unused)]
impl BruteForceSolver {
    pub fn new() -> BruteForceSolver {
        BruteForceSolver {
            candidate_paths: permutations(NUM_VERTS),
            best_path: (0..(NUM_VERTS - 1)).collect(),
            best_weight: f64::INFINITY,
        }
    }
}

impl Solver for BruteForceSolver {
    fn current_best_path(&self) -> Path {
        self.best_path.clone()
    }

    fn current_best_weight(&self) -> f64 {
        self.best_weight
    }

    fn progress(&self) -> String {
        format!("Remaining Paths: {}", self.candidate_paths.len())
    }

    fn try_next(&mut self, graph: &Graph) -> Option<Path> {
        if self.candidate_paths.len() == 0 {
            return None;
        }
        let next_path = self.candidate_paths.pop().unwrap();
        let next_weight = graph.weight_path(&next_path);
        if next_weight < self.best_weight {
            self.best_path = next_path.clone();
            self.best_weight = next_weight;

            Some(next_path)
        } else {
            None
        }
    }
}
