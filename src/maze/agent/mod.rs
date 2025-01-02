use std::collections::HashSet;

use rand::seq::SliceRandom;

use crate::{agent::Agent, environment::Environment};

use super::{coordinates::Coordinates, environment::MazeStimuli, Maze, Path};

pub mod astar;
pub mod graph_based;
pub mod greedy;

/// Path finder solver
#[derive(Debug, Clone)]
pub struct PathFinder<const N: usize, F: FrontierManager> {
    pub current_solution: Path,
    goal: Coordinates,
    frontier: F,
    visited: HashSet<Coordinates>,
}

/// How the [`PathFinder`] agent updates
/// it's frontier.
pub trait FrontierManager {
    fn init(path: Path, goal: Coordinates) -> Self;

    fn is_empty(&self) -> bool;

    fn pop(&mut self) -> Path;

    fn choose(&mut self, candidates: Vec<Path>);
}

impl<const N: usize, F: FrontierManager> PathFinder<N, F> {
    fn is_cycle(&self, path: &Path) -> bool {
        let last = path.last();

        path.0[0..path.0.len() - 1].iter().any(|elem| *elem == last)
    }

    fn is_visited(&self, path: &Path) -> bool {
        self.visited.contains(&path.last())
    }
}

impl<const N: usize, F: FrontierManager> Agent for PathFinder<N, F> {
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
            frontier: F::init(initial_path, initial_stimuli.target_position),
            visited: HashSet::new(),
        }
    }

    fn act(&mut self, environment: &mut Self::Environment) -> Result<(), Self::Error> {
        // Remove from frontier
        let state = self.frontier.pop();

        // Update current solution
        self.current_solution = state.clone();

        // Act in the environment
        let stimuli = environment.update(self.current_solution.clone())?;

        // Visit neighbors of the last element of path
        let mut viable_neighbors = Vec::new();
        for action in stimuli.neighbors.iter() {
            let neighbor = self.current_solution.walk(*action);

            if !self.is_visited(&neighbor) && !self.is_cycle(&neighbor) {
                viable_neighbors.push(neighbor);
            }
        }

        self.visited.insert(self.current_solution.last());

        self.frontier.choose(viable_neighbors);

        Ok(())
    }

    fn should_stop(&self) -> bool {
        self.current_solution.last() == self.goal || self.frontier.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct RandomFinder(Vec<Path>);

impl FrontierManager for RandomFinder {
    fn init(path: Path, _goal: Coordinates) -> Self {
        Self(vec![path])
    }

    fn pop(&mut self) -> Path {
        self.0.pop().unwrap()
    }

    fn choose(&mut self, candidates: Vec<Path>) {
        for path in candidates.choose_multiple(&mut rand::thread_rng(), candidates.len()) {
            self.0.insert(0, path.clone());
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
