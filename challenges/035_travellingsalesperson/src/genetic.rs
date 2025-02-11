use super::{graph::Graph, Path, Solver, NUM_VERTS};

const GEN_SIZE: usize = 100;
const MUTATION_RATE: f64 = 0.5;
const NUM_PARENTS: usize = GEN_SIZE / 10;

#[derive(Clone)]
struct WeightedPath {
    path: Path,
    weight: f64,
}

impl Default for WeightedPath {
    fn default() -> WeightedPath {
        WeightedPath {
            path: vec![],
            weight: f64::INFINITY,
        }
    }
}

pub struct GeneticSolver {
    current_paths: Vec<WeightedPath>,
    current_gen: u64,
}

impl GeneticSolver {
    pub fn new() -> GeneticSolver {
        GeneticSolver {
            current_paths: vec![],
            current_gen: 0,
        }
    }

    fn best_path(&self) -> WeightedPath {
        if self.current_paths.is_empty() {
            WeightedPath::default()
        } else {
            self.current_paths
                .iter()
                .min_by(|path1, path2| path1.weight.partial_cmp(&path2.weight).unwrap())
                .unwrap()
                .clone()
        }
    }

    fn random_path() -> Path {
        let mut unpermuted: Vec<usize> = (0..NUM_VERTS).collect();
        let mut path = vec![];
        while !unpermuted.is_empty() {
            let next_ind = rand::random::<usize>() % unpermuted.len();
            path.push(unpermuted.remove(next_ind));
        }
        path
    }

    fn first_gen(&mut self, graph: &Graph) {
        for _ in 0..GEN_SIZE {
            let next_path = Self::random_path();
            let next_weight = graph.weight_path(&next_path);
            self.current_paths.push(WeightedPath {
                path: next_path,
                weight: next_weight,
            })
        }
    }

    fn cross_over(path1: &Path, path2: &Path) -> Path {
        let from_path1 = rand::random::<usize>() % path1.len();
        let mut next_path: Path = path1.iter().take(from_path1).copied().collect();
        for ind in path2.iter() {
            if !next_path.contains(ind) {
                next_path.push(*ind)
            }
        }
        next_path
    }

    fn mutate(path: &mut Path) {
        if rand::random::<f64>() > MUTATION_RATE {
            return;
        }

        let ind1 = rand::random::<usize>() % path.len();
        let ind2 = rand::random::<usize>() % path.len();
        let tmp = path[ind1];
        path[ind1] = path[ind2];
        path[ind2] = tmp;
    }

    fn next_gen(parents: Vec<&WeightedPath>, graph: &Graph) -> Vec<WeightedPath> {
        let mut next_paths = vec![];
        for _ in 0..GEN_SIZE {
            let parent1_ind = rand::random::<usize>() % parents.len();
            let parent2_ind = rand::random::<usize>() % parents.len();
            let parent1 = &parents[parent1_ind];
            let parent2 = &parents[parent2_ind];
            let mut next_path = Self::cross_over(&parent1.path, &parent2.path);
            Self::mutate(&mut next_path);
            let next_weight = graph.weight_path(&next_path);
            next_paths.push(WeightedPath {
                path: next_path,
                weight: next_weight,
            })
        }
        next_paths
    }
}

impl Solver for GeneticSolver {
    fn current_best_path(&self) -> Path {
        self.best_path().path
    }

    fn current_best_weight(&self) -> f64 {
        self.best_path().weight
    }

    fn progress(&self) -> String {
        format!("Current Gen: {}", self.current_gen)
    }

    fn try_next(&mut self, graph: &Graph) -> Option<Path> {
        if self.current_gen == 0 {
            self.first_gen(graph);
            self.current_gen += 1;
            return Some(self.best_path().path);
        }

        let last_best = self.current_best_weight();
        self.current_paths
            .sort_by(|path1, path2| path1.weight.partial_cmp(&path2.weight).unwrap());
        let parents: Vec<&WeightedPath> = self.current_paths.iter().take(NUM_PARENTS).collect();
        let next_gen = Self::next_gen(parents, graph);
        self.current_paths = next_gen;
        self.current_gen += 1;

        let next_best = self.current_best_weight();

        if next_best < last_best {
            Some(self.current_best_path())
        } else {
            None
        }
    }
}
