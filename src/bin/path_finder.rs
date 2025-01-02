use std::io::stdout;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use solver::agent::Agent;
use solver::maze::agent::greedy::GreedyFinder;
use solver::maze::agent::PathFinder;
use solver::maze::coordinates::Coordinates;
use solver::maze::render::DefaultRenderer;
use solver::maze::Maze;
use solver::simulator::Simulator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let maze = Maze::<20>::new(Coordinates::new(0, 0), Coordinates::new(19, 19), 5);
    let path_finder = PathFinder::<20, GreedyFinder>::new(&maze);

    let renderer  = DefaultRenderer::new(stdout());

    let mut simulator = Simulator::new(
        path_finder,
        renderer,
        maze,
    );

    let _ = simulator.simulate();


    Ok(())
}
