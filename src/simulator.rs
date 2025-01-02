use std::fmt::Debug;

use crate::agent::Agent;
use crate::render::Renderer;

pub struct Simulator<A, R>
where
    A: Agent,
    R: Renderer<A::Environment>,
{
    agent: A,
    renderer: R,
    environment: A::Environment,
}

impl<A, R> Simulator<A, R>
where
    A: Agent + Debug,
    R: Renderer<A::Environment>,
{
    pub fn new(agent: A, renderer: R, environment: A::Environment) -> Self {
        Self {
            environment,
            agent,
            renderer,
        }
    }

    pub fn simulate(&mut self) -> Result<(), A::Error> {
        self.renderer.setup();

        eprintln!("Starting acting with agent: {:?}", self.agent);
        self.renderer.render(&self.environment);

        while !self.agent.should_stop() {
            self.agent.act(&mut self.environment)?;
            self.renderer.render(&self.environment);
        }

        eprintln!("Solution found:");
        self.renderer.render(&self.environment);

        self.renderer.teardown();
        Ok(())
    }
}
