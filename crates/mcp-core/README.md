# mcp-core-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-core-fishcode2025.svg)](https://crates.io/crates/mcp-core-fishcode2025)
[![Documentation](https://docs.rs/mcp-core-fishcode2025/badge.svg)](https://docs.rs/mcp-core-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Model Context Protocol (MCP) 的核心数据结构和协议定义。

## 功能

- 协议消息定义
- 资源相关数据结构
- 工具相关数据结构
- 提示相关数据结构

## 使用方法

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-core-fishcode2025 = "1.0"
```

## 示例

```rust
use mcp_core_fishcode2025::protocol::{JsonRpcRequest, JsonRpcResponse};
use mcp_core_fishcode2025::tool::Tool;
use mcp_core_fishcode2025::resource::Resource;

// 使用核心数据结构
```

更多示例请参考 [项目文档](https://docs.rs/mcp-core-fishcode2025)。 