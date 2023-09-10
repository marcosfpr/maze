use crate::{agent::Agent, maze::agent::PathFinder};

use super::{agent::RandomFinder, Maze};

/// Solve the maze puzzle and print the results.
pub fn solve<const N: usize>(mut maze: Maze<N>) -> Result<(), ()> {
	let mut agent = PathFinder::<N, RandomFinder>::new(&maze);
	eprintln!("{}", maze);
	while !agent.should_stop() {
		agent.act(&mut maze)?;
		eprintln!("{}", maze);
	}
	eprintln!("{}", maze);

	Ok(())
}

#[cfg(test)]
mod tests {

	use crate::maze::{coordinates::Coordinates, solver::solve, Maze};

	#[test]
	fn test_maze_creation() {
		let maze = Maze::<10>::new(Coordinates::new(0, 0), Coordinates::new(9, 9), 50);

		eprintln!("{}", maze);

		solve(maze).unwrap();
	}
}
