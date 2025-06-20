use crate::DEFAULT_MAX_HISTORY_SIZE;
use crate::core::StateMachine;
use std::collections::VecDeque;

/// State machine instance that can execute state transitions
///
/// The state machine instance maintains the current state, transition history,
/// and provides state transition operations. History is implemented using a ring buffer
/// for automatic memory management.
#[derive(Debug, Clone)]
pub struct StateMachineInstance<SM: StateMachine> {
    /// Current state
    current_state: SM::State,
    /// Transition history: sequence of (from_state, input) pairs
    history: VecDeque<(SM::State, SM::Input)>,
    /// Maximum history size
    max_history_size: usize,
}

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

    /// Get the maximum history size
    pub fn max_history_size(&self) -> usize {
        self.max_history_size
    }

    /// Get a read-only reference to the current state
    pub fn current_state(&self) -> &SM::State {
        &self.current_state
    }

    /// Get a read-only reference to the transition history
    pub fn history(&self) -> &VecDeque<(SM::State, SM::Input)> {
        &self.history
    }

    /// Check if the given input is valid for the current state
    pub fn can_accept(&self, input: &SM::Input) -> bool {
        SM::valid_inputs(&self.current_state).contains(input)
    }

    /// Get all valid inputs for the current state
    pub fn valid_inputs(&self) -> Vec<SM::Input> {
        SM::valid_inputs(&self.current_state)
    }

    /// Execute a state transition
    ///
    /// If the transition succeeds, returns the new state; if the input is invalid
    /// or the transition fails, returns an error message.
    ///
    /// # Arguments
    /// - `input`: The input that triggers the transition
    ///
    /// # Returns
    /// - `Ok(new_state)`: Transition succeeded, returns the new state
    /// - `Err(error_message)`: Transition failed, returns an error message
    pub fn transition(&mut self, input: SM::Input) -> Result<SM::State, String> {
        // Check if the input is valid for the current state
        if !self.can_accept(&input) {
            return Err(format!(
                "Invalid input {:?} for state {:?}",
                input, self.current_state
            ));
        }

        // Execute deterministic transition
        let next_state = SM::next_state(&self.current_state, &input);
        match next_state {
            Some(new_state) => {
                // Record transition history
                self.history.push_back((self.current_state.clone(), input));

                // Maintain history size limit using efficient ring buffer operations
                if self.history.len() > self.max_history_size {
                    self.history.pop_front();
                }

                // Update current state
                self.current_state = new_state.clone();

                Ok(new_state)
            }
            None => Err(format!(
                "No valid transition from state {:?} with input {:?}",
                self.current_state, input
            )),
        }
    }

    /// Reset the state machine to its initial state and clear history
    pub fn reset(&mut self) {
        self.current_state = SM::initial_state();
        self.history.clear();
    }

    /// Get the length of the history
    pub fn history_len(&self) -> usize {
        self.history.len()
    }

    /// Check if the history is empty
    pub fn history_is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

impl<SM: StateMachine> Default for StateMachineInstance<SM> {
    fn default() -> Self {
        Self::new()
    }
}
