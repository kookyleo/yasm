# YASM Examples

This directory contains various usage examples for the YASM state machine library.

## Example List

### 1. Basic Demo (`basic_demo.rs`)
**Run command**: `cargo run --example basic_demo`

Demonstrates YASM's core functionality:
- 🚪 **Door State Machine**: Simple three-state machine (Closed, Open, Locked)
- 📦 **Order State Machine**: E-commerce order processing workflow
- 🔍 **Query Functions**: State reachability analysis, path queries
- 📊 **Documentation Generation**: Mermaid charts and transition tables

**Suitable for**: First-time YASM users to understand basic concepts and usage

### 2. Advanced Usage (`advanced_usage.rs`)
**Run command**: `cargo run --example advanced_usage`

Demonstrates more complex state machine scenarios:
- 🌐 **Network Connection State Machine**: Connection, reconnection, failure handling
- 🎮 **Game Character State Machine**: Multi-state character behavior control
- 📊 **State Machine Analysis**: Connectivity analysis, terminal state detection

**Suitable for**: Developers handling complex business logic

### 3. Documentation Generator (`generate_docs.rs`)
**Run command**: `cargo run --example generate_docs`

Automatically generates state machine documentation:
- 📝 Generate complete Markdown format documentation
- 🎨 Generate Mermaid format state diagram files
- 📁 Output to `docs/` directory

**Suitable for**: Projects that need to generate documentation for state machines

## Quick Start

1. **View basic functionality**:
   ```bash
   cargo run --example basic_demo
   ```

2. **Explore advanced features**:
   ```bash
   cargo run --example advanced_usage
   ```

3. **Generate project documentation**:
   ```bash
   cargo run --example generate_docs
   ```

## Example Output

After running examples, you will see:
- 🎯 Real-time demonstration of state transitions
- 📊 Query results and analysis data
- 🎨 Mermaid format state diagrams
- 📋 State transition tables

## Custom Examples

You can create your own state machines based on these examples:

1. Copy any example file
2. Modify the state machine definition
3. Adjust the demonstration logic
4. Run to see the effects

## Related Documentation

- [Main README](../README.md) - Complete library documentation
- [API Documentation](../src/lib.rs) - Detailed API descriptions
- [Generated Documentation](../docs/) - Auto-generated state machine documentation 