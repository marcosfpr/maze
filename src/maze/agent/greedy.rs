//! Greedy finder

use std::{cmp, collections::BinaryHeap};

use ordered_float::OrderedFloat;

use crate::maze::{coordinates::Coordinates, Path};

use super::FrontierManager;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Heuristic {
	value: OrderedFloat<f32>,
	path: Path,
}

#[derive(Debug, Clone)]
pub struct GreedyFinder {
	heap: BinaryHeap<Heuristic>,
	goal: Coordinates,
}

impl FrontierManager for GreedyFinder {
	fn init(
		path: Path,
		goal: Coordinates,
	) -> Self {
		let mut heap = BinaryHeap::new();

		heap.push(Heuristic {
			value: (-1f32 * path.last().euclidean_dist(&goal)).into(),
			path,
		});

		Self {
			heap,
			goal,
		}
	}

	fn is_empty(&self) -> bool {
		self.heap.is_empty()
	}

	fn choose(
		&mut self,
		candidates: Vec<Path>,
	) {
		for candidate in candidates {
			let h = Heuristic {
				value: (-1f32 * candidate.last().euclidean_dist(&self.goal)).into(),
				path: candidate,
			};

			self.heap.push(h);
		}
	}

	fn pop(&mut self) -> Path {
		self.heap.pop().unwrap().path
	}
}

impl PartialOrd for Heuristic {
	fn partial_cmp(
		&self,
		other: &Self,
	) -> Option<cmp::Ordering> {
		self.value.partial_cmp(&other.value)
	}
}

impl Ord for Heuristic {
	fn cmp(
		&self,
		other: &Self,
	) -> cmp::Ordering {
		self.value.cmp(&other.value)
	}
}
