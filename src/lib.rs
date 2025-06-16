use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::hash::Hash;

/// State machine definition trait
pub trait StateMachine {
    type State: Clone + Debug + Hash + Eq;
    type Input: Clone + Debug + Hash + Eq;

    /// Get all possible states
    fn states() -> Vec<Self::State>;

    /// Get all possible inputs
    fn inputs() -> Vec<Self::Input>;

    /// Get valid inputs for a given state
    fn valid_inputs(state: &Self::State) -> Vec<Self::Input>;

    /// Get possible next states from current state with given input
    fn next_states(state: &Self::State, input: &Self::Input) -> Vec<Self::State>;

    /// Get the initial state
    fn initial_state() -> Self::State;

    /// Get state name for display
    fn state_name(state: &Self::State) -> String;

    /// Get input name for display
    fn input_name(input: &Self::Input) -> String;
}

/// State machine instance that can execute transitions
#[derive(Debug, Clone)]
pub struct StateMachineInstance<SM: StateMachine> {
    current_state: SM::State,
    history: VecDeque<(SM::State, SM::Input)>,
    max_history_size: usize,
}

/// Default maximum history size
const DEFAULT_MAX_HISTORY_SIZE: usize = 512;

impl<SM: StateMachine> StateMachineInstance<SM> {
    /// Create a new state machine instance with default history size
    pub fn new() -> Self {
        Self {
            current_state: SM::initial_state(),
            history: VecDeque::new(),
            max_history_size: DEFAULT_MAX_HISTORY_SIZE,
        }
    }

    /// Create a new state machine instance with custom history size
    pub fn with_max_history(max_size: usize) -> Self {
        Self {
            current_state: SM::initial_state(),
            history: VecDeque::with_capacity(max_size),
            max_history_size: max_size,
        }
    }

    /// Get maximum history size
    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }

    /// Get current state
    pub fn current_state(&self) -> &SM::State {
        &self.current_state
    }

    /// Get transition history
    pub fn history(&self) -> &VecDeque<(SM::State, SM::Input)> {
        &self.history
    }

    /// Check if input is valid for current state
    pub fn can_accept(&self, input: &SM::Input) -> bool {
        SM::valid_inputs(&self.current_state).contains(input)
    }

    /// Get all valid inputs for current state
    pub fn valid_inputs(&self) -> Vec<SM::Input> {
        SM::valid_inputs(&self.current_state)
    }

    /// Execute a transition
    pub fn transition(&mut self, input: SM::Input) -> Result<Vec<SM::State>, String> {
        if !self.can_accept(&input) {
            return Err(format!(
                "Invalid input {:?} for state {:?}",
                input, self.current_state
            ));
        }

        let next_states = SM::next_states(&self.current_state, &input);
        if next_states.is_empty() {
            return Err(format!(
                "No valid transitions from state {:?} with input {:?}",
                self.current_state, input
            ));
        }

        // Record transition in history
        self.history.push_back((self.current_state.clone(), input));

        // Maintain history size limit using efficient ring buffer operations
        if self.history.len() > self.max_history_size {
            self.history.pop_front();
        }

        // For MVP, we take the first valid next state
        // In future versions, this could support non-deterministic transitions
        self.current_state = next_states[0].clone();

        Ok(next_states)
    }
}

impl<SM: StateMachine> Default for StateMachineInstance<SM> {
    fn default() -> Self {
        Self::new()
    }
}

/// Query utilities for state machine
pub struct StateMachineQuery<SM: StateMachine> {
    _phantom: std::marker::PhantomData<SM>,
}

impl<SM: StateMachine> StateMachineQuery<SM> {
    /// Get all states that can reach the target state
    pub fn states_leading_to(target: &SM::State) -> Vec<SM::State> {
        let mut result = Vec::new();

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                if SM::next_states(&state, &input).contains(target) {
                    result.push(state.clone());
                    break;
                }
            }
        }

        result
    }

    /// Get all reachable states from a given state
    pub fn reachable_states(from: &SM::State) -> Vec<SM::State> {
        let mut reachable = HashSet::new();
        let mut to_visit = vec![from.clone()];

        while let Some(current) = to_visit.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current.clone());

            for input in SM::valid_inputs(&current) {
                for next_state in SM::next_states(&current, &input) {
                    if !reachable.contains(&next_state) {
                        to_visit.push(next_state);
                    }
                }
            }
        }

        reachable.into_iter().collect()
    }

    /// Check if there's a path from one state to another
    pub fn has_path(from: &SM::State, to: &SM::State) -> bool {
        Self::reachable_states(from).contains(to)
    }
}

/// Documentation generator for state machine
pub struct StateMachineDoc<SM: StateMachine> {
    _phantom: std::marker::PhantomData<SM>,
}

impl<SM: StateMachine> StateMachineDoc<SM> {
    /// Check if an input should be included in documentation
    fn should_include_input(input: &SM::Input) -> bool {
        !SM::input_name(input).starts_with('_')
    }

    /// Generate Mermaid state diagram
    pub fn generate_mermaid() -> String {
        let mut mermaid = String::from("stateDiagram-v2\n");

        // Add initial state marker
        let initial = SM::initial_state();
        mermaid.push_str(&format!("    [*] --> {}\n", SM::state_name(&initial)));

        // Separate normal transitions from self-loops
        let mut normal_transitions = HashMap::new();
        let mut self_loops = HashMap::new();

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                // Skip inputs that start with underscore
                if !Self::should_include_input(&input) {
                    continue;
                }

                for next_state in SM::next_states(&state, &input) {
                    if state == next_state {
                        // Self-loop
                        self_loops
                            .entry(state.clone())
                            .or_insert_with(Vec::new)
                            .push(input.clone());
                    } else {
                        // Normal transition
                        let key = (state.clone(), next_state.clone());
                        normal_transitions
                            .entry(key)
                            .or_insert_with(Vec::new)
                            .push(input.clone());
                    }
                }
            }
        }

        // Add normal transitions first
        for ((from, to), inputs) in normal_transitions {
            let input_labels: Vec<String> = inputs.iter().map(|i| SM::input_name(i)).collect();
            let label = input_labels.join(" / ");

            mermaid.push_str(&format!(
                "    {} --> {} : {}\n",
                SM::state_name(&from),
                SM::state_name(&to),
                label
            ));
        }

        // Add self-loops separately with better formatting
        for (state, inputs) in self_loops {
            // Group self-loop inputs - if there are many, show them on separate lines
            if inputs.len() <= 2 {
                let input_labels: Vec<String> = inputs.iter().map(|i| SM::input_name(i)).collect();
                let label = input_labels.join(" / ");
                mermaid.push_str(&format!(
                    "    {} --> {} : {}\n",
                    SM::state_name(&state),
                    SM::state_name(&state),
                    label
                ));
            } else {
                // For many self-loop inputs, add them individually to avoid overcrowding
                for input in inputs {
                    mermaid.push_str(&format!(
                        "    {} --> {} : {}\n",
                        SM::state_name(&state),
                        SM::state_name(&state),
                        SM::input_name(&input)
                    ));
                }
            }
        }

        mermaid
    }

    /// Generate state transition table
    pub fn generate_transition_table() -> String {
        let mut table = String::from("# State Transition Table\n\n");
        table.push_str("| Current State | Input | Next State(s) |\n");
        table.push_str("|---------------|-------|---------------|\n");

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                // Skip inputs that start with underscore
                if !Self::should_include_input(&input) {
                    continue;
                }

                let next_states = SM::next_states(&state, &input);
                let next_state_names: Vec<String> =
                    next_states.iter().map(|s| SM::state_name(s)).collect();

                table.push_str(&format!(
                    "| {} | {} | {} |\n",
                    SM::state_name(&state),
                    SM::input_name(&input),
                    next_state_names.join(", ")
                ));
            }
        }

        table
    }
}

/// Macro to help define state machines
#[macro_export]
macro_rules! define_state_machine {
    (
        name: $name:ident,
        states: { $($state:ident),* $(,)? },
        inputs: { $($input:ident),* $(,)? },
        initial: $initial:ident,
        transitions: {
            $(
                $from:ident + $inp:ident => $to:ident
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub enum State {
            $($state),*
        }

        #[derive(Debug, Clone, Hash, PartialEq, Eq)]
        pub enum Input {
            $($input),*
        }

        pub struct $name;

        impl $crate::StateMachine for $name {
            type State = State;
            type Input = Input;

            fn states() -> Vec<Self::State> {
                vec![$(State::$state),*]
            }

            fn inputs() -> Vec<Self::Input> {
                vec![$(Input::$input),*]
            }

            fn initial_state() -> Self::State {
                State::$initial
            }

            fn state_name(state: &Self::State) -> String {
                format!("{:?}", state)
            }

            fn input_name(input: &Self::Input) -> String {
                format!("{:?}", input)
            }

            fn valid_inputs(state: &Self::State) -> Vec<Self::Input> {
                let mut inputs = Vec::new();
                $(
                    if matches!(state, State::$from) {
                        inputs.push(Input::$inp);
                    }
                )*
                inputs
            }

            fn next_states(state: &Self::State, input: &Self::Input) -> Vec<Self::State> {
                #[allow(unreachable_patterns)]
                match (state, input) {
                    $(
                        (State::$from, Input::$inp) => vec![State::$to],
                    )*
                    _ => vec![],
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Example state machine for testing
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

    // Test state machine with underscore inputs in separate module
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
    fn test_state_machine_basic() {
        let mut sm = StateMachineInstance::<TrafficLight>::new();
        assert_eq!(*sm.current_state(), State::Red);

        // Test valid transition
        let result = sm.transition(Input::Timer);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::Green);

        // Test invalid transition
        let result = sm.transition(Input::Timer);
        assert!(result.is_ok());
        assert_eq!(*sm.current_state(), State::Yellow);
    }

    #[test]
    fn test_query_functions() {
        let reachable = StateMachineQuery::<TrafficLight>::reachable_states(&State::Red);
        assert!(reachable.contains(&State::Green));
        assert!(reachable.contains(&State::Yellow));

        let leading_to_red = StateMachineQuery::<TrafficLight>::states_leading_to(&State::Red);
        assert!(leading_to_red.contains(&State::Yellow));
        assert!(leading_to_red.contains(&State::Green));
    }

    #[test]
    fn test_mermaid_generation() {
        let mermaid = StateMachineDoc::<TrafficLight>::generate_mermaid();
        assert!(mermaid.contains("stateDiagram-v2"));
        assert!(mermaid.contains("Red"));
        assert!(mermaid.contains("Green"));
        assert!(mermaid.contains("Yellow"));
    }

    #[test]
    fn test_history_size_limit() {
        let mut sm = StateMachineInstance::<TrafficLight>::with_max_history(2);
        assert_eq!(sm.max_history_size(), 2);

        // Perform multiple transitions
        sm.transition(Input::Timer).unwrap(); // Red -> Green
        sm.transition(Input::Timer).unwrap(); // Green -> Yellow
        sm.transition(Input::Timer).unwrap(); // Yellow -> Red

        // History should only contain the last 2 transitions
        assert_eq!(sm.history().len(), 2);
        assert_eq!(sm.history()[0], (State::Green, Input::Timer));
        assert_eq!(sm.history()[1], (State::Yellow, Input::Timer));
    }

    #[test]
    fn test_history_size_dynamic_change() {
        // Test default history size
        let mut sm = StateMachineInstance::<TrafficLight>::new();
        assert_eq!(sm.max_history_size(), 512); // DEFAULT_MAX_HISTORY_SIZE

        // Perform multiple transitions
        sm.transition(Input::Timer).unwrap();
        sm.transition(Input::Timer).unwrap();
        sm.transition(Input::Timer).unwrap();
        assert_eq!(sm.history().len(), 3);

        // Test small history size limit
        let mut sm_limited = StateMachineInstance::<TrafficLight>::with_max_history(1);
        assert_eq!(sm_limited.max_history_size(), 1);
        assert_eq!(sm_limited.history().len(), 0); // New instance has empty history

        // Add transitions to limited instance
        sm_limited.transition(Input::Timer).unwrap(); // Red -> Green
        assert_eq!(sm_limited.history().len(), 1);
        sm_limited.transition(Input::Timer).unwrap(); // Green -> Yellow
        assert_eq!(sm_limited.history().len(), 1); // Still 1 due to limit
        assert_eq!(sm_limited.history()[0], (State::Green, Input::Timer)); // Only latest

        let mut sm_with_default_size = StateMachineInstance::<TrafficLight>::new();
        sm_with_default_size.transition(Input::Emergency).unwrap();
        sm_with_default_size.transition(Input::Timer).unwrap();
        sm_with_default_size.transition(Input::Emergency).unwrap();
        assert_eq!(sm_with_default_size.history().len(), 3);
    }

    #[test]
    fn test_underscore_inputs_excluded_from_docs() {
        let mermaid = StateMachineDoc::<test_machine::TestMachine>::generate_mermaid();

        // Should contain normal actions
        assert!(mermaid.contains("Action"));

        // Should NOT contain underscore-prefixed actions
        assert!(!mermaid.contains("_HiddenAction"));
        assert!(!mermaid.contains("_Debug"));

        let table = StateMachineDoc::<test_machine::TestMachine>::generate_transition_table();

        // Should contain normal actions
        assert!(table.contains("Action"));

        // Should NOT contain underscore-prefixed actions
        assert!(!table.contains("_HiddenAction"));
        assert!(!table.contains("_Debug"));
    }

    #[test]
    fn test_underscore_inputs_still_functional() {
        use test_machine::{Input, State, TestMachine};

        let mut sm = StateMachineInstance::<TestMachine>::new();
        assert_eq!(*sm.current_state(), State::StateA);

        // Test that underscore inputs are still valid for transitions
        let valid_inputs = sm.valid_inputs();
        assert!(valid_inputs.contains(&Input::Action));
        assert!(valid_inputs.contains(&Input::_HiddenAction));
        assert!(valid_inputs.contains(&Input::_Debug));

        // Test transitions with underscore inputs work
        sm.transition(Input::_HiddenAction).unwrap();
        assert_eq!(*sm.current_state(), State::StateA);

        sm.transition(Input::_Debug).unwrap();
        assert_eq!(*sm.current_state(), State::StateA);

        // Test normal transition
        sm.transition(Input::Action).unwrap();
        assert_eq!(*sm.current_state(), State::StateB);
    }
}
