//! Maze environment for path finder agents
//!
//! A maze consists in a matrix of 0's and 1's, where 0's indicate
//! the absence of a wall and 1's indicate that this position is blocked.

use rand::Rng;
use tabled::{settings::Style, tables::IterTable};

use self::{
	coordinates::{Coordinates, Direction},
	generator::RbGenerator,
};

pub mod agent;
pub mod coordinates;
pub mod environment;
pub mod generator;

/// Maze consists in a square matrix with obstacles.
///
/// The maze is built using the Recursive-Backtracking algorithm
/// The algorithm works as follows:
///
/// 1. Choose a starting point in the field and make it the current cell
/// 2. Randomly choose a direction, check if the field in that direction has not yet been visited.
///     If that is the case, make the cell in that direction the new current cell and carve a passage between the two.
/// 3. If all adjacent fields have been visited, back up to the last field with unvisited neighbors.
/// 4. The algorithm terminates when it has backed up all the way to the starting point.
pub struct Maze<const N: usize> {
	pub initial_position: Coordinates,
	pub target_position: Coordinates,
	pub current_path: Path,
	maze: [[Ground; N]; N],
}

impl<const N: usize> Maze<N> {
	/// Creates a new maze.
	///
	/// # Panics
	/// Panics if the density is not higher than 100.
	pub fn new(
		initial_position: Coordinates,
		target_position: Coordinates,
		density: u8,
	) -> Self {
		assert!(density <= 100);

		// Just a naive way to initalize the matrix
		let mut rng = rand::thread_rng();

		let mut maze: [[Ground; N]; N] = [[Ground::Free; N]; N];

		for i in 0..N {
			for j in 0..N {
				maze[i][j] = rng.gen_ratio(density as u32, 100).into();
			}
		}

		// Makes sure that the initial and target position dont not have an obstacle
		maze[initial_position.x as usize][initial_position.y as usize] = Ground::Free;
		maze[target_position.x as usize][target_position.y as usize] = Ground::Free;

		let maze = Self {
			initial_position,
			target_position,
			current_path: Path::new(vec![initial_position]),
			maze,
		};

		maze.carve()
	}

	/// Enables the maze by carving cells from target to initial position.
	fn carve(mut self) -> Self {
		let mut gen = RbGenerator::new(None);

		let goal = self.target_position;
		gen.carve(&mut self, goal);

		self
	}

	/// Get the value of the cell if it exists
	pub fn get(
		&self,
		coordinates: &Coordinates,
	) -> Option<Ground> {
		if coordinates.x < 0
			|| coordinates.y < 0
			|| coordinates.x >= N as i64
			|| coordinates.y >= N as i64
		{
			return None;
		}

		Some(self.maze[coordinates.x as usize][coordinates.y as usize])
	}

	/// Get the value of the cell if it exists
	pub fn get_mut(
		&mut self,
		coordinates: &Coordinates,
	) -> Option<&mut Ground> {
		if coordinates.x < 0
			|| coordinates.y < 0
			|| coordinates.x >= N as i64
			|| coordinates.y >= N as i64
		{
			return None;
		}

		Some(&mut self.maze[coordinates.x as usize][coordinates.y as usize])
	}

	/// Gets the neighbors of a position where  there's a valid path.
	pub fn neighbors(
		&self,
		pos: Coordinates,
	) -> Vec<Direction> {
		[
			Direction::East,
			Direction::North,
			Direction::South,
			Direction::West,
		]
		.into_iter()
		.map(|dir| (dir, pos.next(dir)))
		.filter(|(_, coord)| {
			if let Some(ground) = self.get(coord) {
				matches!(ground, Ground::Free)
			} else {
				false
			}
		})
		.map(|(dir, _)| dir)
		.collect()
	}
}

/// Type of ground in a maze.
///
/// A maze can be blocked (meaning a wall) or free (meaning free to pass)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ground {
	Blocked,
	Free,
	Path,
}

/// Path is a sequence of coordinates
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Path(Vec<Coordinates>);

impl Path {
	pub fn new(coordinates: Vec<Coordinates>) -> Self {
		Self(coordinates)
	}

	pub fn get(&self) -> &[Coordinates] {
		&self.0
	}

	pub fn last(&self) -> Coordinates {
		self.0.last().unwrap().clone()
	}

	pub fn walk(
		&self,
		dir: Direction,
	) -> Self {
		let mut new_path = self.0.clone();
		new_path.push(self.last().next(dir));
		Self(new_path)
	}
}

impl From<bool> for Ground {
	fn from(value: bool) -> Self {
		if value {
			Self::Blocked
		} else {
			Self::Free
		}
	}
}

impl AsRef<str> for Ground {
	fn as_ref(&self) -> &str {
		match self {
			Self::Blocked => "◼",
			Self::Free => " ",
			Self::Path => "★",
		}
	}
}

impl<const N: usize> std::fmt::Display for Maze<N> {
	fn fmt(
		&self,
		f: &mut std::fmt::Formatter<'_>,
	) -> std::fmt::Result {
		let mut maze_with_path = self.maze.clone();

		for coord in self.current_path.get() {
			maze_with_path[coord.x as usize][coord.y as usize] = Ground::Path;
		}

		let table = IterTable::new(maze_with_path.iter())
			.with(Style::modern().remove_horizontal().remove_vertical())
			.to_string();

		f.write_str(&table)?;
		Ok(())
	}
}
