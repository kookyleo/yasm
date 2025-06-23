use crate::core::StateMachine;
use std::collections::HashMap;

/// Callback function type for state entry
pub type StateEntryCallback<SM> = Box<dyn Fn(&<SM as StateMachine>::State) + Send + Sync>;

/// Callback function type for state exit
pub type StateExitCallback<SM> = Box<dyn Fn(&<SM as StateMachine>::State) + Send + Sync>;

/// Callback function type for transition
pub type TransitionCallback<SM> = Box<
    dyn Fn(
            &<SM as StateMachine>::State,
            &<SM as StateMachine>::Input,
            &<SM as StateMachine>::State,
        ) + Send
        + Sync,
>;

/// Type alias for transition key to reduce complexity
pub type TransitionKey<SM> = (<SM as StateMachine>::State, <SM as StateMachine>::Input);

/// Callback registry for state machine events
///
/// This structure manages callbacks for state machine events including:
/// - State entry callbacks: triggered when entering a state
/// - State exit callbacks: triggered when leaving a state  
/// - Transition callbacks: triggered during state transitions
pub struct CallbackRegistry<SM: StateMachine> {
    /// State entry callbacks mapped by state
    state_entry_callbacks: HashMap<<SM as StateMachine>::State, Vec<StateEntryCallback<SM>>>,
    
    /// State exit callbacks mapped by state
    state_exit_callbacks: HashMap<<SM as StateMachine>::State, Vec<StateExitCallback<SM>>>,
    
    /// Transition callbacks mapped by (from_state, input) pairs
    transition_callbacks: HashMap<TransitionKey<SM>, Vec<TransitionCallback<SM>>>,
    
    /// Global callbacks that trigger on any state entry
    global_entry_callbacks: Vec<StateEntryCallback<SM>>,
    
    /// Global callbacks that trigger on any state exit
    global_exit_callbacks: Vec<StateExitCallback<SM>>,
    
    /// Global callbacks that trigger on any transition
    global_transition_callbacks: Vec<TransitionCallback<SM>>,
}

impl<SM: StateMachine> Default for CallbackRegistry<SM> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SM: StateMachine> CallbackRegistry<SM> {
    /// Create a new callback registry
    pub fn new() -> Self {
        Self {
            state_entry_callbacks: HashMap::new(),
            state_exit_callbacks: HashMap::new(),
            transition_callbacks: HashMap::new(),
            global_entry_callbacks: Vec::new(),
            global_exit_callbacks: Vec::new(),
            global_transition_callbacks: Vec::new(),
        }
    }

    /// Register a callback for when entering a specific state
    ///
    /// # Arguments
    /// * `state` - The state to monitor for entry
    /// * `callback` - The callback function to execute
    pub fn on_state_entry<F>(&mut self, state: SM::State, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.state_entry_callbacks
            .entry(state)
            .or_default()
            .push(Box::new(callback));
    }

    /// Register a callback for when exiting a specific state
    ///
    /// # Arguments
    /// * `state` - The state to monitor for exit
    /// * `callback` - The callback function to execute
    pub fn on_state_exit<F>(&mut self, state: SM::State, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.state_exit_callbacks
            .entry(state)
            .or_default()
            .push(Box::new(callback));
    }

    /// Register a callback for a specific transition
    ///
    /// # Arguments
    /// * `from_state` - The source state
    /// * `input` - The input that triggers the transition
    /// * `callback` - The callback function to execute
    pub fn on_transition<F>(&mut self, from_state: SM::State, input: SM::Input, callback: F)
    where
        F: Fn(&SM::State, &SM::Input, &SM::State) + Send + Sync + 'static,
    {
        self.transition_callbacks
            .entry((from_state, input))
            .or_default()
            .push(Box::new(callback));
    }

    /// Register a global callback that triggers on any state entry
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    pub fn on_any_state_entry<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.global_entry_callbacks.push(Box::new(callback));
    }

    /// Register a global callback that triggers on any state exit
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    pub fn on_any_state_exit<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State) + Send + Sync + 'static,
    {
        self.global_exit_callbacks.push(Box::new(callback));
    }

    /// Register a global callback that triggers on any transition
    ///
    /// # Arguments
    /// * `callback` - The callback function to execute
    pub fn on_any_transition<F>(&mut self, callback: F)
    where
        F: Fn(&SM::State, &SM::Input, &SM::State) + Send + Sync + 'static,
    {
        self.global_transition_callbacks.push(Box::new(callback));
    }

    /// Trigger state entry callbacks
    ///
    /// # Arguments
    /// * `state` - The state being entered
    pub(crate) fn trigger_state_entry(&self, state: &SM::State) {
        // Trigger global entry callbacks
        for callback in &self.global_entry_callbacks {
            callback(state);
        }

        // Trigger state-specific entry callbacks
        if let Some(callbacks) = self.state_entry_callbacks.get(state) {
            for callback in callbacks {
                callback(state);
            }
        }
    }

    /// Trigger state exit callbacks
    ///
    /// # Arguments
    /// * `state` - The state being exited
    pub(crate) fn trigger_state_exit(&self, state: &SM::State) {
        // Trigger global exit callbacks
        for callback in &self.global_exit_callbacks {
            callback(state);
        }

        // Trigger state-specific exit callbacks
        if let Some(callbacks) = self.state_exit_callbacks.get(state) {
            for callback in callbacks {
                callback(state);
            }
        }
    }

    /// Trigger transition callbacks
    ///
    /// # Arguments
    /// * `from_state` - The source state
    /// * `input` - The input that triggered the transition
    /// * `to_state` - The destination state
    pub(crate) fn trigger_transition(
        &self,
        from_state: &SM::State,
        input: &SM::Input,
        to_state: &SM::State,
    ) {
        // Trigger global transition callbacks
        for callback in &self.global_transition_callbacks {
            callback(from_state, input, to_state);
        }

        // Trigger transition-specific callbacks
        let key = (from_state.clone(), input.clone());
        if let Some(callbacks) = self.transition_callbacks.get(&key) {
            for callback in callbacks {
                callback(from_state, input, to_state);
            }
        }
    }

    /// Clear all callbacks
    pub fn clear(&mut self) {
        self.state_entry_callbacks.clear();
        self.state_exit_callbacks.clear();
        self.transition_callbacks.clear();
        self.global_entry_callbacks.clear();
        self.global_exit_callbacks.clear();
        self.global_transition_callbacks.clear();
    }

    /// Get the number of registered callbacks
    pub fn callback_count(&self) -> usize {
        self.state_entry_callbacks.values().map(|v| v.len()).sum::<usize>()
            + self.state_exit_callbacks.values().map(|v| v.len()).sum::<usize>()
            + self.transition_callbacks.values().map(|v| v.len()).sum::<usize>()
            + self.global_entry_callbacks.len()
            + self.global_exit_callbacks.len()
            + self.global_transition_callbacks.len()
    }
}

impl<SM: StateMachine> std::fmt::Debug for CallbackRegistry<SM> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackRegistry")
            .field("callback_count", &self.callback_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use std::sync::{Arc, Mutex};

    define_state_machine! {
        name: TestStateMachine,
        states: { StateA, StateB, StateC },
        inputs: { Input1, Input2 },
        initial: StateA,
        transitions: {
            StateA + Input1 => StateB,
            StateB + Input2 => StateC,
            StateC + Input1 => StateA
        }
    }

    #[test]
    fn test_callback_registry() {
        let mut registry = CallbackRegistry::<TestStateMachine>::new();
        let counter = Arc::new(Mutex::new(0));

        // Register state entry callback
        let counter_clone = Arc::clone(&counter);
        registry.on_state_entry(State::StateB, move |_state| {
            *counter_clone.lock().unwrap() += 1;
        });

        // Trigger entry callback
        registry.trigger_state_entry(&State::StateB);
        assert_eq!(*counter.lock().unwrap(), 1);

        // Register global callback
        let counter_clone = Arc::clone(&counter);
        registry.on_any_state_entry(move |_state| {
            *counter_clone.lock().unwrap() += 10;
        });

        // Trigger entry callback again
        registry.trigger_state_entry(&State::StateB);
        // Expected: 1 (initial) + 1 (StateB callback) + 10 (global callback) = 12
        assert_eq!(*counter.lock().unwrap(), 12);

        assert!(registry.callback_count() > 0);
        assert_eq!(registry.callback_count(), 2); // 1 state-specific + 1 global
    }
} 