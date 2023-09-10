use rand::SeedableRng;
use rand_chacha::ChaChaRng;

use super::{
	coordinates::{Coordinates, Direction},
	Ground, Maze,
};

/// [`Generator`] implementation which uses the recursive-backtracking algorithm.
#[derive(Debug, Clone)]
pub struct RbGenerator {
	rng: ChaChaRng,
}

impl RbGenerator {
	/// Create a new instance.
	///
	/// Optionally a 32 bit seed can be provided to seed the internal random generator.
	/// Giving a seed results in identical mazes being generated which omitting it sources the
	/// random generator from entropy.
	pub fn new(seed: Option<[u8; 32]>) -> RbGenerator {
		RbGenerator {
			rng: match seed {
				None => ChaChaRng::from_entropy(),
				Some(seed) => ChaChaRng::from_seed(seed),
			},
		}
	}

	/// Core algorithm implementation
	///
	/// Carves passages in all directions in random order from the current coordinates but only
	/// if the field in that direction has not yet been processed.
	///
	/// Returns coordinates of the goal field
	///
	/// TODO: Review this logic
	pub fn carve<const N: usize>(
		&mut self,
		maze: &mut Maze<N>,
		current_coordinates: Coordinates,
	) -> Coordinates {
		let mut goal_coords = maze.initial_position;
		for dir in Direction::gen_random_order(&mut self.rng).iter() {
			let next_coords = current_coordinates.next(*dir);

			match maze.get_mut(&next_coords) {
				Some(pos) if matches!(pos, Ground::Blocked) => {
					*pos = Ground::Free;
					if goal_coords == maze.initial_position {
						goal_coords = self.carve(maze, next_coords);
					} else {
						self.carve(maze, next_coords);
					}
				}
				_ => continue,
			}
		}

		if goal_coords == maze.initial_position {
			current_coordinates
		} else {
			goal_coords
		}
	}
}
