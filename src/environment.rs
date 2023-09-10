//! AI Environment
//!
//! An environment is a broader term that encapsulates the setting where an Agent can act.
//!
//! An environment could be, for example, a physical setting where an AI agent with physical sensors and actuators,
//!  called a robot will act.

pub trait Environment {
	/// Error type for environment updates.
	type Error;

	// Actions in a given environment.
	type Action;

	// Stimuli generated after executing one action.
	type Stimuli;

	/// Generates the initial stimuli of a given environment.
	fn initial_stimuli(&self) -> Self::Stimuli;

	/// Produces a signal based on an action.
	fn update(
		&mut self,
		action: Self::Action,
	) -> Result<Self::Stimuli, Self::Error>;
}
