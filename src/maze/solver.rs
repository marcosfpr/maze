use std::{
	io::{stdout, Write},
	thread, time,
};

use crossterm::{cursor, terminal, QueueableCommand};

use crate::{agent::Agent, maze::agent::PathFinder};

use super::{agent::BFSFinder, Maze};

/// Solve the maze puzzle and print the results.
pub fn solve<const N: usize>(maze: &mut Maze<N>) -> Result<(), ()> {
	let mut agent = PathFinder::<N, BFSFinder>::new(&maze);
	print_maze(&maze);
	while !agent.should_stop() {
		agent.act(maze)?;
		print_maze(&maze);
	}

	eprintln!("Solution found:");
	eprintln!("{}", maze);

	Ok(())
}

fn print_maze<const N: usize>(maze: &Maze<N>) {
	let mut stdout = stdout();

	stdout.queue(cursor::SavePosition).unwrap();
	stdout.write_all(format!("{}", maze).as_bytes()).unwrap();
	stdout.queue(cursor::RestorePosition).unwrap();
	stdout.flush().unwrap();
	thread::sleep(time::Duration::from_millis(100));

	stdout.queue(cursor::RestorePosition).unwrap();
	stdout
		.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
		.unwrap();
}

#[cfg(test)]
mod tests {

	use crate::maze::{coordinates::Coordinates, solver::solve, Maze};

	use super::print_maze;

	#[test]
	fn test_maze_creation() {
		let mut maze = Maze::<20>::new(Coordinates::new(0, 0), Coordinates::new(19, 19), 5);

		print_maze(&maze);

		solve(&mut maze).unwrap();

		println!("Solution found:");
		println!("{}", maze);
	}
}
