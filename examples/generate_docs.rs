use std::fs;
use yasm::*;

// Reuse previously defined state machines
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

mod server {
    use yasm::*;

    define_state_machine! {
        name: ServerStateMachine,
        states: { Pending, Active, Sunsetting, Maintenance, Terminated },
        inputs: { Activate, Deactivate, Maintain, Terminate, _EditDesc, Rollback, MaintenanceSuccess },
        initial: Pending,
        transitions: {
            Pending + Activate => Active,
            Active + Deactivate => Sunsetting,
            Sunsetting + Maintain => Maintenance,
            Sunsetting + Terminate => Terminated,
            Sunsetting + Rollback => Pending,
            Maintenance + Terminate => Terminated,
            Maintenance + MaintenanceSuccess => Pending,
            Pending + _EditDesc => Pending,
            Active + _EditDesc => Active,
            Sunsetting + _EditDesc => Sunsetting,
            Maintenance + _EditDesc => Maintenance,
            Terminated + _EditDesc => Terminated,
        }
    }
}

fn main() -> std::io::Result<()> {
    println!("📚 Generating state machine documentation...\n");

    // Create output directory
    fs::create_dir_all("examples/docs")?;

    // Generate door state machine documentation
    generate_door_docs()?;

    // Generate order state machine documentation
    generate_order_docs()?;

    // Generate server state machine documentation
    generate_server_docs()?;

    println!("✅ Documentation generation complete! Check the docs/ directory");
    println!("\nGenerated files:");
    println!("- examples/docs/door_state_machine.md");
    println!("- examples/docs/order_state_machine.md");
    println!("- examples/docs/door_state_machine.mermaid");
    println!("- examples/docs/order_state_machine.mermaid");
    println!("- examples/docs/server_state_machine.md");
    println!("- examples/docs/server_state_machine.mermaid");

    Ok(())
}

fn generate_door_docs() -> std::io::Result<()> {
    println!("🚪 Generating door state machine documentation...");

    // Generate Mermaid diagram
    let mermaid = StateMachineDoc::<door::DoorStateMachine>::generate_mermaid();
    fs::write("examples/docs/door_state_machine.mermaid", &mermaid)?;

    // Generate complete Markdown documentation
    let mut doc = String::new();
    doc.push_str("# Door State Machine\n\n");
    doc.push_str("This is a simple door state machine that demonstrates basic door operations: opening, closing, and locking.\n\n");

    doc.push_str("## State Diagram\n\n");
    doc.push_str("```mermaid\n");
    doc.push_str(&mermaid);
    doc.push_str("```\n\n");

    doc.push_str("## State Descriptions\n\n");
    doc.push_str("- **Closed**: Door is closed, can be opened or locked\n");
    doc.push_str("- **Open**: Door is open, can only be closed\n");
    doc.push_str("- **Locked**: Door is locked, can only be unlocked\n\n");

    doc.push_str("## Input Descriptions\n\n");
    doc.push_str("- **OpenDoor**: Open door operation\n");
    doc.push_str("- **CloseDoor**: Close door operation\n");
    doc.push_str("- **Lock**: Lock door operation\n");
    doc.push_str("- **Unlock**: Unlock door operation\n\n");

    doc.push_str(&StateMachineDoc::<door::DoorStateMachine>::generate_transition_table());

    // Add usage examples
    doc.push_str("\n## Usage Example\n\n");
    doc.push_str("```rust\n");
    doc.push_str("use yasm::*;\n\n");
    doc.push_str("let mut door = StateMachineInstance::<door::DoorStateMachine>::new();\n");
    doc.push_str("assert_eq!(*door.current_state(), door::State::Closed);\n\n");
    doc.push_str("// Open door\n");
    doc.push_str("door.transition(door::Input::OpenDoor).unwrap();\n");
    doc.push_str("assert_eq!(*door.current_state(), door::State::Open);\n\n");
    doc.push_str("// Close door\n");
    doc.push_str("door.transition(door::Input::CloseDoor).unwrap();\n");
    doc.push_str("assert_eq!(*door.current_state(), door::State::Closed);\n\n");
    doc.push_str("// Lock door\n");
    doc.push_str("door.transition(door::Input::Lock).unwrap();\n");
    doc.push_str("assert_eq!(*door.current_state(), door::State::Locked);\n");
    doc.push_str("```\n");

    fs::write("examples/docs/door_state_machine.md", doc)?;

    Ok(())
}

fn generate_order_docs() -> std::io::Result<()> {
    println!("📦 Generating order state machine documentation...");

    // Generate Mermaid diagram
    let mermaid = StateMachineDoc::<order::OrderStateMachine>::generate_mermaid();
    fs::write("examples/docs/order_state_machine.mermaid", &mermaid)?;

    // Generate complete Markdown documentation
    let mut doc = String::new();
    doc.push_str("# Order Processing State Machine\n\n");
    doc.push_str("This is an order processing state machine that demonstrates the complete lifecycle of an e-commerce order.\n\n");

    doc.push_str("## State Diagram\n\n");
    doc.push_str("```mermaid\n");
    doc.push_str(&mermaid);
    doc.push_str("```\n\n");

    doc.push_str("## State Descriptions\n\n");
    doc.push_str("- **Created**: Order has been created, waiting for payment\n");
    doc.push_str("- **Paid**: Order has been paid, waiting for shipment\n");
    doc.push_str("- **Shipped**: Order has been shipped, in transit\n");
    doc.push_str("- **Delivered**: Order has been delivered, transaction complete\n");
    doc.push_str("- **Cancelled**: Order has been cancelled\n\n");

    doc.push_str("## Input Descriptions\n\n");
    doc.push_str("- **Pay**: Pay for the order\n");
    doc.push_str("- **Ship**: Ship the order\n");
    doc.push_str("- **Deliver**: Confirm delivery\n");
    doc.push_str("- **Cancel**: Cancel the order\n");
    doc.push_str("- **Refund**: Request a refund\n\n");

    doc.push_str(&StateMachineDoc::<order::OrderStateMachine>::generate_transition_table());

    // Add business process descriptions
    doc.push_str("\n## Business Processes\n\n");
    doc.push_str("### Normal Flow\n");
    doc.push_str("1. Order created (Created)\n");
    doc.push_str("2. User pays (Pay) → Paid\n");
    doc.push_str("3. Merchant ships (Ship) → Shipped\n");
    doc.push_str("4. Delivery confirmed (Deliver) → Delivered\n\n");

    doc.push_str("### Cancellation Flow\n");
    doc.push_str("- Direct cancellation after creation: Created → (Cancel) → Cancelled\n");
    doc.push_str("- Refund after payment: Paid → (Refund) → Cancelled\n");
    doc.push_str("- Cancellation after shipping: Shipped → (Cancel) → Cancelled\n\n");

    doc.push_str("## Usage Example\n\n");
    doc.push_str("```rust\n");
    doc.push_str("use yasm::*;\n\n");
    doc.push_str("let mut order = StateMachineInstance::<order::OrderStateMachine>::new();\n\n");
    doc.push_str("// Normal order flow\n");
    doc.push_str("order.transition(order::Input::Pay).unwrap();\n");
    doc.push_str("order.transition(order::Input::Ship).unwrap();\n");
    doc.push_str("order.transition(order::Input::Deliver).unwrap();\n");
    doc.push_str("assert_eq!(*order.current_state(), order::State::Delivered);\n");
    doc.push_str("```\n");

    fs::write("examples/docs/order_state_machine.md", doc)?;

    Ok(())
}

fn generate_server_docs() -> std::io::Result<()> {
    println!("👷 Generating server state machine documentation...");

    // Generate Mermaid diagram
    let mermaid = StateMachineDoc::<server::ServerStateMachine>::generate_mermaid();
    fs::write("examples/docs/server_state_machine.mermaid", &mermaid)?;

    // Generate complete Markdown documentation
    let mut doc = String::new();
    doc.push_str("# Server State Machine\n\n");
    doc.push_str("This is a comprehensive server state machine that manages the lifecycle of servers (such as workers, services, or infrastructure components).\n\n");

    doc.push_str("## State Diagram\n\n");
    doc.push_str("```mermaid\n");
    doc.push_str(&mermaid);
    doc.push_str("```\n\n");

    doc.push_str("## State Descriptions\n\n");
    doc.push_str("- **Pending**: Server is created and waiting to be activated\n");
    doc.push_str("- **Active**: Server is running and serving requests\n");
    doc.push_str("- **Sunsetting**: Server is being phased out, no new requests accepted\n");
    doc.push_str("- **Maintenance**: Server is under maintenance, temporarily unavailable\n");
    doc.push_str("- **Terminated**: Server has been permanently shut down\n\n");

    doc.push_str("## Input Descriptions\n\n");
    doc.push_str("- **Activate**: Start the resource and make it available\n");
    doc.push_str("- **Deactivate**: Begin the sunsetting process\n");
    doc.push_str("- **Maintain**: Put the resource into maintenance mode\n");
    doc.push_str("- **Terminate**: Permanently shut down the resource\n");
    doc.push_str(
        "- **EditDesc**: Add or modify notes about the resource (available in all states)\n",
    );

    doc.push_str(&StateMachineDoc::<server::ServerStateMachine>::generate_transition_table());

    // Add operational workflows
    doc.push_str("\n## Operational Workflows\n\n");
    doc.push_str("### Normal Lifecycle\n");
    doc.push_str("1. Server created (Pending)\n");
    doc.push_str("2. Server activated (Activate) → Active\n");
    doc.push_str("3. Server deactivated (Deactivate) → Sunsetting\n");
    doc.push_str("4. Server maintenance (Maintain) → Maintenance\n");
    doc.push_str("5. Server terminated (Terminate) → Terminated\n\n");

    doc.push_str("### Maintenance Workflow\n");
    doc.push_str("- From Sunsetting: (Maintain) → Maintenance\n");
    doc.push_str("- From Maintenance: (Terminate) → Terminated\n\n");

    doc.push_str("### Monitoring Operations\n");
    doc.push_str("- EditDesc operations are available in all states\n");
    doc.push_str("- These operations don't change the server state\n");
    doc.push_str("- Useful for operational monitoring and documentation\n\n");

    doc.push_str("## Usage Example\n\n");
    doc.push_str("```rust\n");
    doc.push_str("use yasm::*;\n\n");
    doc.push_str("let mut server = StateMachineInstance::<server::ServerStateMachine>::new();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Pending);\n\n");
    doc.push_str("// Activate server\n");
    doc.push_str("server.transition(server::Input::Activate).unwrap();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Active);\n\n");
    doc.push_str("// Add notes while active\n");
    doc.push_str("server.transition(server::Input::EditDesc).unwrap();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Active);\n\n");
    doc.push_str("// Begin sunsetting\n");
    doc.push_str("server.transition(server::Input::Deactivate).unwrap();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Sunsetting);\n\n");
    doc.push_str("// Enter maintenance mode\n");
    doc.push_str("server.transition(server::Input::Maintain).unwrap();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Maintenance);\n\n");
    doc.push_str("// Maintenance successful\n");
    doc.push_str("server.transition(server::Input::MaintenanceSuccess).unwrap();\n");
    doc.push_str("assert_eq!(*server.current_state(), server::State::Pending);\n");
    doc.push_str("```\n");

    fs::write("examples/docs/server_state_machine.md", doc)?;

    Ok(())
}
