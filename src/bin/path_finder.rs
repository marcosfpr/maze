use solver::agent::Agent;
use solver::maze::agent::greedy::GreedyFinder;
use solver::maze::agent::PathFinder;
use solver::maze::coordinates::Coordinates;
use solver::maze::Maze;
use solver::simulator::Simulator;

fn main() {
    let maze = Maze::<20>::new(Coordinates::new(0, 0), Coordinates::new(19, 19), 5);

    let mut simulator = Simulator::new(PathFinder::<20, GreedyFinder>::new(&maze), maze);

    let _ = simulator.simulate();
}
