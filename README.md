# YASM (Yet Another State Machine)

[![CI](https://github.com/kookyleo/yasm/workflows/CI/badge.svg)](https://github.com/kookyleo/yasm/actions)
[![codecov](https://codecov.io/github/kookyleo/yasm/graph/badge.svg?token=mtxTMfIqih)](https://codecov.io/github/kookyleo/yasm)
[![Crates.io](https://img.shields.io/crates/v/yasm.svg)](https://crates.io/crates/yasm)
[![Documentation](https://docs.rs/yasm/badge.svg)](https://docs.rs/yasm)
[![License](https://img.shields.io/crates/l/yasm.svg)](https://github.com/kookyleo/yasm#license)

A modern, efficient **deterministic** finite state machine library.

## 🚀 Features

- **⚡ Deterministic State Machine**: Each state+input combination has exactly one possible next state, ensuring predictability and debuggability
- **🎯 Type Safety**: Leverage Rust's type system to prevent invalid state transitions at compile time
- **🔧 Macro-Driven**: Define state machines using clean, declarative macro syntax
- **📊 Visualization**: Automatically generate Mermaid diagrams for state machine visualization
- **🔍 Rich Query API**: Comprehensive state machine analysis including pathfinding, reachability, and connectivity analysis
- **📝 Documentation Generation**: Auto-generate transition tables, statistics, and complete documentation
- **🔒 Hidden Operations**: Support underscore-prefixed inputs that remain functional but don't appear in documentation
- **📈 Memory Efficient**: Ring buffer implementation with configurable history limits (default: 512 entries)
- **🏗️ Modular Architecture**: Clean separation of concerns across core, instance, query, and documentation modules
- **📦 Optional Serde Support**: Serialize and deserialize state machines with the `serde` feature

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yasm = "0.4.1"

# For serialization support
yasm = { version = "0.4.1", features = ["serde"] }
```

## 🎯 Quick Start

### Basic Usage

```rust
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
    // Create state machine instance
    let mut door = StateMachineInstance::<door::DoorStateMachine>::new();
    
    // Check current state
    println!("Current state: {:?}", door.current_state()); // Closed
    
    // Execute transitions
    door.transition(door::Input::OpenDoor).unwrap();
    println!("New state: {:?}", door.current_state()); // Open
    
    // Check valid inputs
    println!("Valid inputs: {:?}", door.valid_inputs()); // [CloseDoor]
}
```

### Advanced Features

#### History Management
```rust
// Create instance with custom history limit
let mut door = StateMachineInstance::<door::DoorStateMachine>::with_max_history(100);

// View transition history (efficient ring buffer)
println!("History: {:?}", door.history());
println!("Max history size: {}", door.max_history_size());
```

#### Hidden Operations
```rust
define_state_machine! {
    name: ServerStateMachine,
    states: { Active, Maintenance },
    inputs: { Maintain, Restore, _Debug, _Log },  // _Debug and _Log are hidden
    initial: Active,
    transitions: {
        Active + Maintain => Maintenance,
        Maintenance + Restore => Active,
        // Hidden operations (won't appear in docs but fully functional)
        Active + _Debug => Active,
        Maintenance + _Debug => Maintenance,
        Active + _Log => Active,
        Maintenance + _Log => Maintenance
    }
}
```

#### Query and Analysis
```rust
// Find reachable states
let reachable = StateMachineQuery::<door::DoorStateMachine>::reachable_states(&door::State::Closed);
println!("Reachable from Closed: {:?}", reachable);

// Find paths between states
let has_path = StateMachineQuery::<door::DoorStateMachine>::has_path(
    &door::State::Open, 
    &door::State::Locked
);
println!("Path exists: {}", has_path);

// Find shortest path
let path = StateMachineQuery::<door::DoorStateMachine>::shortest_path(
    &door::State::Open, 
    &door::State::Locked
);
println!("Shortest path: {:?}", path);
```

#### Documentation Generation
```rust
// Generate Mermaid diagram
let mermaid = StateMachineDoc::<door::DoorStateMachine>::generate_mermaid();
println!("{}", mermaid);

// Generate transition table
let table = StateMachineDoc::<door::DoorStateMachine>::generate_transition_table();
println!("{}", table);

// Generate complete documentation
let docs = StateMachineDoc::<door::DoorStateMachine>::generate_full_documentation();
println!("{}", docs);
```

## 📚 Examples

The project includes comprehensive examples:

### 🎯 Basic Demo
```bash
cargo run --example basic_demo
```
- Door and order state machines
- Basic transitions and queries
- Documentation generation

### 🚀 Advanced Usage
```bash
cargo run --example advanced_usage
```
- Network connection state machine
- Game character state machine
- State machine analysis tools

### 🔧 Feature Demo
```bash
cargo run --example feature_demo
```
- Hidden operations demonstration
- History management features
- Performance optimizations

### 📖 Documentation Generation
```bash
cargo run --example generate_docs
```
- Auto-generate project documentation
- Create Mermaid diagram files
- Generate transition tables

## 🏗️ Architecture

```
yasm/
├── src/
│   ├── lib.rs          # Main library entry point with comprehensive tests
│   ├── core.rs         # StateMachine trait and core type definitions
│   ├── instance.rs     # StateMachineInstance implementation with history management
│   ├── query.rs        # StateMachineQuery analysis and pathfinding algorithms
│   ├── doc.rs          # StateMachineDoc documentation generation utilities
│   └── macros.rs       # define_state_machine! macro implementation
├── examples/           # Comprehensive examples and demos
└── docs/              # Auto-generated documentation
```

## 🔧 API Reference

### Core Components

#### `StateMachine` Trait
Defines the core behavior of deterministic state machines:
- `states()` - Get all possible states
- `inputs()` - Get all possible inputs  
- `next_state()` - Deterministic state transition logic
- `valid_inputs()` - Get valid inputs for a state

#### `StateMachineInstance<SM>`
Runtime state machine instance with history tracking:
- `new()` - Create with default history (512 entries)
- `with_max_history(size)` - Create with custom history limit
- `transition(input)` - Execute state transition
- `current_state()` - Get current state
- `valid_inputs()` - Get valid inputs for current state
- `history()` - Access transition history (ring buffer)

#### `StateMachineQuery<SM>`
State machine analysis utilities:
- `reachable_states(from)` - Find all reachable states
- `states_leading_to(target)` - Find states that can reach target
- `has_path(from, to)` - Check if path exists
- `shortest_path(from, to)` - Find shortest path between states
- `terminal_states()` - Find states with no outgoing transitions
- `is_strongly_connected()` - Check graph connectivity

#### `StateMachineDoc<SM>`
Documentation generation utilities:
- `generate_mermaid()` - Create Mermaid state diagram
- `generate_transition_table()` - Create Markdown transition table
- `generate_statistics()` - Generate state machine statistics
- `generate_full_documentation()` - Complete documentation bundle

## 🎨 Design Principles

1. **Deterministic First**: Every state+input combination maps to exactly one next state
2. **Type Safety**: Compile-time prevention of invalid state transitions
3. **Zero-Cost Abstractions**: Efficient macro-generated code with minimal runtime overhead
4. **Memory Efficiency**: Ring buffer history management with configurable limits
5. **Developer Experience**: Clean APIs, comprehensive documentation, and helpful error messages
6. **Extensibility**: Modular design allows easy extension and customization

## 🚀 Performance Features

- **Ring Buffer History**: Automatic memory management for transition history
- **Compile-Time Generation**: Macro-generated code with optimal performance
- **Minimal Allocations**: Efficient data structures and memory usage
- **Configurable Limits**: Customize memory usage based on your needs

## 🔄 Roadmap

- [ ] Conditional transitions with guards
- [ ] Additional export formats (GraphViz, PlantUML)
- [ ] WebAssembly support

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with features
cargo test --features serde

# Run specific test
cargo test test_deterministic_state_machine_basic
```

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.

## 🤝 Contributing

Contributions welcome! Please read our contributing guidelines and submit pull requests to our repository.

---

Built with ❤️ for the Rust community