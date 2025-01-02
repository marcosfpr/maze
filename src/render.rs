use crate::environment::Environment;

/// Renders the environment state to the screen.
pub trait Renderer<E: Environment> {
    /// Setup renderer.
    fn setup(&mut self);

    /// Render the environment state.
    fn render(&mut self, environment: &E);

    /// Teardown renderer.
    fn teardown(&mut self);
}
