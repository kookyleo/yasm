use crate::core::StateMachine;
use std::collections::HashMap;

/// State machine documentation generator
///
/// Provides functionality to generate Mermaid diagrams and transition tables.
pub struct StateMachineDoc<SM: StateMachine> {
    _phantom: std::marker::PhantomData<SM>,
}

impl<SM: StateMachine> StateMachineDoc<SM> {
    /// Check if an input should be included in documentation
    ///
    /// Inputs starting with underscore are typically used for internal debugging
    /// or special purposes and should not be included in user documentation.
    fn should_include_input(input: &SM::Input) -> bool {
        !SM::input_name(input).starts_with('_')
    }

    /// Generate Mermaid state diagram
    ///
    /// Generates a state diagram definition compliant with Mermaid syntax,
    /// which can be used to visualize the state machine structure.
    /// Self-loops and normal transitions are handled separately for better readability.
    ///
    /// # Returns
    /// Returns a Mermaid-formatted state diagram string
    pub fn generate_mermaid() -> String {
        let mut mermaid = String::from("stateDiagram-v2\n");

        // Add initial state marker
        let initial = SM::initial_state();
        mermaid.push_str(&format!("    [*] --> {}\n", SM::state_name(&initial)));

        // Collect normal transitions and self-loops separately
        let mut normal_transitions = HashMap::new();
        let mut self_loops = HashMap::new();

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                // Skip inputs starting with underscore
                if !Self::should_include_input(&input) {
                    continue;
                }

                if let Some(next_state) = SM::next_state(&state, &input) {
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

        // Add normal transitions
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

        // Add self-loops with different formats based on input count
        for (state, inputs) in self_loops {
            if inputs.len() <= 2 {
                // Merge few inputs for display
                let input_labels: Vec<String> = inputs.iter().map(|i| SM::input_name(i)).collect();
                let label = input_labels.join(" / ");
                mermaid.push_str(&format!(
                    "    {} --> {} : {}\n",
                    SM::state_name(&state),
                    SM::state_name(&state),
                    label
                ));
            } else {
                // Display many inputs separately to avoid cluttered diagrams
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
    ///
    /// Generates a Markdown-formatted state transition table listing all valid state transitions.
    ///
    /// # Returns
    /// Returns a Markdown-formatted transition table string
    pub fn generate_transition_table() -> String {
        let mut table = String::from("# State Transition Table\n\n");
        table.push_str("| Current State | Input | Next State |\n");
        table.push_str("|---------------|-------|------------|\n");

        for state in SM::states() {
            for input in SM::valid_inputs(&state) {
                // Skip inputs starting with underscore
                if !Self::should_include_input(&input) {
                    continue;
                }

                if let Some(next_state) = SM::next_state(&state, &input) {
                    table.push_str(&format!(
                        "| {} | {} | {} |\n",
                        SM::state_name(&state),
                        SM::input_name(&input),
                        SM::state_name(&next_state)
                    ));
                }
            }
        }

        table
    }

    /// Generate state machine statistics
    ///
    /// Generates a report containing statistics such as state count, transition count, etc.
    ///
    /// # Returns
    /// Returns a statistics information string
    pub fn generate_statistics() -> String {
        let states = SM::states();
        let inputs = SM::inputs();

        let mut transition_count = 0;
        let mut self_loop_count = 0;

        for state in &states {
            for input in SM::valid_inputs(state) {
                if let Some(next_state) = SM::next_state(state, &input) {
                    if *state == next_state {
                        self_loop_count += 1;
                    } else {
                        transition_count += 1;
                    }
                }
            }
        }

        format!(
            "# State Machine Statistics\n\n\
            - **Number of States**: {}\n\
            - **Number of Input Types**: {}\n\
            - **Number of Transitions**: {}\n\
            - **Number of Self-loops**: {}\n\
            - **Total Transitions**: {}\n\
            - **Initial State**: {}\n",
            states.len(),
            inputs.len(),
            transition_count,
            self_loop_count,
            transition_count + self_loop_count,
            SM::state_name(&SM::initial_state())
        )
    }

    /// Generate complete documentation
    ///
    /// Complete documentation containing statistics, transition tables, and Mermaid diagrams.
    ///
    /// # Returns
    /// Returns the complete documentation string
    pub fn generate_full_documentation() -> String {
        let mut doc = String::new();

        // Add title
        doc.push_str("# State Machine Documentation\n\n");

        // Add statistics
        doc.push_str(&Self::generate_statistics());
        doc.push('\n');

        // Add transition table
        doc.push_str(&Self::generate_transition_table());
        doc.push('\n');

        // Add Mermaid diagram
        doc.push_str("# State Diagram\n\n");
        doc.push_str("```mermaid\n");
        doc.push_str(&Self::generate_mermaid());
        doc.push_str("```\n");

        doc
    }
}
