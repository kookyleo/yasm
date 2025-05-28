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

// Define a more complex order processing state machine
mod order {
    use yasm::*;
    
    define_state_machine! {
        name: OrderStateMachine,
        states: { Created, Paid, Shipped, Delivered, Cancelled },
        inputs: { Pay, Ship, Deliver, Cancel, Refund },
        initial: Created,
        transitions: {
            Created + Pay => Paid,
            Created + Cancel => Cancelled,
            Paid + Ship => Shipped,
            Paid + Refund => Cancelled,
            Shipped + Deliver => Delivered,
            Shipped + Cancel => Cancelled
        }
    }
}

fn main() {
    println!("=== YASM (Yet Another State Machine) Basic Demo ===\n");
    
    // Demonstrate door state machine
    demo_door_state_machine();
    
    println!("\n{}\n", "=".repeat(50));
    
    // Demonstrate order state machine
    demo_order_state_machine();
    
    println!("\n{}\n", "=".repeat(50));
    
    // Demonstrate query functions
    demo_query_functions();
    
    println!("\n{}\n", "=".repeat(50));
    
    // Demonstrate documentation generation
    demo_documentation_generation();
}

fn demo_door_state_machine() {
    println!("🚪 Door State Machine Demo");
    println!("{}", "-".repeat(25));
    
    let mut door_sm = StateMachineInstance::<door::DoorStateMachine>::new();
    println!("Initial state: {:?}", door_sm.current_state());
    
    // Demonstrate state transitions
    println!("\nCurrent valid inputs: {:?}", door_sm.valid_inputs());
    
    println!("\nTrying to open door...");
    match door_sm.transition(door::Input::OpenDoor) {
        Ok(_) => println!("✅ Success! Current state: {:?}", door_sm.current_state()),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    println!("\nCurrent valid inputs: {:?}", door_sm.valid_inputs());
    
    println!("\nTrying to lock door (should fail because door is open)...");
    match door_sm.transition(door::Input::Lock) {
        Ok(_) => println!("✅ Success! Current state: {:?}", door_sm.current_state()),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    println!("\nClosing door...");
    match door_sm.transition(door::Input::CloseDoor) {
        Ok(_) => println!("✅ Success! Current state: {:?}", door_sm.current_state()),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    println!("\nLocking door...");
    match door_sm.transition(door::Input::Lock) {
        Ok(_) => println!("✅ Success! Current state: {:?}", door_sm.current_state()),
        Err(e) => println!("❌ Failed: {}", e),
    }
    
    println!("\nTransition history: {:?}", door_sm.history());
}

fn demo_order_state_machine() {
    println!("📦 Order State Machine Demo");
    println!("{}", "-".repeat(27));
    
    let mut order_sm = StateMachineInstance::<order::OrderStateMachine>::new();
    println!("Initial state: {:?}", order_sm.current_state());
    
    // Normal workflow
    println!("\n=== Normal Order Workflow ===");
    let transitions = vec![
        (order::Input::Pay, "Pay for order"),
        (order::Input::Ship, "Ship order"),
        (order::Input::Deliver, "Deliver order"),
    ];
    
    for (input, description) in transitions {
        println!("\n{}: {:?} -> ", description, order_sm.current_state());
        match order_sm.transition(input) {
            Ok(_) => println!("✅ {:?}", order_sm.current_state()),
            Err(e) => println!("❌ {}", e),
        }
    }
    
    // Demonstrate cancellation workflow
    println!("\n=== Order Cancellation Workflow ===");
    let mut order2 = StateMachineInstance::<order::OrderStateMachine>::new();
    println!("New order state: {:?}", order2.current_state());
    
    println!("\nPaying for order...");
    order2.transition(order::Input::Pay).unwrap();
    println!("State: {:?}", order2.current_state());
    
    println!("\nRequesting refund...");
    match order2.transition(order::Input::Refund) {
        Ok(_) => println!("✅ Refund successful! State: {:?}", order2.current_state()),
        Err(e) => println!("❌ Refund failed: {}", e),
    }
}

fn demo_query_functions() {
    println!("🔍 Query Functions Demo");
    println!("{}", "-".repeat(22));
    
    // Door state machine queries
    println!("Door state machine queries:");
    println!("All states reachable from Closed: {:?}", 
        StateMachineQuery::<door::DoorStateMachine>::reachable_states(&door::State::Closed));
    
    println!("All states that can reach Locked: {:?}", 
        StateMachineQuery::<door::DoorStateMachine>::states_leading_to(&door::State::Locked));
    
    println!("Path exists from Open to Locked: {}", 
        StateMachineQuery::<door::DoorStateMachine>::has_path(&door::State::Open, &door::State::Locked));
    
    // Order state machine queries
    println!("\nOrder state machine queries:");
    println!("All states reachable from Created: {:?}", 
        StateMachineQuery::<order::OrderStateMachine>::reachable_states(&order::State::Created));
    
    println!("All states that can reach Delivered: {:?}", 
        StateMachineQuery::<order::OrderStateMachine>::states_leading_to(&order::State::Delivered));
}

fn demo_documentation_generation() {
    println!("📚 Documentation Generation Demo");
    println!("{}", "-".repeat(32));
    
    println!("Door state machine Mermaid diagram:");
    println!("{}", StateMachineDoc::<door::DoorStateMachine>::generate_mermaid());
    
    println!("\nOrder state machine Mermaid diagram:");
    println!("{}", StateMachineDoc::<order::OrderStateMachine>::generate_mermaid());
    
    println!("\nDoor state machine transition table:");
    println!("{}", StateMachineDoc::<door::DoorStateMachine>::generate_transition_table());
} 