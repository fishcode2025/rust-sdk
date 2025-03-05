# mcp-client-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-client-fishcode2025.svg)](https://crates.io/crates/mcp-client-fishcode2025)
[![Documentation](https://docs.rs/mcp-client-fishcode2025/badge.svg)](https://docs.rs/mcp-client-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Model Context Protocol (MCP) 的客户端实现，用于连接 MCP 服务器。

## 功能

- 支持多种传输方式：
  - 标准输入/输出 (stdio)
  - 服务器发送事件 (SSE)
  - 自定义传输
- 提供简单的 API 用于：
  - 初始化连接
  - 获取资源
  - 调用工具
  - 处理提示

## 安装

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-client-fishcode2025 = "0.1.0"
mcp-core-fishcode2025 = "0.1.0"
```

## 使用示例

### 基本用法

```rust
use mcp_client_fishcode2025::{McpClient, StdioTransport};
use mcp_core_fishcode2025::protocol::CallToolParams;
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

## 测试示例

### 测试 stdio 传输

```bash
cargo run -p mcp-client-fishcode2025 --example stdio
```

### 测试 SSE 传输

1. 在一个终端中启动 MCP 服务器: `fastmcp run -t sse echo.py`
2. 在另一个终端中运行客户端示例: `cargo run -p mcp-client-fishcode2025 --example sse`

## 自定义传输

您可以通过实现 `Transport` trait 来创建自定义传输：

```rust
use mcp_client_fishcode2025::transport::Transport;
use mcp_core_fishcode2025::protocol::{JsonRpcRequest, JsonRpcResponse};

struct MyTransport {
    // 您的状态
}

#[async_trait::async_trait]
impl Transport for MyTransport {
    async fn send(&mut self, request: JsonRpcRequest) -> anyhow::Result<JsonRpcResponse> {
        // 实现您的发送逻辑
    }
}
```

## 更多信息

更多示例和详细文档请参考 [项目文档](https://docs.rs/mcp-client-fishcode2025) 或查看 examples 目录。

