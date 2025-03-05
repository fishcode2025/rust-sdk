# mcp-macros-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-macros-fishcode2025.svg)](https://crates.io/crates/mcp-macros-fishcode2025)
[![Documentation](https://docs.rs/mcp-macros-fishcode2025/badge.svg)](https://docs.rs/mcp-macros-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

用于简化 Model Context Protocol (MCP) 开发的宏。

## 功能

- 简化工具定义
- 简化资源处理
- 简化路由器实现

## 使用方法

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-core-fishcode2025 = "1.0"
mcp-macros-fishcode2025 = "1.0"
```

## 示例

```rust
use mcp_core_fishcode2025::tool::Tool;
use mcp_macros_fishcode2025::tool;

#[tool]
fn my_tool(param1: String, param2: i32) -> Result<String, String> {
    // 工具实现
    Ok(format!("处理了 {} 和 {}", param1, param2))
}
```

更多示例请参考 [项目文档](https://docs.rs/mcp-macros-fishcode2025) 或 examples 目录。 