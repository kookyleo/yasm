use crate::DEFAULT_MAX_HISTORY_SIZE;
use crate::callbacks::CallbackRegistry;
use crate::core::StateMachine;
use std::collections::VecDeque;

/// State machine instance that can execute state transitions
///
/// The state machine instance maintains the current state, transition history,
/// and provides state transition operations. History is implemented using a ring buffer
/// for automatic memory management. It also supports callbacks for state transitions.
#[derive(Debug)]
pub struct StateMachineInstance<SM: StateMachine> {
    /// Current state
    current_state: SM::State,
    /// Transition history: sequence of (from_state, input) pairs
    history: VecDeque<(SM::State, SM::Input)>,
    /// Maximum history size
    max_history_size: usize,
    /// Callback registry for state machine events
    callback_registry: CallbackRegistry<SM>,
}

impl<SM: StateMachine> StateMachineInstance<SM> {
    /// Create a new state machine instance with default history size
    pub fn new() -> Self {
        Self {
            current_state: SM::initial_state(),
            history: VecDeque::new(),
            max_history_size: DEFAULT_MAX_HISTORY_SIZE,
            callback_registry: CallbackRegistry::new(),
        }
    }

    /// Create a new state machine instance with custom history size
    pub fn with_max_history(max_size: usize) -> Self {
        Self {
            current_state: SM::initial_state(),
            history: VecDeque::with_capacity(max_size),
            max_history_size: max_size,
            callback_registry: CallbackRegistry::new(),
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
                let old_state = self.current_state.clone();

                // Trigger state exit callbacks (only if changing state)
                if old_state != new_state {
                    self.callback_registry.trigger_state_exit(&old_state);
                }

                // Trigger transition callbacks
                self.callback_registry.trigger_transition(&old_state, &input, &new_state);

                // Record transition history
                self.history.push_back((old_state, input));

                // Maintain history size limit using efficient ring buffer operations
                if self.history.len() > self.max_history_size {
                    self.history.pop_front();
                }

                // Update current state
                self.current_state = new_state.clone();

                // Trigger state entry callbacks (only if changing state)
                if self.current_state != self.history.back().unwrap().0 {
                    self.callback_registry.trigger_state_entry(&new_state);
                }

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

    /// Get a mutable reference to the callback registry
    ///
    /// This allows registration and management of callbacks for state machine events.
    pub fn callback_registry(&mut self) -> &mut CallbackRegistry<SM> {
        &mut self.callback_registry
    }

    /// Get a read-only reference to the callback registry
    pub fn callback_registry_ref(&self) -> &CallbackRegistry<SM> {
        &self.callback_registry
    }

    // Convenience methods for callback registration - more intuitive API

    /// Register a callback for when entering a specific state
    ///
    /// # Arguments
    /// * `state` - The state to monitor for entry
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_state_entry(State::Active, |state| {
    ///     println!("Entered active state: {:?}", state);
    /// });
    /// ```
    pub fn on_state_entry<F>(&mut self, state: SM::State, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_state_entry(state, callback);
    }

    /// Register a callback for when exiting a specific state
    ///
    /// # Arguments
    /// * `state` - The state to monitor for exit
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_state_exit(State::Active, |state| {
    ///     println!("Exiting active state: {:?}", state);
    /// });
    /// ```
    pub fn on_state_exit<F>(&mut self, state: SM::State, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_state_exit(state, callback);
    }

    /// Register a callback for a specific transition
    ///
    /// # Arguments
    /// * `from_state` - The source state
    /// * `input` - The input that triggers the transition
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_transition(State::Draft, Input::Submit, |from, input, to| {
    ///     println!("Transition: {:?} --{:?}--> {:?}", from, input, to);
    /// });
    /// ```
    pub fn on_transition<F>(&mut self, from_state: SM::State, input: SM::Input, callback: F)
    where
        F: Fn(&SM::State, &SM::Input, &SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_transition(from_state, input, callback);
    }

    /// Register a global callback that triggers on any state entry
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_any_state_entry(|state| {
    ///     println!("Entered state: {:?}", state);
    /// });
    /// ```
    pub fn on_any_state_entry<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_any_state_entry(callback);
    }

    /// Register a global callback that triggers on any state exit
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_any_state_exit(|state| {
    ///     println!("Exiting state: {:?}", state);
    /// });
    /// ```
    pub fn on_any_state_exit<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_any_state_exit(callback);
    }

    /// Register a global callback that triggers on any transition
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    ///
    /// # Example
    /// ```ignore
    /// workflow.on_any_transition(|from, input, to| {
    ///     println!("Transition: {:?} --{:?}--> {:?}", from, input, to);
    /// });
    /// ```
    pub fn on_any_transition<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State, &SM::Input, &SM::State) + Send + Sync + 'static,
    {
        self.callback_registry.on_any_transition(callback);
    }

    /// Clear all registered callbacks
    ///
    /// # Example
    /// ```ignore
    /// workflow.clear_callbacks();
    /// ```
    pub fn clear_callbacks(&mut self) {
        self.callback_registry.clear();
    }

    /// Get the total number of registered callbacks
    ///
    /// # Returns
    /// The total count of all registered callbacks
    ///
    /// # Example
    /// ```ignore
    /// let count = workflow.callback_count();
    /// println!("Total callbacks: {}", count);
    /// ```
    pub fn callback_count(&self) -> usize {
        self.callback_registry.callback_count()
    }
}

impl<SM: StateMachine> Default for StateMachineInstance<SM> {
    fn default() -> Self {
        Self::new()
    }
}
