use std::collections::{HashMap, HashSet};
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
    history: Vec<(SM::State, SM::Input)>,
}

impl<SM: StateMachine> StateMachineInstance<SM> {
    /// Create a new state machine instance
    pub fn new() -> Self {
        Self {
            current_state: SM::initial_state(),
            history: Vec::new(),
        }
    }
    
    /// Get current state
    pub fn current_state(&self) -> &SM::State {
        &self.current_state
    }
    
    /// Get transition history
    pub fn history(&self) -> &[(SM::State, SM::Input)] {
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
        
        // For MVP, we take the first valid next state
        // In future versions, this could support non-deterministic transitions
        self.history.push((self.current_state.clone(), input));
        self.current_state = next_states[0].clone();
        
        Ok(next_states)
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
    /// Generate Mermaid state diagram
    pub fn generate_mermaid() -> String {
        let mut mermaid = String::from("stateDiagram-v2\n");
        
        // Add initial state marker
        let initial = SM::initial_state();
        mermaid.push_str(&format!("    [*] --> {}\n", SM::state_name(&initial)));
        
        // Add all transitions
        let mut transitions = HashMap::new();
        
        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                for next_state in SM::next_states(&state, &input) {
                    let key = (state.clone(), next_state.clone());
                    transitions.entry(key).or_insert_with(Vec::new).push(input.clone());
                }
            }
        }
        
        for ((from, to), inputs) in transitions {
            let input_labels: Vec<String> = inputs.iter()
                .map(|i| SM::input_name(i))
                .collect();
            let label = input_labels.join(" / ");
            
            mermaid.push_str(&format!(
                "    {} --> {} : {}\n",
                SM::state_name(&from),
                SM::state_name(&to),
                label
            ));
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
                let next_states = SM::next_states(&state, &input);
                let next_state_names: Vec<String> = next_states.iter()
                    .map(|s| SM::state_name(s))
                    .collect();
                
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
} 