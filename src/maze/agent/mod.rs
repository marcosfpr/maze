use std::{collections::HashSet, marker::PhantomData};

use rand::seq::SliceRandom;

use crate::{agent::Agent, environment::Environment};

use super::{coordinates::Coordinates, environment::MazeStimuli, Maze, Path};

/// Path finder solver
pub struct PathFinder<const N: usize, S: FrontierStrategy> {
	pub current_solution: Path,
	goal: Coordinates,

	frontier: Vec<Path>,
	visited: HashSet<Coordinates>,

	strategy: PhantomData<S>,
}

impl<const N: usize, S: FrontierStrategy> PathFinder<N, S> {
	/// Pop's from the frontier queue
	pub fn remove_from_frontier(&mut self) -> Option<Path> {
		self.frontier.pop()
	}
}

/// How the [`PathFinder`] agent updates
/// it's frontier.
pub trait FrontierStrategy {
	fn update_frontier(
		frontier: &mut Vec<Path>,
		candidates: Vec<Path>,
	);
}

impl<const N: usize, S: FrontierStrategy> Agent for PathFinder<N, S> {
	type Error = ();

	type Action = Path;

	type Stimuli = MazeStimuli;

	type Environment = Maze<N>;

	fn new(environment: &Self::Environment) -> Self {
		let initial_stimuli = environment.initial_stimuli();
		let initial_path = initial_stimuli.current_path.clone();

		Self {
			current_solution: initial_stimuli.current_path,
			goal: initial_stimuli.target_position,
			frontier: vec![initial_path.clone()],
			visited: HashSet::new(),
			strategy: PhantomData,
		}
	}

	fn act(
		&mut self,
		environment: &mut Self::Environment,
	) -> Result<(), Self::Error> {
		// Remove from frontier
		let state = self.remove_from_frontier().ok_or_else(|| ())?;

		// Update current solution
		self.current_solution = state.clone();

		// Act in the environment
		let stimuli = environment.update(self.current_solution.clone())?;

		// Visit neighbors of the last element of path
		let mut viable_neighbors = Vec::new();
		for action in stimuli.neighbors.iter() {
			let neighbor = self.current_solution.walk(*action);

			if !self.visited.contains(&neighbor.last()) {
				viable_neighbors.push(neighbor);
			}
		}

		self.visited.insert(self.current_solution.last());

		S::update_frontier(&mut self.frontier, viable_neighbors);

		Ok(())
	}

	fn should_stop(&self) -> bool {
		self.current_solution.last() == self.goal || self.frontier.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct RandomFinder;

impl FrontierStrategy for RandomFinder {
	fn update_frontier(
		frontier: &mut Vec<Path>,
		candidates: Vec<Path>,
	) {
		for path in candidates.choose_multiple(&mut rand::thread_rng(), candidates.len()) {
			frontier.insert(0, path.clone());
		}
	}
}

#[derive(Debug, Clone)]
pub struct BFSFinder;

impl FrontierStrategy for BFSFinder {
	fn update_frontier(
		frontier: &mut Vec<Path>,
		candidates: Vec<Path>,
	) {
		for neighbor in candidates {
			// todo check cycles and visited nodes
			frontier.push(neighbor.clone());
		}
	}
}
