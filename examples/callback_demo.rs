use yasm::*;
use std::sync::{Arc, Mutex};

// Define a simple workflow state machine for demonstration
mod workflow {
    use yasm::*;

    define_state_machine! {
        name: WorkflowStateMachine,
        states: { Draft, Review, Approved, Published, Archived },
        inputs: { Submit, Approve, Reject, Publish, Archive, Edit },
        initial: Draft,
        transitions: {
            Draft + Submit => Review,
            Review + Approve => Approved,
            Review + Reject => Draft,
            Approved + Publish => Published,
            Published + Archive => Archived,
            Draft + Edit => Draft,
            Archived + Edit => Draft
        }
    }
}

fn main() {
    println!("=== YASM Callback System Demo ===\n");

    demo_state_callbacks();
    println!("\n{}\n", "=".repeat(50));

    demo_transition_callbacks();
    println!("\n{}\n", "=".repeat(50));

    demo_global_callbacks();
}

fn demo_state_callbacks() {
    println!("🎯 State Entry/Exit Callbacks Demo");
    println!("{}", "-".repeat(34));

    let mut workflow = StateMachineInstance::<workflow::WorkflowStateMachine>::new();
    
    // Shared counter for tracking callback executions
    let entry_counter = Arc::new(Mutex::new(0));
    let exit_counter = Arc::new(Mutex::new(0));

    // Register state entry callbacks
    let entry_counter_clone = Arc::clone(&entry_counter);
    workflow.on_state_entry(
        workflow::State::Review,
        move |state| {
            println!("📥 Entering Review state: {state:?}");
            println!("   - Notifying reviewers...");
            println!("   - Setting up review deadline...");
            *entry_counter_clone.lock().unwrap() += 1;
        }
    );

    let entry_counter_clone = Arc::clone(&entry_counter);
    workflow.on_state_entry(
        workflow::State::Published,
        move |state| {
            println!("🚀 Entering Published state: {state:?}");
            println!("   - Sending publication notifications...");
            println!("   - Updating search index...");
            *entry_counter_clone.lock().unwrap() += 1;
        }
    );

    // Register state exit callbacks
    let exit_counter_clone = Arc::clone(&exit_counter);
    workflow.on_state_exit(
        workflow::State::Draft,
        move |state| {
            println!("📤 Exiting Draft state: {state:?}");
            println!("   - Saving draft changes...");
            println!("   - Cleaning up temporary files...");
            *exit_counter_clone.lock().unwrap() += 1;
        }
    );

    // Execute workflow transitions
    println!("\nInitial state: {:?}", workflow.current_state());

    println!("\n=== Submitting for review ===");
    workflow.transition(workflow::Input::Submit).unwrap();
    println!("Current state: {:?}", workflow.current_state());

    println!("\n=== Approving submission ===");
    workflow.transition(workflow::Input::Approve).unwrap();
    println!("Current state: {:?}", workflow.current_state());

    println!("\n=== Publishing content ===");
    workflow.transition(workflow::Input::Publish).unwrap();
    println!("Current state: {:?}", workflow.current_state());

    println!("\n=== Callback execution summary ===");
    println!("Entry callbacks triggered: {}", *entry_counter.lock().unwrap());
    println!("Exit callbacks triggered: {}", *exit_counter.lock().unwrap());
}

fn demo_transition_callbacks() {
    println!("🔄 Transition Callbacks Demo");
    println!("{}", "-".repeat(29));

    let mut workflow = StateMachineInstance::<workflow::WorkflowStateMachine>::new();
    
    // Register transition-specific callbacks
    workflow.on_transition(
        workflow::State::Draft,
        workflow::Input::Submit,
        |from, input, to| {
            println!("🔄 Transition: {from:?} --{input:?}--> {to:?}");
            println!("   - Validating submission requirements...");
            println!("   - Creating review task...");
            println!("   - Assigning to review queue...");
        }
    );

    workflow.on_transition(
        workflow::State::Review,
        workflow::Input::Approve,
        |from, input, to| {
            println!("🔄 Transition: {from:?} --{input:?}--> {to:?}");
            println!("   - Recording approval timestamp...");
            println!("   - Updating approval metrics...");
            println!("   - Preparing for publication...");
        }
    );

    workflow.on_transition(
        workflow::State::Review,
        workflow::Input::Reject,
        |from, input, to| {
            println!("🔄 Transition: {from:?} --{input:?}--> {to:?}");
            println!("   - Logging rejection reason...");
            println!("   - Notifying author of feedback...");
            println!("   - Resetting to draft mode...");
        }
    );

    // Execute workflow with transitions
    println!("\nInitial state: {:?}", workflow.current_state());

    println!("\n=== Submitting for review ===");
    workflow.transition(workflow::Input::Submit).unwrap();

    println!("\n=== Rejecting submission ===");
    workflow.transition(workflow::Input::Reject).unwrap();

    println!("\n=== Resubmitting for review ===");
    workflow.transition(workflow::Input::Submit).unwrap();

    println!("\n=== Approving submission ===");
    workflow.transition(workflow::Input::Approve).unwrap();

    println!("\nFinal state: {:?}", workflow.current_state());
}

fn demo_global_callbacks() {
    println!("🌐 Global Callbacks Demo");
    println!("{}", "-".repeat(25));

    let mut workflow = StateMachineInstance::<workflow::WorkflowStateMachine>::new();
    
    // Shared counters for tracking global events
    let transition_count = Arc::new(Mutex::new(0));
    let state_change_count = Arc::new(Mutex::new(0));

    // Register global callbacks
    let transition_count_clone = Arc::clone(&transition_count);
    workflow.on_any_transition(move |from, input, to| {
        let count = {
            let mut c = transition_count_clone.lock().unwrap();
            *c += 1;
            *c
        };
        println!("🌐 Global transition #{count}: {from:?} --{input:?}--> {to:?}");
    });

    let state_change_count_clone = Arc::clone(&state_change_count);
    workflow.on_any_state_entry(move |state| {
        let count = {
            let mut c = state_change_count_clone.lock().unwrap();
            *c += 1;
            *c
        };
        println!("🌐 Global state entry #{count}: {state:?}");
    });

    let _state_change_count_clone = Arc::clone(&state_change_count);
    workflow.on_any_state_exit(move |state| {
        println!("🌐 Global state exit: {state:?}");
    });

    // Execute multiple transitions
    println!("\nExecuting workflow transitions with global monitoring:");
    println!("Initial state: {:?}", workflow.current_state());

    let transitions = vec![
        (workflow::Input::Submit, "Submit for review"),
        (workflow::Input::Approve, "Approve submission"),
        (workflow::Input::Publish, "Publish content"),
        (workflow::Input::Archive, "Archive content"),
    ];

    for (input, description) in transitions {
        println!("\n=== {description} ===");
        match workflow.transition(input) {
            Ok(new_state) => println!("✅ Transitioned to: {new_state:?}"),
            Err(e) => println!("❌ Transition failed: {e}"),
        }
    }

    println!("\n=== Global callback summary ===");
    println!("Total transitions monitored: {}", *transition_count.lock().unwrap());
    println!("Total state entries monitored: {}", *state_change_count.lock().unwrap());
    
    println!("\nCallback registry stats:");
    println!("Total registered callbacks: {}", workflow.callback_count());
} 