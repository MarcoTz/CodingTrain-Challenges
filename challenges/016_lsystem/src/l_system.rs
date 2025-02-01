use std::{collections::HashMap, hash::Hash};

pub trait Symbol: PartialEq + Eq + Hash + Clone {}

pub struct LSystem<T: Symbol> {
    pub axiom: Vec<T>,
    pub rules: HashMap<T, Vec<T>>,
}

impl<T: Symbol> LSystem<T> {
    pub fn next(&self, state: &[T]) -> Vec<T> {
        let mut next = vec![];
        for t in state {
            next.extend(self.rules.get(&t).cloned().unwrap_or(vec![t.clone()]));
        }
        next
    }
}
