# YASM (Yet Another State Machine)

[![CI](https://github.com/kookyleo/yasm/workflows/CI/badge.svg)](https://github.com/kookyleo/yasm/actions)
[![codecov](https://codecov.io/github/kookyleo/yasm/graph/badge.svg?token=mtxTMfIqih)](https://codecov.io/github/kookyleo/yasm)
[![Crates.io](https://img.shields.io/crates/v/yasm.svg)](https://crates.io/crates/yasm)
[![Documentation](https://docs.rs/yasm/badge.svg)](https://docs.rs/yasm)
[![License](https://img.shields.io/crates/l/yasm.svg)](https://github.com/kookyleo/yasm#license)

现代、高效的**确定性**有限状态机库，专为 Rust 2024 edition 设计。

## ✨ 核心特性

- **⚡ 确定性**: 一个状态 + 输入 = 唯一下一状态（保证）
- **🔒 类型安全**: 编译时防止无效状态转换
- **🚀 回调系统**: 通过灵活的事件钩子响应状态变化
- **🔧 宏驱动**: 简洁的声明式状态机定义语法
- **📊 可视化**: 自动生成 Mermaid 图表和文档
- **🔍 分析工具**: 路径查找、可达性和连通性分析
- **📈 内存高效**: 可配置历史记录的环形缓冲区（默认: 512）
- **📦 可选 Serde**: 通过 `serde` 特性支持序列化

## 📦 安装

```toml
[dependencies]
yasm = "0.4.1"

# 启用序列化支持
yasm = { version = "0.4.1", features = ["serde"] }
```

## 🚀 快速开始

### 定义状态机

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

### 基本用法

```rust
fn main() {
    let mut door = StateMachineInstance::<DoorStateMachine>::new();
    
    println!("当前状态: {:?}", door.current_state()); // Closed
    
    door.transition(Input::OpenDoor).unwrap();
    println!("开门后: {:?}", door.current_state()); // Open
    
    // 查看下一步可能的输入
    println!("有效输入: {:?}", door.valid_inputs()); // [CloseDoor]
}
```

## 🔥 核心功能

### 1. 回调系统

通过灵活的回调钩子响应状态机事件：

```rust
let mut door = StateMachineInstance::<DoorStateMachine>::new();

// 响应特定状态进入
door.on_state_entry(State::Open, |state| {
    println!("门已打开！开启照明系统...");
});

// 响应特定转换
door.on_transition(State::Closed, Input::Lock, |from, input, to| {
    println!("安全系统激活: {from:?} --{input:?}--> {to:?}");
});

// 全局监控
door.on_any_transition(|from, input, to| {
    println!("状态变化: {from:?} → {to:?}");
});

// 现在所有转换都会触发回调
door.transition(Input::OpenDoor).unwrap(); // 触发回调
```

### 2. 查询与分析

分析状态机结构：

```rust
// 查找可达状态
let reachable = StateMachineQuery::<DoorStateMachine>::reachable_states(&State::Closed);
println!("从 Closed 可达: {reachable:?}");

// 检查连通性
let has_path = StateMachineQuery::<DoorStateMachine>::has_path(
    &State::Open, 
    &State::Locked
);
println!("Open 能到达 Locked: {has_path}");

// 查找最短路径
if let Some(path) = StateMachineQuery::<DoorStateMachine>::shortest_path(
    &State::Open, 
    &State::Locked
) {
    println!("最短路径: {path:?}");
}
```

### 3. 文档生成

自动生成可视化文档：

```rust
// Mermaid 状态图
let diagram = StateMachineDoc::<DoorStateMachine>::generate_mermaid();
println!("{diagram}");

// 转换表
let table = StateMachineDoc::<DoorStateMachine>::generate_transition_table();
println!("{table}");
```

### 4. 历史记录管理

通过高效环形缓冲区跟踪转换：

```rust
// 自定义历史记录大小
let mut door = StateMachineInstance::<DoorStateMachine>::with_max_history(100);

// 执行一些转换
door.transition(Input::OpenDoor).unwrap();
door.transition(Input::CloseDoor).unwrap();

// 查看历史记录
println!("历史记录: {:?}", door.history());
println!("历史记录长度: {}", door.history_len());
```

## 🛠️ 高级特性

### 隐藏操作

使用下划线前缀的输入进行内部操作，不会出现在文档中：

```rust
define_state_machine! {
    name: ServerStateMachine,
    states: { Active, Maintenance },
    inputs: { Maintain, Restore, _Debug, _AdminReset }, // 隐藏输入
    initial: Active,
    transitions: {
        Active + Maintain => Maintenance,
        Maintenance + Restore => Active,
        // 隐藏转换（功能完整但不记录在文档中）
        Active + _Debug => Active,
        Maintenance + _AdminReset => Active
    }
}
```

### 多种回调类型

回调系统支持各种事件类型：

```rust
let mut workflow = StateMachineInstance::<WorkflowStateMachine>::new();

// 状态特定回调
workflow.on_state_entry(State::Review, |state| { /* ... */ });
workflow.on_state_exit(State::Draft, |state| { /* ... */ });

// 转换特定回调  
workflow.on_transition(State::Draft, Input::Submit, |from, input, to| { /* ... */ });

// 全局回调
workflow.on_any_state_entry(|state| { /* ... */ });
workflow.on_any_state_exit(|state| { /* ... */ });
workflow.on_any_transition(|from, input, to| { /* ... */ });

// 回调管理
println!("回调总数: {}", workflow.callback_count());
workflow.clear_callbacks();
```

### 特性标志

#### Serde 支持

通过 `serde` 特性启用序列化：

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

## 📚 示例

运行全面的示例：

```bash
# 基本用法模式
cargo run --example basic_demo

# 高级特性和分析
cargo run --example advanced_usage

# 回调系统演示
cargo run --example callback_demo

# 简洁回调示例
cargo run --example simple_callback_demo

# 文档生成
cargo run --example generate_docs
```

## 🏗️ 架构

```
src/
├── lib.rs          # 公共 API 和全面测试
├── core.rs         # StateMachine trait 定义
├── instance.rs     # StateMachineInstance 包含历史记录和回调
├── callbacks.rs    # 回调注册表和事件系统
├── query.rs        # 分析算法（路径、可达性）
├── doc.rs          # 文档生成工具
└── macros.rs       # define_state_machine! 宏实现
```

## 🔧 API 概览

### 核心类型

- **`StateMachine`** - 定义状态机行为的核心 trait
- **`StateMachineInstance<SM>`** - 包含状态和历史记录的运行时实例
- **`CallbackRegistry<SM>`** - 事件回调管理系统
- **`StateMachineQuery<SM>`** - 分析和路径查找工具
- **`StateMachineDoc<SM>`** - 文档生成工具

### 关键方法

```rust
// 实例管理
let mut sm = StateMachineInstance::<MyStateMachine>::new();
let mut sm = StateMachineInstance::<MyStateMachine>::with_max_history(256);

// 状态操作
sm.transition(input)?;           // 执行转换
sm.current_state();              // 获取当前状态
sm.valid_inputs();               // 获取有效输入
sm.can_accept(&input);           // 检查输入是否有效

// 回调注册  
sm.on_state_entry(state, callback);
sm.on_transition(from, input, callback);
sm.on_any_transition(callback);

// 历史记录访问
sm.history();                    // 获取转换历史记录
sm.history_len();                // 历史记录长度
sm.reset();                      // 重置到初始状态

// 分析
StateMachineQuery::<SM>::reachable_states(&from);
StateMachineQuery::<SM>::shortest_path(&from, &to);
StateMachineQuery::<SM>::has_path(&from, &to);

// 文档生成
StateMachineDoc::<SM>::generate_mermaid();
StateMachineDoc::<SM>::generate_transition_table();
```

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 测试带特性
cargo test --features serde

# 测试特定功能
cargo test callbacks
```

## 📄 许可证

MIT 许可证。详见 [LICENSE](LICENSE)。

---

用 ❤️ 为 Rust 社区构建