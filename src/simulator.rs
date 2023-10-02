use std::fmt::Debug;

use crate::{agent::Agent, environment::Environment};

pub struct Simulator<A>
where
	A: Agent,
{
	agent: A,
	environment: A::Environment,
}

impl<A> Simulator<A>
where
	A: Agent + Debug,
{
	pub fn new(
		agent: A,
		environment: A::Environment,
	) -> Self {
		Self {
			agent,
			environment,
		}
	}

	pub fn simulate(&mut self) -> Result<(), A::Error> {
		eprintln!("Starting acting with agent: {:?}", self.agent);

		self.environment.render();

		while !self.agent.should_stop() {
			self.agent.act(&mut self.environment)?;
			self.environment.render();
		}

		eprintln!("Solution found:");
		self.environment.render();

		Ok(())
	}
}

#[cfg(test)]
mod tests {

	use crate::{
		agent::Agent,
		maze::{
			agent::{greedy::GreedyFinder, PathFinder},
			coordinates::Coordinates,
			Maze,
		},
		simulator::Simulator,
	};

	#[test]
	fn test_maze_creation() {
		let maze = Maze::<20>::new(Coordinates::new(0, 0), Coordinates::new(19, 19), 5);

		let mut simulator = Simulator::new(PathFinder::<20, GreedyFinder>::new(&maze), maze);

		let _ = simulator.simulate();
	}
}
