# MCP (Model Context Protocol) Rust SDK

这是Model Context Protocol (MCP)的Rust实现，本项目fork自[官方MCP Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)。

[![Crates.io](https://img.shields.io/crates/v/mcp-core.svg)](https://crates.io/crates/mcp-core)
[![Documentation](https://docs.rs/mcp-core/badge.svg)](https://docs.rs/mcp-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 项目简介

Model Context Protocol (MCP) 是一个开放协议，旨在实现LLM应用程序与外部数据源和工具之间的无缝集成。无论您是构建AI驱动的IDE、增强聊天界面，还是创建自定义AI工作流，MCP都提供了一种标准化的方式来连接LLM与它们所需的上下文。

本SDK提供了MCP协议的Rust实现，包括：

- **mcp-core**: 核心数据结构和协议定义
- **mcp-server**: 服务器端实现，用于创建MCP服务
- **mcp-client**: 客户端实现，用于连接MCP服务
- **mcp-macros**: 用于简化MCP开发的宏

## 架构概述

MCP协议基于以下核心概念：

```
┌─────────────┐      ┌─────────────┐      ┌─────────────┐
│             │      │             │      │             │
│    Host     │◄────►│   Client    │◄────►│   Server    │
│  (LLM应用)   │      │ (MCP客户端) │      │ (MCP服务器) │
│             │      │             │      │             │
└─────────────┘      └─────────────┘      └─────────────┘
```

- **Host**: LLM应用程序，如AI助手或IDE
- **Client**: 在Host中运行的MCP客户端，负责与Server通信
- **Server**: 提供上下文和功能的MCP服务器

MCP协议支持三种主要功能：

1. **Resources (资源)**: 上下文和数据，供用户或AI模型使用
2. **Prompts (提示)**: 用户的模板化消息和工作流
3. **Tools (工具)**: AI模型可以执行的函数

## 安装

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
# 核心库
mcp-core = "1.0"

# 如果需要创建MCP服务器
mcp-server = "1.0"

# 如果需要创建MCP客户端
mcp-client = "1.0"

# 可选的宏支持
mcp-macros = "1.0"
```

或者，您可以直接从GitHub克隆并使用本地路径：

```toml
[dependencies]
mcp-core = { path = "./crates/mcp-core" }
mcp-server = { path = "./crates/mcp-server" }
mcp-client = { path = "./crates/mcp-client" }
mcp-macros = { path = "./crates/mcp-macros" }
```

## 使用指南

### 创建MCP服务器

以下是创建简单MCP服务器的示例：

```rust
use mcp_server::router::RouterService;
use mcp_server::{ByteTransport, Server};
use tokio::io::{stdin, stdout};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建路由器实例
    let router = RouterService(MyRouter::new());
    
    // 创建并运行服务器
    let server = Server::new(router);
    let transport = ByteTransport::new(stdin(), stdout());
    
    Ok(server.run(transport).await?)
}
```

### 创建MCP客户端

以下是创建MCP客户端并调用工具的示例：

```rust
use mcp_client::{McpClient, StdioTransport};
use mcp_core::protocol::CallToolParams;
use serde_json::json;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建客户端
    let transport = StdioTransport::new(/* 命令和参数 */)?;
    let mut client = McpClient::new(transport);
    
    // 初始化连接
    client.initialize().await?;
    
    // 调用工具
    let result = client.call_tool(CallToolParams {
        name: "example_tool".to_string(),
        arguments: json!({ "param": "value" }),
    }).await?;
    
    println!("工具调用结果: {:?}", result);
    Ok(())
}
```

### 实现自定义路由器

要创建自定义MCP服务器，您需要实现`Router` trait：

```rust
use mcp_core::{Resource, Tool, ToolError};
use mcp_server::Router;
use serde_json::Value;

struct MyRouter {
    // 您的状态
}

impl MyRouter {
    fn new() -> Self {
        Self { /* 初始化 */ }
    }
    
    async fn my_tool(&self, args: Value) -> Result<Value, ToolError> {
        // 实现您的工具逻辑
        Ok(json!({ "result": "success" }))
    }
}

impl Router for MyRouter {
    fn name(&self) -> String {
        "my_router".to_string()
    }
    
    // 实现其他必要的方法...
}
```

## 示例

本项目包含多个示例，展示了MCP的不同用例：

- **计数器服务器**: 一个简单的服务器，提供计数器功能
- **SQLite服务器**: 连接到SQLite数据库的服务器
- **文件系统服务器**: 提供文件系统访问的服务器

要运行示例，请使用以下命令：

```bash
# 运行计数器服务器示例
cargo run --example counter-server

# 运行客户端连接到服务器
cargo run --example clients -- --server-command "cargo run --example counter-server"
```

## 项目结构

```
rust-sdk/
├── crates/                  # 核心crate
│   ├── mcp-core/            # 核心数据结构和协议定义
│   ├── mcp-server/          # 服务器实现
│   ├── mcp-client/          # 客户端实现
│   └── mcp-macros/          # 辅助宏
├── examples/                # 示例代码
│   ├── clients/             # 客户端示例
│   ├── servers/             # 服务器示例
│   └── macros/              # 宏使用示例
└── servers/                 # 完整服务器实现
    └── src/                 # 服务器源代码
        └── sqlite/          # SQLite服务器实现
```

## 核心组件流程

```
┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │
│    请求/响应    │────►│    传输层      │────►│    路由器      │
│                │     │                │     │                │
└────────────────┘     └────────────────┘     └────────────────┘
                                                      │
                                                      ▼
┌────────────────┐     ┌────────────────┐     ┌────────────────┐
│                │     │                │     │                │
│    资源处理     │◄────┤    工具处理    │◄────┤   处理器分发   │
│                │     │                │     │                │
└────────────────┘     └────────────────┘     └────────────────┘
```

## API文档

完整的API文档可以通过以下命令生成：

```bash
cargo doc --open
```

主要模块包括：

- **mcp_core::protocol**: 协议消息定义
- **mcp_core::resource**: 资源相关数据结构
- **mcp_core::tool**: 工具相关数据结构
- **mcp_server::router**: 服务器路由功能
- **mcp_client::client**: 客户端实现

## 贡献指南

我们欢迎所有形式的贡献！无论是修复错误、改进文档还是提出新功能，请按照以下步骤：

1. Fork本仓库
2. 创建您的特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交您的更改 (`git commit -m 'Add some amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 开启一个Pull Request

### 开发环境设置

```bash
# 克隆仓库
git clone https://github.com/yourusername/rust-sdk.git
cd rust-sdk

# 构建项目
cargo build

# 运行测试
cargo test
```

## 许可证

本项目采用MIT许可证 - 详情请参见[LICENSE](LICENSE)文件。

## 相关资源

- [MCP协议规范](https://spec.modelcontextprotocol.io/)
- [MCP官方网站](https://modelcontextprotocol.io/)
- [MCP GitHub组织](https://github.com/modelcontextprotocol)

## 致谢

本项目是[Model Context Protocol官方Rust SDK](https://github.com/modelcontextprotocol/rust-sdk)的一个fork，我们感谢原始项目的所有贡献者。


