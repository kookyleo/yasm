use yasm::*;

// Define a simple door state machine
mod door {
    use yasm::*;

    define_state_machine! {
        name: DoorStateMachine,
        states: { Closed, Open, Locked },
        inputs: { OpenDoor, CloseDoor, Lock, Unlock },
        initial: Closed,
        transitions: {
            Closed + OpenDoor => Open,
            Open + CloseDoor => Closed,
            Closed + Lock => Locked,
            Locked + Unlock => Closed
        }
    }
}

fn main() {
    println!("=== Simple Callback API Demo ===\n");

    let mut door = StateMachineInstance::<door::DoorStateMachine>::new();

    // Register state entry callbacks
    door.on_state_entry(door::State::Open, |_state| {
        println!("🚪 Door opened - turning on ventilation system");
    });

    door.on_state_exit(door::State::Open, |_state| {
        println!("🚪 Door closed - turning off ventilation system");
    });

    // Register specific transition callback
    door.on_transition(door::State::Closed, door::Input::Lock, |from, input, to| {
        println!("🔒 Security transition: {from:?} --{input:?}--> {to:?}");
        println!("   Activating security system...");
    });

    // Register global monitoring callback
    door.on_any_transition(|from, input, to| {
        println!("📊 State change recorded: {from:?} → {to:?} (input: {input:?})");
    });

    println!("Initial state: {:?}\n", door.current_state());

    // Test transitions with callbacks
    println!("=== Opening door ===");
    door.transition(door::Input::OpenDoor).unwrap();

    println!("\n=== Closing door ===");
    door.transition(door::Input::CloseDoor).unwrap();

    println!("\n=== Locking door ===");
    door.transition(door::Input::Lock).unwrap();

    println!("\n=== Unlocking door ===");
    door.transition(door::Input::Unlock).unwrap();

    println!("\n✨ Demo completed! Total callbacks registered: {}", door.callback_count());
} 