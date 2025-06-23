# YASM (Yet Another State Machine)

[![CI](https://github.com/kookyleo/yasm/workflows/CI/badge.svg)](https://github.com/kookyleo/yasm/actions)
[![codecov](https://codecov.io/github/kookyleo/yasm/graph/badge.svg?token=mtxTMfIqih)](https://codecov.io/github/kookyleo/yasm)
[![Crates.io](https://img.shields.io/crates/v/yasm.svg)](https://crates.io/crates/yasm)
[![Documentation](https://docs.rs/yasm/badge.svg)](https://docs.rs/yasm)
[![License](https://img.shields.io/crates/l/yasm.svg)](https://github.com/kookyleo/yasm#license)

A modern, efficient **deterministic** finite state machine library for Rust 2024 edition.

## ✨ Key Features

- **⚡ Deterministic**: One state + input = one next state (guaranteed)
- **🔒 Type Safe**: Compile-time prevention of invalid transitions
- **🚀 Callback System**: React to state changes with flexible event hooks
- **🔧 Macro-Driven**: Clean declarative syntax for state machine definition
- **📊 Visualization**: Auto-generate Mermaid diagrams and documentation
- **🔍 Analysis Tools**: Pathfinding, reachability, and connectivity analysis
- **📈 Memory Efficient**: Ring buffer with configurable history (default: 512)
- **📦 Optional Serde**: Serialization support with `serde` feature

## 📦 Installation

```toml
[dependencies]
yasm = "0.4.1"

# With serialization support
yasm = { version = "0.4.1", features = ["serde"] }
```

## 🚀 Quick Start

### Define a State Machine

```rust
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
```

### Basic Usage

```rust
fn main() {
    let mut door = StateMachineInstance::<DoorStateMachine>::new();
    
    println!("Current: {:?}", door.current_state()); // Closed
    
    door.transition(Input::OpenDoor).unwrap();
    println!("After opening: {:?}", door.current_state()); // Open
    
    // Check what's possible next
    println!("Valid inputs: {:?}", door.valid_inputs()); // [CloseDoor]
}
```

## 🔥 Core Features

### 1. Callback System

React to state machine events with flexible callback hooks:

```rust
let mut door = StateMachineInstance::<DoorStateMachine>::new();

// React to specific state entries
door.on_state_entry(State::Open, |state| {
    println!("Door opened! Turning on lights...");
});

// React to specific transitions
door.on_transition(State::Closed, Input::Lock, |from, input, to| {
    println!("Security system activated: {from:?} --{input:?}--> {to:?}");
});

// Global monitoring
door.on_any_transition(|from, input, to| {
    println!("State change: {from:?} → {to:?}");
});

// Now all transitions will trigger callbacks
door.transition(Input::OpenDoor).unwrap(); // Triggers callbacks
```

### 2. Query & Analysis

Analyze your state machine structure:

```rust
// Find reachable states
let reachable = StateMachineQuery::<DoorStateMachine>::reachable_states(&State::Closed);
println!("From Closed, can reach: {reachable:?}");

// Check connectivity
let has_path = StateMachineQuery::<DoorStateMachine>::has_path(
    &State::Open, 
    &State::Locked
);
println!("Open can reach Locked: {has_path}");

// Find shortest path
if let Some(path) = StateMachineQuery::<DoorStateMachine>::shortest_path(
    &State::Open, 
    &State::Locked
) {
    println!("Shortest path: {path:?}");
}
```

### 3. Documentation Generation

Generate visual documentation automatically:

```rust
// Mermaid state diagram
let diagram = StateMachineDoc::<DoorStateMachine>::generate_mermaid();
println!("{diagram}");

// Transition table
let table = StateMachineDoc::<DoorStateMachine>::generate_transition_table();
println!("{table}");
```

### 4. History Management

Track transitions with efficient ring buffer:

```rust
// Custom history size
let mut door = StateMachineInstance::<DoorStateMachine>::with_max_history(100);

// Make some transitions
door.transition(Input::OpenDoor).unwrap();
door.transition(Input::CloseDoor).unwrap();

// View history
println!("History: {:?}", door.history());
println!("History length: {}", door.history_len());
```

## 🛠️ Advanced Features

### Hidden Operations

Use underscore-prefixed inputs for internal operations that won't appear in documentation:

```rust
define_state_machine! {
    name: ServerStateMachine,
    states: { Active, Maintenance },
    inputs: { Maintain, Restore, _Debug, _AdminReset }, // Hidden inputs
    initial: Active,
    transitions: {
        Active + Maintain => Maintenance,
        Maintenance + Restore => Active,
        // Hidden transitions (functional but not documented)
        Active + _Debug => Active,
        Maintenance + _AdminReset => Active
    }
}
```

### Multiple Callback Types

The callback system supports various event types:

```rust
let mut workflow = StateMachineInstance::<WorkflowStateMachine>::new();

// State-specific callbacks
workflow.on_state_entry(State::Review, |state| { /* ... */ });
workflow.on_state_exit(State::Draft, |state| { /* ... */ });

// Transition-specific callbacks  
workflow.on_transition(State::Draft, Input::Submit, |from, input, to| { /* ... */ });

// Global callbacks
workflow.on_any_state_entry(|state| { /* ... */ });
workflow.on_any_state_exit(|state| { /* ... */ });
workflow.on_any_transition(|from, input, to| { /* ... */ });

// Callback management
println!("Total callbacks: {}", workflow.callback_count());
workflow.clear_callbacks();
```

### Feature Flags

#### Serde Support

Enable with the `serde` feature for serialization:

```toml
[dependencies]
yasm = { version = "0.4.1", features = ["serde"] }
```

```rust
#[cfg(feature = "serde")]
{
    let state = State::Open;
    let json = serde_json::to_string(&state).unwrap();
    let restored: State = serde_json::from_str(&json).unwrap();
}
```

## 📚 Examples

Run comprehensive examples:

```bash
# Basic usage patterns
cargo run --example basic_demo

# Advanced features and analysis
cargo run --example advanced_usage

# Callback system demonstration  
cargo run --example callback_demo

# Simple callback examples
cargo run --example simple_callback_demo

# Documentation generation
cargo run --example generate_docs
```

## 🏗️ Architecture

```
src/
├── lib.rs          # Public API and comprehensive tests
├── core.rs         # StateMachine trait definition
├── instance.rs     # StateMachineInstance with history & callbacks
├── callbacks.rs    # Callback registry and event system
├── query.rs        # Analysis algorithms (paths, reachability)
├── doc.rs          # Documentation generation utilities
└── macros.rs       # define_state_machine! macro implementation
```

## 🔧 API Overview

### Core Types

- **`StateMachine`** - Core trait defining state machine behavior
- **`StateMachineInstance<SM>`** - Runtime instance with state and history
- **`CallbackRegistry<SM>`** - Event callback management system
- **`StateMachineQuery<SM>`** - Analysis and pathfinding utilities
- **`StateMachineDoc<SM>`** - Documentation generation tools

### Key Methods

```rust
// Instance management
let mut sm = StateMachineInstance::<MyStateMachine>::new();
let mut sm = StateMachineInstance::<MyStateMachine>::with_max_history(256);

// State operations
sm.transition(input)?;           // Execute transition
sm.current_state();              // Get current state
sm.valid_inputs();               // Get valid inputs
sm.can_accept(&input);           // Check if input is valid

// Callback registration  
sm.on_state_entry(state, callback);
sm.on_transition(from, input, callback);
sm.on_any_transition(callback);

// History access
sm.history();                    // Get transition history
sm.history_len();                // History length
sm.reset();                      // Reset to initial state

// Analysis
StateMachineQuery::<SM>::reachable_states(&from);
StateMachineQuery::<SM>::shortest_path(&from, &to);
StateMachineQuery::<SM>::has_path(&from, &to);

// Documentation
StateMachineDoc::<SM>::generate_mermaid();
StateMachineDoc::<SM>::generate_transition_table();
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Test with features
cargo test --features serde

# Test specific functionality
cargo test callbacks
```

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.

---

Built with ❤️ for the Rust community