use crate::maze::{coordinates::Coordinates, Path};

use super::FrontierManager;

#[derive(Debug, Clone)]
pub struct BFSFinder(Vec<Path>);

impl FrontierManager for BFSFinder {
    fn init(path: Path, _goal: Coordinates) -> Self {
        Self(vec![path])
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn choose(&mut self, candidates: Vec<Path>) {
        for neighbor in candidates {
            self.0.push(neighbor.clone());
        }
    }

    fn pop(&mut self) -> Path {
        self.0.remove(0)
    }
}

#[derive(Debug, Clone)]
pub struct DFSFinder(Vec<Path>);

impl FrontierManager for DFSFinder {
    fn init(path: Path, _goal: Coordinates) -> Self {
        Self(vec![path])
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn choose(&mut self, candidates: Vec<Path>) {
        for neighbor in candidates {
            self.0.push(neighbor.clone());
        }
    }

    fn pop(&mut self) -> Path {
        self.0.pop().unwrap()
    }
}
