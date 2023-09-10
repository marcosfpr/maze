//! Maze Environment

use crate::{
	environment::Environment,
	maze::{
		coordinates::{Coordinates, Direction},
		Maze,
	},
};

use super::Path;

/// Stimuli produced by a [`Maze`] update.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct MazeStimuli {
	pub current_path: Path,
	pub target_position: Coordinates,
	pub neighbors: Vec<Direction>,
}

impl<const N: usize> Environment for Maze<N> {
	type Error = ();

	type Action = Direction;

	type Stimuli = MazeStimuli;

	fn initial_stimuli(&self) -> Self::Stimuli {
		Self::Stimuli {
			current_path: self.current_path.clone(),
			target_position: self.target_position,
			neighbors: self.neighbors(self.initial_position),
		}
	}

	fn update(
		&mut self,
		action: Self::Action,
	) -> Result<Self::Stimuli, Self::Error> {
		self.current_path.walk(action);
		Ok(Self::Stimuli {
			current_path: self.current_path.clone(),
			target_position: self.target_position,
			neighbors: self.neighbors(self.current_path.last()),
		})
	}
}
