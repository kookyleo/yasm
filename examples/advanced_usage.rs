use yasm::*;

// Network connection state machine
mod network {
    use yasm::*;

    define_state_machine! {
        name: NetworkConnection,
        states: { Disconnected, Connecting, Connected, Reconnecting, Failed },
        inputs: { Connect, Disconnect, Timeout, Success, Retry },
        initial: Disconnected,
        transitions: {
            Disconnected + Connect => Connecting,
            Connecting + Success => Connected,
            Connecting + Timeout => Failed,
            Connected + Disconnect => Disconnected,
            Connected + Timeout => Reconnecting,
            Reconnecting + Success => Connected,
            Reconnecting + Timeout => Failed,
            Failed + Retry => Connecting
        }
    }
}

// Game character state machine
mod game_character {
    use yasm::*;

    define_state_machine! {
        name: CharacterState,
        states: { Idle, Walking, Running, Jumping, Attacking, Dead },
        inputs: { StartWalk, StartRun, Jump, Attack, Stop, Die, Respawn },
        initial: Idle,
        transitions: {
            Idle + StartWalk => Walking,
            Idle + StartRun => Running,
            Idle + Jump => Jumping,
            Idle + Attack => Attacking,
            Walking + StartRun => Running,
            Walking + Stop => Idle,
            Walking + Jump => Jumping,
            Walking + Attack => Attacking,
            Running + Stop => Idle,
            Running + Jump => Jumping,
            Running + Attack => Attacking,
            Jumping + Stop => Idle,
            Attacking + Stop => Idle,
            Idle + Die => Dead,
            Walking + Die => Dead,
            Running + Die => Dead,
            Jumping + Die => Dead,
            Attacking + Die => Dead,
            Dead + Respawn => Idle
        }
    }
}

fn main() {
    println!("=== Advanced State Machine Usage Examples ===\n");

    // Network connection state machine demo
    demo_network_connection();

    println!("\n{}\n", "=".repeat(60));

    // Game character state machine demo
    demo_game_character();

    println!("\n{}\n", "=".repeat(60));

    // State machine analysis
    analyze_state_machines();
}

fn demo_network_connection() {
    println!("🌐 Network Connection State Machine Demo");
    println!("{}", "-".repeat(40));

    let mut connection = StateMachineInstance::<network::NetworkConnection>::new();

    // Simulate connection process
    println!("Initial state: {:?}", connection.current_state());

    println!("\nStarting connection...");
    connection.transition(network::Input::Connect).unwrap();
    println!("State: {:?}", connection.current_state());

    println!("\nConnection successful!");
    connection.transition(network::Input::Success).unwrap();
    println!("State: {:?}", connection.current_state());

    println!("\nNetwork timeout, starting reconnection...");
    connection.transition(network::Input::Timeout).unwrap();
    println!("State: {:?}", connection.current_state());

    println!("\nReconnection successful!");
    connection.transition(network::Input::Success).unwrap();
    println!("State: {:?}", connection.current_state());

    println!("\nManually disconnecting");
    connection.transition(network::Input::Disconnect).unwrap();
    println!("State: {:?}", connection.current_state());

    println!("\nConnection history: {:?}", connection.history());
}

fn demo_game_character() {
    println!("🎮 Game Character State Machine Demo");
    println!("{}", "-".repeat(36));

    let mut character = StateMachineInstance::<game_character::CharacterState>::new();

    println!("Character initial state: {:?}", character.current_state());

    // Simulate game action sequence
    let actions = vec![
        (game_character::Input::StartWalk, "Start walking"),
        (game_character::Input::StartRun, "Start running"),
        (game_character::Input::Jump, "Jump"),
        (game_character::Input::Stop, "Stop"),
        (game_character::Input::Attack, "Attack"),
        (game_character::Input::Die, "Die"),
        (game_character::Input::Respawn, "Respawn"),
    ];

    for (input, description) in actions {
        println!("\n{}: {:?} -> ", description, character.current_state());
        match character.transition(input) {
            Ok(_) => {
                println!("✅ {:?}", character.current_state());
                println!("   Available actions: {:?}", character.valid_inputs());
            }
            Err(e) => println!("❌ {}", e),
        }
    }
}

fn analyze_state_machines() {
    println!("📊 State Machine Analysis");
    println!("{}", "-".repeat(26));

    // Analyze network connection state machine
    println!("Network connection state machine analysis:");
    println!(
        "- Total states: {}",
        network::NetworkConnection::states().len()
    );
    println!(
        "- Total inputs: {}",
        network::NetworkConnection::inputs().len()
    );

    // Analyze state connectivity
    let all_states = network::NetworkConnection::states();
    for state in &all_states {
        let reachable = StateMachineQuery::<network::NetworkConnection>::reachable_states(state);
        println!("- From {:?} can reach {} states", state, reachable.len());
    }

    // Find terminal states (states with no outgoing edges)
    println!("\nTerminal state analysis:");
    for state in &all_states {
        let valid_inputs = network::NetworkConnection::valid_inputs(state);
        if valid_inputs.is_empty() {
            println!("- {:?} is a terminal state", state);
        } else {
            println!("- {:?} has {} available inputs", state, valid_inputs.len());
        }
    }

    println!("\nGame character state machine Mermaid diagram:");
    println!(
        "{}",
        StateMachineDoc::<game_character::CharacterState>::generate_mermaid()
    );
}
