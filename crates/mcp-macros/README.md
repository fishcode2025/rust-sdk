# mcp-macros-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-macros-fishcode2025.svg)](https://crates.io/crates/mcp-macros-fishcode2025)
[![Documentation](https://docs.rs/mcp-macros-fishcode2025/badge.svg)](https://docs.rs/mcp-macros-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

用于简化 Model Context Protocol (MCP) 开发的宏。这个包提供了一系列过程宏，可以大大减少实现 MCP 工具和资源所需的样板代码。

## 功能

- **工具宏**：简化工具定义和实现
- **资源宏**：简化资源处理和管理
- **路由器宏**：简化路由器实现
- **自动生成**：自动生成 JSON Schema 和参数验证
- **类型安全**：提供类型安全的接口

## 安装

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-core-fishcode2025 = "0.1.0"
mcp-macros-fishcode2025 = "0.1.0"
```

## 使用示例

### 工具宏

```rust
use mcp_core_fishcode2025::tool::ToolError;
use mcp_macros_fishcode2025::tool;
use mcp_core_fishcode2025 as mcp_core;

#[tool(
    name = "calculator",
    description = "执行基本的算术运算",
    params(
        x = "计算中的第一个数字",
        y = "计算中的第二个数字",
        operation = "要执行的操作（加、减、乘、除）"
    )
)]
async fn calculator(x: i32, y: i32, operation: String) -> Result<i32, ToolError> {
    match operation.as_str() {
        "add" => Ok(x + y),
        "subtract" => Ok(x - y),
        "multiply" => Ok(x * y),
        "divide" => {
            if y == 0 {
                Err(ToolError::ExecutionError("除数不能为零".into()))
            } else {
                Ok(x / y)
            }
        }
        _ => Err(ToolError::InvalidParameters(format!(
            "未知操作: {}",
            operation
        ))),
    }
}
```

### 路由器宏

```rust
use mcp_macros_fishcode2025::router;
use mcp_core_fishcode2025 as mcp_core;

#[router]
struct MyRouter {
    counter: std::sync::atomic::AtomicI32,
}

#[router::tool]
impl MyRouter {
    #[tool(name = "increment", description = "增加计数器")]
    async fn increment(&self) -> Result<i32, ToolError> {
        Ok(self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1)
    }
    
    #[tool(name = "get_count", description = "获取当前计数")]
    async fn get_count(&self) -> Result<i32, ToolError> {
        Ok(self.counter.load(std::sync::atomic::Ordering::SeqCst))
    }
}
```

### 资源宏

```rust
use mcp_macros_fishcode2025::resource;
use mcp_core_fishcode2025 as mcp_core;

#[resource(
    id = "user_profile",
    name = "用户资料",
    description = "包含用户信息的资源"
)]
struct UserProfile {
    name: String,
    age: u32,
    email: String,
}
```

## 注意事项

使用 `mcp-macros-fishcode2025` 时，需要同时导入 `mcp_core_fishcode2025 as mcp_core`，因为宏在展开时会引用 `mcp_core` 模块。

## 更多信息

更多示例和详细文档请参考 [项目文档](https://docs.rs/mcp-macros-fishcode2025) 或查看 examples 目录。 