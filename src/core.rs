use std::fmt::Debug;
use std::hash::Hash;

/// Deterministic state machine definition trait
///
/// A deterministic state machine guarantees that for any given state and input combination,
/// there is at most one possible next state. This design simplifies state transition logic
/// and improves predictability and debuggability.
pub trait StateMachine {
    /// State type that must support cloning, debug output, hashing, and equality comparison
    type State: Clone + Debug + Hash + Eq;

    /// Input type that must support cloning, debug output, hashing, and equality comparison
    type Input: Clone + Debug + Hash + Eq;

    /// Get all possible states
    fn states() -> Vec<Self::State>;

    /// Get all possible inputs
    fn inputs() -> Vec<Self::Input>;

    /// Get valid inputs for a given state
    fn valid_inputs(state: &Self::State) -> Vec<Self::Input>;

    /// Deterministic state transition: determine the next state from current state and given input
    ///
    /// Returns Some(next_state) if the transition is valid, otherwise None
    fn next_state(state: &Self::State, input: &Self::Input) -> Option<Self::State>;

    /// Get the initial state
    fn initial_state() -> Self::State;

    /// Get the display name of a state
    fn state_name(state: &Self::State) -> String;

    /// Get the display name of an input
    fn input_name(input: &Self::Input) -> String;
}
