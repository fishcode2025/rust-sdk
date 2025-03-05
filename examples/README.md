# Model Context Protocol 示例

本目录包含演示如何使用 Model Context Protocol (MCP) Rust SDK 的示例。

## 目录结构

- `clients/`: MCP 客户端示例
- `servers/`: MCP 服务器示例
- `macros/`: MCP 宏示例

## 运行客户端示例

客户端示例演示了连接 MCP 服务器的不同方式。

### 可用示例

您可以通过两种方式运行这些示例：

#### 方式一：从 examples/clients 目录运行

```bash
cd examples/clients
cargo run --example clients
cargo run --example sse
cargo run --example stdio
cargo run --example stdio_integration
```

#### 方式二：从根目录运行

```bash
cargo run -p mcp-client-examples-fishcode2025 --example clients
cargo run -p mcp-client-examples-fishcode2025 --example sse
cargo run -p mcp-client-examples-fishcode2025 --example stdio
cargo run -p mcp-client-examples-fishcode2025 --example stdio_integration
```

## 运行服务器示例

服务器示例演示了如何实现 MCP 服务器。

### 可用示例

您可以通过两种方式运行服务器示例：

#### 方式一：从 examples/servers 目录运行

```bash
cd examples/servers
cargo run --example counter-server
```

#### 方式二：从根目录运行

```bash
cargo run -p mcp-server-examples-fishcode2025 --example counter-server
```

## 运行宏示例

宏示例演示了如何使用 MCP 宏创建工具。

### 可用示例

您可以通过两种方式运行宏示例：

#### 方式一：从 examples/macros 目录运行

```bash
cd examples/macros
cargo run --example calculator
```

#### 方式二：从根目录运行

```bash
cargo run -p mcp-macros-examples-fishcode2025 --example calculator
```

## 示例说明

### 客户端示例

- **clients**: 演示如何创建和使用 MCP 客户端
- **sse**: 演示如何使用服务器发送事件 (SSE) 传输
- **stdio**: 演示如何使用标准输入/输出传输
- **stdio_integration**: 演示如何将 stdio 传输集成到现有应用程序中

### 服务器示例

- **counter-server**: 一个简单的计数器服务器，提供增加和获取计数的功能
- **axum**: 演示如何将 MCP 服务器集成到 Axum web 框架中

### 宏示例

- **calculator**: 演示如何使用 `#[tool]` 宏创建一个简单的计算器工具

## 注意事项

- 某些示例可能需要额外的设置或同时运行客户端和服务器组件。
- 服务器示例使用标准 I/O 进行通信，因此可以使用 stdio 传输连接到客户端示例。
- 对于 SSE 示例，您可能需要运行单独的 SSE 服务器或使用兼容的 MCP 服务器实现。
- 使用宏示例时，请确保导入 `use mcp_core_fishcode2025 as mcp_core;`，以避免宏展开错误。

## 依赖关系

所有示例都依赖于以下包：

- `mcp-core-fishcode2025`: 核心数据结构和协议定义
- `mcp-client-fishcode2025`: 客户端实现
- `mcp-server-fishcode2025`: 服务器实现
- `mcp-macros-fishcode2025`: 宏实现
