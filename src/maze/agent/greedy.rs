//! Greedy finder

#[derive(Debug, Clone)]
pub struct GreedyFinder {
	heuristic: i64,
	// todo HEAP
}

impl FrontierStrategy for GreedyFinder {
	fn update_frontier(
		frontier: &mut Vec<Path>,
		candidates: Vec<Path>,
	) {
		todo!()
	}

	fn remove_from_frontier(frontier: &mut Vec<Path>) -> Path {
		todo!()
	}
}
