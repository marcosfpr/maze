use std::fmt::Debug;

use crate::agent::Agent;
use crate::environment::Environment;

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
    pub fn new(agent: A, environment: A::Environment) -> Self {
        Self { environment, agent }
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
