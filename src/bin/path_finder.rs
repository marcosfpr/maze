use std::io::stdout;

use solver::agent::Agent;
use solver::maze::agent::astar::AStarFinder;
use solver::maze::agent::greedy::GreedyFinder;
use solver::maze::agent::PathFinder;
use solver::maze::coordinates::Coordinates;
use solver::maze::render::DefaultRenderer;
use solver::maze::Maze;
use solver::simulator::Simulator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const N: usize = 50;
    
    let density = 10;

    let start = Coordinates::new(0, 0);
    let goal = Coordinates::new(49, 49);

    let maze = Maze::<N>::new(start, goal, density);
    let path_finder = PathFinder::<N, GreedyFinder>::new(&maze);

    let renderer  = DefaultRenderer::new(stdout());

    let mut simulator = Simulator::new(
        path_finder,
        renderer,
        maze,
    );

    let _ = simulator.simulate();


    Ok(())
}
