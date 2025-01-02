//! AStar finder

use std::{cmp, collections::BinaryHeap};

use ordered_float::OrderedFloat;

use crate::maze::{coordinates::Coordinates, Path};

use super::FrontierManager;

/// Heuristic + Cost
#[derive(Debug, Clone, PartialEq, Eq)]
struct Heuristic {
    value: OrderedFloat<f32>,
    path: Path,
}

#[derive(Debug, Clone)]
pub struct AStarFinder {
    heap: BinaryHeap<Heuristic>,
    goal: Coordinates,
    current: Path,
}

impl AStarFinder {
    /// Calculate the cost of the current path.
    pub fn cost(&self) -> f32 {
        let first = self.current.first();
        let last = self.current.last();

        first.euclidean_dist(&last)
    }
}

impl FrontierManager for AStarFinder {
    fn init(path: Path, goal: Coordinates) -> Self {
        let mut heap = BinaryHeap::new();

        let current = path.clone();

        // initial cost is 0, use just the heuristic
        let h = path.last().euclidean_dist(&goal);
        heap.push(Heuristic {
            value: (-1f32 * h).into(),
            path,
        });

        Self {
            heap,
            goal,
            current,
        }
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn choose(&mut self, candidates: Vec<Path>) {
        let cost = self.cost();
        for candidate in candidates {
            let new_cost = cost + self.current.last().euclidean_dist(&candidate.last());
            let new_h = new_cost + candidate.last().euclidean_dist(&self.goal);

            let h = Heuristic {
                value: (-1f32 * new_h).into(),
                path: candidate,
            };

            self.heap.push(h);
        }
    }

    fn pop(&mut self) -> Path {
        let path = self.heap.pop().unwrap().path;
        self.current = path.clone();
        path
    }
}

impl PartialOrd for Heuristic {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.value.cmp(&other.value))
    }
}

impl Ord for Heuristic {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value.cmp(&other.value)
    }
}
