# mcp-server-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-server-fishcode2025.svg)](https://crates.io/crates/mcp-server-fishcode2025)
[![Documentation](https://docs.rs/mcp-server-fishcode2025/badge.svg)](https://docs.rs/mcp-server-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Model Context Protocol (MCP) 的服务器端实现，用于创建 MCP 服务。

## 功能

- **路由器**：处理客户端请求的核心组件
- **传输层**：支持多种通信方式
- **资源管理**：提供上下文和数据
- **工具处理**：执行函数和操作
- **提示处理**：处理模板化消息

## 安装

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-server-fishcode2025 = "0.1.0"
mcp-core-fishcode2025 = "0.1.0"
```

## 使用示例

### 基本服务器

```rust
use mcp_server_fishcode2025::router::RouterService;
use mcp_server_fishcode2025::{ByteTransport, Server};
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

### 自定义路由器

```rust
use mcp_core_fishcode2025::{Resource, Tool, ToolError};
use mcp_server_fishcode2025::Router;
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

## 测试

### 使用 MCP Inspector 测试

```bash
npx @modelcontextprotocol/inspector cargo run -p mcp-server-fishcode2025
```

然后在浏览器窗口中访问 Inspector 并测试不同的端点。

## 高级用法

### 集成到 Web 服务器

```rust
use axum::{routing::post, Router};
use mcp_server_fishcode2025::router::RouterService;

async fn setup_mcp_server() -> Router {
    let mcp_router = RouterService(MyRouter::new());
    
    Router::new()
        .route("/mcp", post(move |body: String| async move {
            // 处理 MCP 请求
        }))
}
```

## 更多信息

更多示例和详细文档请参考 [项目文档](https://docs.rs/mcp-server-fishcode2025) 或查看 examples 目录。

我用这个server实现了一个 `mcp-sqlite`，用法可参考[该项目](https://github.com/fishcode2025/mcp-sqlite)。