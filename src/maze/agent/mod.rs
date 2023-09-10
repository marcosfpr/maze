use std::{collections::HashSet, marker::PhantomData};

use rand::seq::SliceRandom;

use crate::{agent::Agent, environment::Environment};

use super::{
	coordinates::{Coordinates, Direction},
	environment::MazeStimuli,
	Maze, Path,
};

/// A Path is a sequence of coordinates and a list of valid actions to take
/// from the last position of the path.
#[derive(Clone, Debug)]
pub struct FinderState {
	pub path: Path,
	pub neighbors: Vec<Direction>,
}

impl FinderState {
	/// Retrieves the last element in the path.
	///
	/// # Panics
	/// Panics if the vector is empty
	pub fn last(&self) -> Coordinates {
		self.path.last()
	}
}

/// Path finder solver
pub struct PathFinder<const N: usize, S: FrontierStrategy<Self>> {
	pub current_solution: Path,
	goal: Coordinates,

	frontier: Vec<FinderState>,
	visited: HashSet<Coordinates>,

	strategy: PhantomData<S>,
}

impl<const N: usize, S: FrontierStrategy<Self>> PathFinder<N, S> {
	/// Pop's from the frontier queue
	pub fn remove_from_frontier(&mut self) -> Option<FinderState> {
		self.frontier.pop()
	}
}

/// How the [`PathFinder`] agent updates
/// it's frontier.
pub trait FrontierStrategy<A: Agent> {
	fn update_frontier(
		agent: &mut A,
		candidates: Vec<FinderState>,
	);
}

impl<const N: usize, S: FrontierStrategy<Self>> Agent for PathFinder<N, S> {
	type Error = ();

	type Action = Direction;

	type Stimuli = MazeStimuli;

	type Environment = Maze<N>;

	fn new(environment: &Self::Environment) -> Self {
		let initial_stimuli = environment.initial_stimuli();
		let initial_path = FinderState {
			path: initial_stimuli.current_path.clone(),
			neighbors: initial_stimuli.neighbors,
		};

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
		let state = self.remove_from_frontier().ok_or_else(|| ())?;

		self.current_solution = state.path.clone();

		let mut viable_neighbors = Vec::new();
		// Visit neighbors of the last element of path
		for action in state.neighbors.iter() {
			let stimuli = environment.update(*action)?;
			if !self.visited.contains(&self.current_solution.last()) {
				viable_neighbors.push(FinderState {
					path: stimuli.current_path,
					neighbors: stimuli.neighbors,
				})
			}
		}

		self.visited.insert(self.current_solution.last());

		S::update_frontier(self, viable_neighbors);

		Ok(())
	}

	fn should_stop(&self) -> bool {
		self.current_solution.last() == self.goal || self.frontier.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct RandomFinder;

impl<const N: usize> FrontierStrategy<PathFinder<N, Self>> for RandomFinder {
	fn update_frontier(
		agent: &mut PathFinder<N, Self>,
		candidates: Vec<FinderState>,
	) {
		if let Some(path) = candidates.choose(&mut rand::thread_rng()) {
			agent.frontier.insert(0, path.clone());
		}
	}
}
