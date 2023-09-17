//! Maze Environment

use std::{
	io::{stdout, Write},
	thread, time,
};

use crossterm::{cursor, terminal, QueueableCommand};

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

	type Action = Path;

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
		new_path: Self::Action,
	) -> Result<Self::Stimuli, Self::Error> {
		self.current_path = new_path;

		Ok(Self::Stimuli {
			current_path: self.current_path.clone(),
			target_position: self.target_position,
			neighbors: self.neighbors(self.current_path.last()),
		})
	}

	fn render(&self) {
		let mut stdout = stdout();

		stdout.queue(cursor::SavePosition).unwrap();
		stdout.write_all(format!("{}", self).as_bytes()).unwrap();
		stdout.queue(cursor::RestorePosition).unwrap();
		stdout.flush().unwrap();
		thread::sleep(time::Duration::from_millis(100));

		stdout.queue(cursor::RestorePosition).unwrap();
		stdout
			.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
			.unwrap();
	}
}
