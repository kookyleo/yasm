use crate::core::StateMachine;
use std::collections::HashSet;

/// State machine query utilities
///
/// Provides various state machine analysis capabilities such as reachability analysis
/// and path finding.
pub struct StateMachineQuery<SM: StateMachine> {
    _phantom: std::marker::PhantomData<SM>,
}

impl<SM: StateMachine> StateMachineQuery<SM> {
    /// Get all states that can reach the target state
    ///
    /// Traverses all state and input combinations to find all states that can
    /// reach the target state through a single transition.
    ///
    /// # Arguments
    /// - `target`: The target state
    ///
    /// # Returns
    /// Returns a list of all states that can reach the target state
    #[allow(clippy::collapsible_if)]
    pub fn states_leading_to(target: &SM::State) -> Vec<SM::State> {
        let mut result = Vec::new();

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                if let Some(next_state) = SM::next_state(&state, &input) {
                    if next_state == *target {
                        result.push(state.clone());
                        break; // Found one transition, avoid duplicate additions
                    }
                }
            }
        }

        result
    }

    /// Get all states reachable from a given state
    ///
    /// Uses depth-first search algorithm to recursively find all reachable states.
    ///
    /// # Arguments
    /// - `from`: The starting state
    ///
    /// # Returns
    /// Returns a list of all states reachable from the starting state (including the starting state itself)
    #[allow(clippy::collapsible_if)]
    pub fn reachable_states(from: &SM::State) -> Vec<SM::State> {
        let mut reachable = HashSet::new();
        let mut to_visit = vec![from.clone()];

        while let Some(current) = to_visit.pop() {
            if reachable.contains(&current) {
                continue;
            }
            reachable.insert(current.clone());

            // Explore all possible next states
            for input in SM::valid_inputs(&current) {
                if let Some(next_state) = SM::next_state(&current, &input) {
                    if !reachable.contains(&next_state) {
                        to_visit.push(next_state);
                    }
                }
            }
        }

        reachable.into_iter().collect()
    }

    /// Check if a path exists from one state to another
    ///
    /// # Arguments
    /// - `from`: The starting state
    /// - `to`: The target state
    ///
    /// # Returns
    /// Returns true if a path exists, otherwise false
    pub fn has_path(from: &SM::State, to: &SM::State) -> bool {
        Self::reachable_states(from).contains(to)
    }

    /// Get all terminal states in the state machine (states with no outgoing edges)
    ///
    /// Terminal states are states that have no valid inputs that can trigger transitions.
    ///
    /// # Returns
    /// Returns a list of all terminal states
    pub fn terminal_states() -> Vec<SM::State> {
        let mut terminal_states = Vec::new();

        for state in SM::states() {
            if SM::valid_inputs(&state).is_empty() {
                terminal_states.push(state);
            }
        }

        terminal_states
    }

    /// Check if the state machine is strongly connected
    ///
    /// Strong connectivity means that from any state, you can reach any other state.
    ///
    /// # Returns
    /// Returns true if the state machine is strongly connected, otherwise false
    pub fn is_strongly_connected() -> bool {
        let states = SM::states();
        if states.is_empty() {
            return true;
        }

        // Check if all other states are reachable from the first state
        let reachable_from_first = Self::reachable_states(&states[0]);
        if reachable_from_first.len() != states.len() {
            return false;
        }

        // Check if the first state is reachable from all other states
        for state in &states[1..] {
            if !Self::has_path(state, &states[0]) {
                return false;
            }
        }

        true
    }

    /// Find the shortest path from the starting state to the target state
    ///
    /// Uses breadth-first search algorithm to find the shortest path.
    ///
    /// # Arguments
    /// - `from`: The starting state
    /// - `to`: The target state
    ///
    /// # Returns
    /// Returns the state sequence of the shortest path, or None if unreachable
    #[allow(clippy::collapsible_if)]
    pub fn shortest_path(from: &SM::State, to: &SM::State) -> Option<Vec<SM::State>> {
        use std::collections::{HashMap, VecDeque};

        if from == to {
            return Some(vec![from.clone()]);
        }

        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();

        queue.push_back(from.clone());
        visited.insert(from.clone());

        while let Some(current) = queue.pop_front() {
            for input in SM::valid_inputs(&current) {
                if let Some(next_state) = SM::next_state(&current, &input) {
                    if !visited.contains(&next_state) {
                        visited.insert(next_state.clone());
                        parent.insert(next_state.clone(), current.clone());
                        queue.push_back(next_state.clone());

                        if next_state == *to {
                            // Reconstruct path
                            let mut path = Vec::new();
                            let mut current_state = to.clone();
                            path.push(current_state.clone());

                            while let Some(prev_state) = parent.get(&current_state) {
                                path.push(prev_state.clone());
                                current_state = prev_state.clone();
                            }

                            path.reverse();
                            return Some(path);
                        }
                    }
                }
            }
        }

        None
    }
}
