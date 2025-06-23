//! # YASM (Yet Another State Machine)
//!
//! A modern, efficient deterministic state machine library designed for Rust 2024 edition.
//!
//! ## Features
//!
//! - **Deterministic State Machine**: Each state+input combination has at most one possible next state
//! - **Type Safety**: Leverage Rust's type system to ensure state machine correctness
//! - **Macro Support**: Use declarative macros to quickly define state machines
//! - **History Tracking**: Automatically maintain state transition history for debugging and analysis
//! - **Query Functions**: Rich state machine analysis capabilities
//! - **Documentation Generation**: Automatically generate Mermaid diagrams and transition tables
//! - **Serde Support**: Optional serialization and deserialization support
//!
//! ## Basic Usage
//!
//! ```rust
//! use yasm::*;
//!
//! // Define state machine
//! define_state_machine! {
//!     name: TrafficLight,
//!     states: { Red, Yellow, Green },
//!     inputs: { Timer, Emergency },
//!     initial: Red,
//!     transitions: {
//!         Red + Timer => Green,
//!         Green + Timer => Yellow,
//!         Yellow + Timer => Red,
//!         Red + Emergency => Yellow,
//!         Green + Emergency => Red,
//!         Yellow + Emergency => Red
//!     }
//! }
//!
//! // Create state machine instance
//! let mut traffic_light = StateMachineInstance::<TrafficLight>::new();
//!
//! // Execute state transition
//! traffic_light.transition(Input::Timer).unwrap();
//! assert_eq!(*traffic_light.current_state(), State::Green);
//! ```
//!
//! ## Module Structure
//!
//! - [`core`][]: Core trait and type definitions
//! - [`instance`][]: State machine instance implementation
//! - [`query`][]: State machine query and analysis functionality
//! - [`doc`][]: Documentation generation functionality
//! - [`macros`][]: Macro definitions

// Module declarations
pub mod core;
pub mod doc;
pub mod instance;
pub mod macros;
pub mod query;

// Re-export public interface
pub use core::StateMachine;
pub use doc::StateMachineDoc;
pub use instance::StateMachineInstance;
pub use query::StateMachineQuery;

/// Default maximum history size
pub const DEFAULT_MAX_HISTORY_SIZE: usize = 512;

#[cfg(test)]
mod tests {
    use super::*;

    // Test traffic light state machine
    define_state_machine! {
        name: TrafficLight,
        states: { Red, Yellow, Green },
        inputs: { Timer, Emergency },
        initial: Red,
        transitions: {
            Red + Timer => Green,
            Green + Timer => Yellow,
            Yellow + Timer => Red,
            Red + Emergency => Yellow,
            Green + Emergency => Red,
            Yellow + Emergency => Red
        }
    }

    // Test state machine with hidden inputs
    mod test_machine {
        use super::super::*;

        define_state_machine! {
            name: TestMachine,
            states: { StateA, StateB },
            inputs: { Action, _HiddenAction, _Debug },
            initial: StateA,
            transitions: {
                StateA + Action => StateB,
                StateB + Action => StateA,
                StateA + _HiddenAction => StateA,
                StateB + _HiddenAction => StateB,
                StateA + _Debug => StateA,
                StateB + _Debug => StateB
            }
        }
    }

    #[test]
    fn test_deterministic_state_machine_basic() {
        let mut sm = StateMachineInstance::<TrafficLight>::new();
        assert_eq!(*sm.current_state(), State::Red);

        // Test valid transition
        let result = sm.transition(Input::Timer);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::Green);
        assert_eq!(result.unwrap(), State::Green);

        // Continue transition
        let result = sm.transition(Input::Timer);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::Yellow);
    }

    #[test]
    fn test_invalid_transition() {
        let mut sm = StateMachineInstance::<TrafficLight>::new();
        assert_eq!(*sm.current_state(), State::Red);

        // Test emergency transition - Red can transition to Yellow via Emergency input
        let result = sm.transition(Input::Emergency);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::Yellow);
    }

    #[test]
    fn test_state_machine_instance_methods() {
        let mut sm = StateMachineInstance::<TrafficLight>::new();

        // Test initial state
        assert_eq!(*sm.current_state(), State::Red);
        assert!(sm.history_is_empty());
        assert_eq!(sm.history_len(), 0);

        // Test valid input checking
        assert!(sm.can_accept(&Input::Timer));
        assert!(sm.can_accept(&Input::Emergency));

        // Test valid input list
        let valid_inputs = sm.valid_inputs();
        assert!(valid_inputs.contains(&Input::Timer));
        assert!(valid_inputs.contains(&Input::Emergency));

        // Execute transition
        sm.transition(Input::Timer).unwrap();
        assert_eq!(sm.history_len(), 1);
        assert!(!sm.history_is_empty());

        // Test reset
        sm.reset();
        assert_eq!(*sm.current_state(), State::Red);
        assert!(sm.history_is_empty());
    }

    #[test]
    fn test_query_functions() {
        let reachable = StateMachineQuery::<TrafficLight>::reachable_states(&State::Red);
        assert!(reachable.contains(&State::Green));
        assert!(reachable.contains(&State::Yellow));

        let leading_to_red = StateMachineQuery::<TrafficLight>::states_leading_to(&State::Red);
        assert!(leading_to_red.contains(&State::Yellow));
        assert!(leading_to_red.contains(&State::Green));

        // Test path finding
        assert!(StateMachineQuery::<TrafficLight>::has_path(
            &State::Red,
            &State::Green
        ));

        // Test shortest path
        let path = StateMachineQuery::<TrafficLight>::shortest_path(&State::Red, &State::Green);
        assert!(path.is_some());
        let path = path.unwrap();
        assert_eq!(path[0], State::Red);
        assert_eq!(path[1], State::Green);
    }

    #[test]
    fn test_mermaid_generation() {
        let mermaid = StateMachineDoc::<TrafficLight>::generate_mermaid();
        assert!(mermaid.contains("stateDiagram-v2"));
        assert!(mermaid.contains("Red"));
        assert!(mermaid.contains("Green"));
        assert!(mermaid.contains("Yellow"));
        assert!(mermaid.contains("Timer"));
        assert!(mermaid.contains("Emergency"));
    }

    #[test]
    fn test_history_size_limit() {
        let mut sm = StateMachineInstance::<TrafficLight>::with_max_history(2);
        assert_eq!(sm.max_history_size(), 2);

        // Execute multiple transitions
        sm.transition(Input::Timer).unwrap(); // Red -> Green
        sm.transition(Input::Timer).unwrap(); // Green -> Yellow
        sm.transition(Input::Timer).unwrap(); // Yellow -> Red

        // History should only contain the last 2 transitions
        assert_eq!(sm.history().len(), 2);
        assert_eq!(sm.history()[0], (State::Green, Input::Timer));
        assert_eq!(sm.history()[1], (State::Yellow, Input::Timer));
    }

    #[test]
    fn test_history_size_default() {
        let sm = StateMachineInstance::<TrafficLight>::new();
        assert_eq!(sm.max_history_size(), DEFAULT_MAX_HISTORY_SIZE);

        let sm_default = StateMachineInstance::<TrafficLight>::default();
        assert_eq!(sm_default.max_history_size(), DEFAULT_MAX_HISTORY_SIZE);
    }

    #[test]
    fn test_underscore_inputs_excluded_from_docs() {
        let mermaid = StateMachineDoc::<test_machine::TestMachine>::generate_mermaid();

        // Should contain normal actions
        assert!(mermaid.contains("Action"));

        // Should not contain underscore-prefixed actions
        assert!(!mermaid.contains("_HiddenAction"));
        assert!(!mermaid.contains("_Debug"));

        let table = StateMachineDoc::<test_machine::TestMachine>::generate_transition_table();

        // Should contain normal actions
        assert!(table.contains("Action"));

        // Should not contain underscore-prefixed actions
        assert!(!table.contains("_HiddenAction"));
        assert!(!table.contains("_Debug"));
    }

    #[test]
    fn test_underscore_inputs_still_functional() {
        use test_machine::{Input, State, TestMachine};

        let mut sm = StateMachineInstance::<TestMachine>::new();
        assert_eq!(*sm.current_state(), State::StateA);

        // Test that underscore inputs are still valid
        let valid_inputs = sm.valid_inputs();
        assert!(valid_inputs.contains(&Input::Action));
        assert!(valid_inputs.contains(&Input::_HiddenAction));
        assert!(valid_inputs.contains(&Input::_Debug));

        // Test underscore input transition functionality
        let result = sm.transition(Input::_HiddenAction);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::StateA);

        let result = sm.transition(Input::_Debug);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::StateA);

        // Test normal transition
        let result = sm.transition(Input::Action);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::StateB);
    }

    #[test]
    fn test_display_implementation() {
        assert_eq!(State::Red.to_string(), "Red");
        assert_eq!(Input::Timer.to_string(), "Timer");
    }

    #[test]
    fn test_documentation_generation() {
        let stats = StateMachineDoc::<TrafficLight>::generate_statistics();
        assert!(stats.contains("Number of States"));
        assert!(stats.contains("Number of Transitions"));

        let full_doc = StateMachineDoc::<TrafficLight>::generate_full_documentation();
        assert!(full_doc.contains("State Machine Documentation"));
        assert!(full_doc.contains("State Transition Table"));
        assert!(full_doc.contains("State Diagram"));
    }

    #[test]
    fn test_state_from_str() {
        // Test valid state strings
        let red_state = State::from("Red");
        assert_eq!(red_state, State::Red);

        let yellow_state = State::from("Yellow");
        assert_eq!(yellow_state, State::Yellow);

        let green_state = State::from("Green");
        assert_eq!(green_state, State::Green);
    }

    #[test]
    #[should_panic(expected = "Invalid state")]
    fn test_state_from_str_invalid() {
        // Test invalid state string - should panic
        let _ = State::from("InvalidState");
    }

    #[test]
    fn test_input_from_str() {
        // Test valid input strings
        let timer_input = Input::from("Timer");
        assert_eq!(timer_input, Input::Timer);

        let emergency_input = Input::from("Emergency");
        assert_eq!(emergency_input, Input::Emergency);
    }

    #[test]
    #[should_panic(expected = "Invalid input")]
    fn test_input_from_str_invalid() {
        // Test invalid input string - should panic
        let _ = Input::from("InvalidInput");
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_serialization() {
        // Test state serialization
        let state = State::Red;
        let serialized = serde_json::to_string(&state).unwrap();
        assert_eq!(serialized, "\"Red\"");

        let deserialized: State = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, State::Red);

        // Test input serialization
        let input = Input::Timer;
        let serialized = serde_json::to_string(&input).unwrap();
        assert_eq!(serialized, "\"Timer\"");

        let deserialized: Input = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, Input::Timer);

        // Test multiple states
        let states = vec![State::Red, State::Yellow, State::Green];
        let serialized = serde_json::to_string(&states).unwrap();
        let deserialized: Vec<State> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, states);
    }
}
