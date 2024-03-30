//! AI Agent
//!
//! AI is about practical reasoning: reasoning in order to do something.
//! A coupling of perception, reasoning, and acting comprises an agent.
//! An agent acts in an environment. An agent’s environment often includes other
//! agents. An agent together with its environment is called a world.
//!
//! An agent could be, for example, a coupling of a computational engine with
//! physical sensors and actuators,  called a robot, where the environment is a
//! physical setting. An autonomous agent is one that acts in the  world without
//! human intervention. A semi-autonomous agent acts with a human-in-the-loop
//! who may provide  perceptual information and carry out the task. An agent
//! could be a program that acts in a purely computational  environment, a
//! software agent, often called a bot.

use crate::environment::Environment;

/// An [`Agent`] is something acts in an environment. Typically, they have:
/// - prior knowledge about  the environment
/// - stimuli received from the environment, which can include observations
///   about the environment
///  (e.g., light, sound, keyboard commands, web requests) as well as actions
/// that the environment  imposes on the agent (e.g., bumping the agent)
/// - past experiences, including history of interaction with the environment
///   (its previous actions and stimuli)
/// and other data, from which it can learn
/// - goals that it must try to achieve or preferences over states of the world
/// - abilities, the primitive actions the agent is capable of carrying out.
pub trait Agent {
    /// Error type for action issues.
    type Error;

    /// Action type. Typically is an enumeration of possible actions in a given
    /// environment.
    type Action;

    /// Stimuli type. Typically is an enumeration of possible stimuli sent from
    /// the environment.
    type Stimuli;

    /// Environment type.
    type Environment: Environment<Action = Self::Action, Stimuli = Self::Stimuli>;

    /// Initializes the Agent based on the environment.
    fn new(environment: &Self::Environment) -> Self;

    /// Acts in the environment.
    ///
    /// # Errors
    ///
    /// This function will return an error if the action can't be executed.
    fn act(&mut self, environment: &mut Self::Environment) -> Result<(), Self::Error>;

    /// Checks if the action still was work to do.
    fn should_stop(&self) -> bool;
}
