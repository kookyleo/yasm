use yasm::*;

mod demo_machine {
    use yasm::*;

    define_state_machine! {
        name: DemoStateMachine,
        states: { Active, Paused, Stopped },
        inputs: { Start, Pause, Stop, Resume, _Log, _Debug, _Inspect },
        initial: Stopped,
        transitions: {
            Stopped + Start => Active,
            Active + Pause => Paused,
            Active + Stop => Stopped,
            Paused + Resume => Active,
            Paused + Stop => Stopped,
            // Hidden operations (won't appear in documentation)
            Active + _Log => Active,
            Active + _Debug => Active,
            Active + _Inspect => Active,
            Paused + _Log => Paused,
            Paused + _Debug => Paused,
            Paused + _Inspect => Paused,
            Stopped + _Log => Stopped,
            Stopped + _Debug => Stopped,
            Stopped + _Inspect => Stopped
        }
    }
}

fn main() {
    println!("=== YASM Feature Demo ===\n");

    demo_hidden_operations();
    println!("\n{}\n", "=".repeat(50));

    demo_history_limits();
}

fn demo_hidden_operations() {
    println!("🔧 Hidden Operations Demo");
    println!("{}", "-".repeat(26));

    let mut machine = StateMachineInstance::<demo_machine::DemoStateMachine>::new();
    println!("Initial state: {:?}", machine.current_state());

    // Show all valid inputs (including hidden ones)
    println!("All valid inputs: {:?}", machine.valid_inputs());

    // Start the machine
    machine.transition(demo_machine::Input::Start).unwrap();
    println!("After start: {:?}", machine.current_state());

    // Use hidden operations
    println!("\nUsing hidden operations:");
    machine.transition(demo_machine::Input::_Log).unwrap();
    println!("After _Log: {:?}", machine.current_state());

    machine.transition(demo_machine::Input::_Debug).unwrap();
    println!("After _Debug: {:?}", machine.current_state());

    machine.transition(demo_machine::Input::_Inspect).unwrap();
    println!("After _Inspect: {:?}", machine.current_state());

    // Normal operations
    machine.transition(demo_machine::Input::Pause).unwrap();
    println!("After pause: {:?}", machine.current_state());

    println!("\nTransition history:");
    for (i, (from_state, input)) in machine.history().iter().enumerate() {
        println!("  {}. {:?} --{:?}--> ", i + 1, from_state, input);
    }

    // Generate documentation (hidden operations won't appear)
    println!("\nGenerated Mermaid diagram:");
    let mermaid = StateMachineDoc::<demo_machine::DemoStateMachine>::generate_mermaid();
    println!("{mermaid}");

    println!("Generated transition table:");
    let table = StateMachineDoc::<demo_machine::DemoStateMachine>::generate_transition_table();
    println!("{table}");
}

fn demo_history_limits() {
    println!("📈 History Limits Demo");
    println!("{}", "-".repeat(22));

    // Create a machine with history limit of 3
    let mut limited_machine =
        StateMachineInstance::<demo_machine::DemoStateMachine>::with_max_history(3);

    println!(
        "Created machine with max history size: {:?}",
        limited_machine.max_history_size()
    );

    // Perform many transitions
    let transitions = [
        demo_machine::Input::Start,
        demo_machine::Input::_Log,
        demo_machine::Input::Pause,
        demo_machine::Input::_Debug,
        demo_machine::Input::Resume,
        demo_machine::Input::_Inspect,
        demo_machine::Input::Stop,
        demo_machine::Input::_Log,
    ];

    println!("\nPerforming {} transitions:", transitions.len());
    for (i, input) in transitions.iter().enumerate() {
        let old_state = limited_machine.current_state().clone();
        limited_machine.transition(input.clone()).unwrap();
        println!(
            "  {}. {:?} --{:?}--> {:?}",
            i + 1,
            old_state,
            input,
            limited_machine.current_state()
        );
    }

    println!("\nHistory (limited to 3 entries):");
    for (i, (from_state, input)) in limited_machine.history().iter().enumerate() {
        println!("  {}. {:?} --{:?}-->", i + 1, from_state, input);
    }
    println!("History length: {}", limited_machine.history().len());

    // Demonstrate dynamic history size changes
    println!("\nChanging history limit to 1:");
    limited_machine = StateMachineInstance::<demo_machine::DemoStateMachine>::with_max_history(1);
    println!(
        "History after resize: {} entries",
        limited_machine.history().len()
    );
    for (i, (from_state, input)) in limited_machine.history().iter().enumerate() {
        println!("  {}. {:?} --{:?}-->", i + 1, from_state, input);
    }

    // Add more transitions
    limited_machine
        .transition(demo_machine::Input::Start)
        .unwrap();
    limited_machine
        .transition(demo_machine::Input::Pause)
        .unwrap();
    limited_machine
        .transition(demo_machine::Input::Resume)
        .unwrap();

    println!(
        "History after removing limit: {} entries",
        limited_machine.history().len()
    );
    println!("Max history size: {:?}", limited_machine.max_history_size());
}
