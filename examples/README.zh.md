# YASM 示例

这个目录包含了 YASM 状态机库的各种使用示例。

## 示例列表

### 1. 基础演示 (`basic_demo.rs`)
**运行命令**: `cargo run --example basic_demo`

展示 YASM 的核心功能：
- 🚪 **门状态机**: 简单的三状态机器（关闭、打开、锁定）
- 📦 **订单状态机**: 电商订单处理流程
- 🔍 **查询功能**: 状态可达性分析、路径查询
- 📊 **文档生成**: Mermaid 图表和转换表

**适合**: 初次使用 YASM 的用户，了解基本概念和用法

### 2. 高级用法 (`advanced_usage.rs`)
**运行命令**: `cargo run --example advanced_usage`

展示更复杂的状态机场景：
- 🌐 **网络连接状态机**: 连接、重连、失败处理
- 🎮 **游戏角色状态机**: 多状态角色行为控制
- 📊 **状态机分析**: 连通性分析、终端状态检测

**适合**: 需要处理复杂业务逻辑的开发者

### 3. 文档生成工具 (`generate_docs.rs`)
**运行命令**: `cargo run --example generate_docs`

自动生成状态机文档：
- 📝 生成 Markdown 格式的完整文档
- 🎨 生成 Mermaid 格式的状态图文件
- 📁 输出到 `docs/` 目录

**适合**: 需要为状态机生成文档的项目

## 快速开始

1. **查看基础功能**:
   ```bash
   cargo run --example basic_demo
   ```

2. **探索高级特性**:
   ```bash
   cargo run --example advanced_usage
   ```

3. **生成项目文档**:
   ```bash
   cargo run --example generate_docs
   ```

## 示例输出

运行示例后，你将看到：
- 🎯 状态转换的实时演示
- 📊 查询结果和分析数据
- 🎨 Mermaid 格式的状态图
- 📋 状态转换表

## 自定义示例

你可以基于这些示例创建自己的状态机：

1. 复制任一示例文件
2. 修改状态机定义
3. 调整演示逻辑
4. 运行查看效果

## 相关文档

- [主 README](../README.md) - 库的完整文档
- [API 文档](../src/lib.rs) - 详细的 API 说明
- [生成的文档](../docs/) - 自动生成的状态机文档 