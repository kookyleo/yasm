# YASM (Yet Another State Machine)

[![CI](https://github.com/kookyleo/yasm/workflows/CI/badge.svg)](https://github.com/kookyleo/yasm/actions)
[![codecov](https://codecov.io/github/kookyleo/yasm/graph/badge.svg?token=mtxTMfIqih)](https://codecov.io/github/kookyleo/yasm)
[![Crates.io](https://img.shields.io/crates/v/yasm.svg)](https://crates.io/crates/yasm)
[![Documentation](https://docs.rs/yasm/badge.svg)](https://docs.rs/yasm)
[![License](https://img.shields.io/crates/l/yasm.svg)](https://github.com/kookyleo/yasm#license)

一个现代、高效的**确定性**有限状态机库。

## 🚀 特性

- **⚡ 确定性状态机**: 每个状态+输入组合都有唯一确定的下一个状态，确保可预测性和可调试性
- **🎯 类型安全**: 利用 Rust 的类型系统在编译时防止无效的状态转换
- **🔧 宏驱动**: 使用简洁的声明式宏语法定义状态机
- **📊 可视化**: 自动生成 Mermaid 图表进行状态机可视化
- **🔍 丰富的查询 API**: 全面的状态机分析功能，包括路径查找、可达性和连通性分析
- **📝 文档生成**: 自动生成转换表、统计信息和完整文档
- **🔒 隐藏操作**: 支持下划线前缀的输入，功能完整但不出现在文档中
- **📈 内存高效**: 环形缓冲区实现，可配置历史记录限制（默认: 512 条）
- **🏗️ 模块化架构**: 核心、实例、查询和文档模块的清晰分离
- **📦 可选 Serde 支持**: 通过 `serde` 特性支持状态机序列化和反序列化

## 📦 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
yasm = "0.4.1"

# 启用序列化支持
yasm = { version = "0.4.1", features = ["serde"] }
```

## 🎯 快速开始

### 基本用法

```rust
use yasm::*;

// 定义一个简单的门状态机
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
    // 创建状态机实例
    let mut door = StateMachineInstance::<door::DoorStateMachine>::new();
    
    // 查看当前状态
    println!("当前状态: {:?}", door.current_state()); // Closed
    
    // 执行状态转换
    door.transition(door::Input::OpenDoor).unwrap();
    println!("新状态: {:?}", door.current_state()); // Open
    
    // 查看有效输入
    println!("有效输入: {:?}", door.valid_inputs()); // [CloseDoor]
}
```

### 高级特性

#### 历史记录管理
```rust
// 创建具有自定义历史记录限制的实例
let mut door = StateMachineInstance::<door::DoorStateMachine>::with_max_history(100);

// 查看转换历史（高效的环形缓冲区）
println!("历史记录: {:?}", door.history());
println!("最大历史记录大小: {}", door.max_history_size());
```

#### 隐藏操作
```rust
define_state_machine! {
    name: ServerStateMachine,
    states: { Active, Maintenance },
    inputs: { Maintain, Restore, _Debug, _Log },  // _Debug 和 _Log 是隐藏的
    initial: Active,
    transitions: {
        Active + Maintain => Maintenance,
        Maintenance + Restore => Active,
        // 隐藏操作（不会出现在文档中但功能完整）
        Active + _Debug => Active,
        Maintenance + _Debug => Maintenance,
        Active + _Log => Active,
        Maintenance + _Log => Maintenance
    }
}
```

#### 查询和分析
```rust
// 查找可达状态
let reachable = StateMachineQuery::<door::DoorStateMachine>::reachable_states(&door::State::Closed);
println!("从 Closed 可达: {:?}", reachable);

// 查找状态间路径
let has_path = StateMachineQuery::<door::DoorStateMachine>::has_path(
    &door::State::Open, 
    &door::State::Locked
);
println!("路径存在: {}", has_path);

// 查找最短路径
let path = StateMachineQuery::<door::DoorStateMachine>::shortest_path(
    &door::State::Open, 
    &door::State::Locked
);
println!("最短路径: {:?}", path);
```

#### 文档生成
```rust
// 生成 Mermaid 图表
let mermaid = StateMachineDoc::<door::DoorStateMachine>::generate_mermaid();
println!("{}", mermaid);

// 生成转换表
let table = StateMachineDoc::<door::DoorStateMachine>::generate_transition_table();
println!("{}", table);

// 生成完整文档
let docs = StateMachineDoc::<door::DoorStateMachine>::generate_full_documentation();
println!("{}", docs);
```

## 📚 示例

项目包含全面的示例：

### 🎯 基础演示
```bash
cargo run --example basic_demo
```
- 门和订单状态机
- 基本转换和查询
- 文档生成

### 🚀 高级用法
```bash
cargo run --example advanced_usage
```
- 网络连接状态机
- 游戏角色状态机
- 状态机分析工具

### 🔧 特性演示
```bash
cargo run --example feature_demo
```
- 隐藏操作演示
- 历史记录管理特性
- 性能优化

### 📖 文档生成
```bash
cargo run --example generate_docs
```
- 自动生成项目文档
- 创建 Mermaid 图表文件
- 生成转换表

## 🏗️ 架构

```
yasm/
├── src/
│   ├── lib.rs          # 主库入口点，包含全面测试
│   ├── core.rs         # StateMachine trait 和核心类型定义
│   ├── instance.rs     # StateMachineInstance 实现，包含历史记录管理
│   ├── query.rs        # StateMachineQuery 分析和路径查找算法
│   ├── doc.rs          # StateMachineDoc 文档生成工具
│   └── macros.rs       # define_state_machine! 宏实现
├── examples/           # 全面的示例和演示
└── docs/              # 自动生成的文档
```

## 🔧 API 参考

### 核心组件

#### `StateMachine` Trait
定义确定性状态机的核心行为：
- `states()` - 获取所有可能状态
- `inputs()` - 获取所有可能输入
- `next_state()` - 确定性状态转换逻辑
- `valid_inputs()` - 获取状态的有效输入

#### `StateMachineInstance<SM>`
带历史记录跟踪的运行时状态机实例：
- `new()` - 创建默认历史记录实例（512 条）
- `with_max_history(size)` - 创建自定义历史记录限制的实例
- `transition(input)` - 执行状态转换
- `current_state()` - 获取当前状态
- `valid_inputs()` - 获取当前状态的有效输入
- `history()` - 访问转换历史（环形缓冲区）

#### `StateMachineQuery<SM>`
状态机分析工具：
- `reachable_states(from)` - 查找所有可达状态
- `states_leading_to(target)` - 查找能到达目标的状态
- `has_path(from, to)` - 检查路径是否存在
- `shortest_path(from, to)` - 查找状态间最短路径
- `terminal_states()` - 查找没有出口转换的状态
- `is_strongly_connected()` - 检查图的连通性

#### `StateMachineDoc<SM>`
文档生成工具：
- `generate_mermaid()` - 创建 Mermaid 状态图
- `generate_transition_table()` - 创建 Markdown 转换表
- `generate_statistics()` - 生成状态机统计信息
- `generate_full_documentation()` - 完整文档包

## 🎨 设计原则

1. **确定性优先**: 每个状态+输入组合映射到唯一的下一个状态
2. **类型安全**: 编译时防止无效状态转换
3. **零成本抽象**: 高效的宏生成代码，最小运行时开销
4. **内存高效**: 环形缓冲区历史记录管理，可配置限制
5. **开发者体验**: 清晰的 API、全面的文档和有用的错误信息
6. **可扩展性**: 模块化设计允许轻松扩展和定制

## 🚀 性能特性

- **环形缓冲区历史**: 转换历史的自动内存管理
- **编译时生成**: 宏生成的代码具有最佳性能
- **最少分配**: 高效的数据结构和内存使用
- **可配置限制**: 根据需要自定义内存使用

## 🔄 路线图

- [ ] 带守卫的条件转换
- [ ] 更多导出格式（GraphViz、PlantUML）
- [ ] WebAssembly 支持

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行带特性的测试
cargo test --features serde

# 运行特定测试
cargo test test_deterministic_state_machine_basic
```

## 📄 许可证

MIT 许可证。详情请见 [LICENSE](LICENSE)。

## 🤝 贡献

欢迎贡献！请阅读我们的贡献指南并向我们的仓库提交 pull request。

---

用 ❤️ 为 Rust 社区构建