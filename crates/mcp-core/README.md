# mcp-core-fishcode2025

[![Crates.io](https://img.shields.io/crates/v/mcp-core-fishcode2025.svg)](https://crates.io/crates/mcp-core-fishcode2025)
[![Documentation](https://docs.rs/mcp-core-fishcode2025/badge.svg)](https://docs.rs/mcp-core-fishcode2025)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Model Context Protocol (MCP) 的核心数据结构和协议定义。这个包是 MCP Rust SDK 的基础，提供了所有其他包所需的基本类型和接口。

## 功能

- **协议消息定义**：JSON-RPC 请求和响应的数据结构
- **资源相关数据结构**：用于表示和管理上下文资源
- **工具相关数据结构**：用于定义和调用工具函数
- **提示相关数据结构**：用于处理模板化消息
- **错误处理**：统一的错误类型和处理机制
- **序列化/反序列化**：与 JSON 格式的互操作性

## 主要模块

- `protocol`：定义 JSON-RPC 消息格式和协议操作
- `resource`：资源管理相关的数据结构和函数
- `tool`：工具定义和调用相关的数据结构和函数
- `prompt`：提示模板相关的数据结构和函数
- `error`：错误类型和处理机制

## 安装

将以下依赖添加到您的`Cargo.toml`文件中：

```toml
[dependencies]
mcp-core-fishcode2025 = "0.1.0"
```

## 使用示例

### 基本协议操作

```rust
use mcp_core_fishcode2025::protocol::{JsonRpcRequest, JsonRpcResponse, Method};
use serde_json::json;

// 创建一个 JSON-RPC 请求
let request = JsonRpcRequest {
    jsonrpc: "2.0".to_string(),
    id: 1,
    method: Method::Initialize.to_string(),
    params: json!({
        "client_info": {
            "name": "my_client",
            "version": "1.0.0"
        }
    }),
};

// 处理响应
fn handle_response(response: JsonRpcResponse) {
    if let Some(error) = response.error {
        println!("错误: {:?}", error);
    } else if let Some(result) = response.result {
        println!("结果: {}", result);
    }
}
```

### 工具定义和使用

```rust
use mcp_core_fishcode2025::tool::{Tool, ToolParameter, ToolError};
use serde_json::Value;
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

// 定义工具参数
#[derive(Serialize, Deserialize, JsonSchema)]
struct CalculatorParams {
    a: i32,
    b: i32,
    operation: String,
}

// 实现工具
struct Calculator;

impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "一个简单的计算器工具"
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        // 返回参数定义
        vec![
            ToolParameter {
                name: "a".to_string(),
                description: "第一个数字".to_string(),
                required: true,
                schema: json!({"type": "integer"}),
            },
            // 其他参数...
        ]
    }
    
    async fn call(&self, params: Value) -> Result<Value, ToolError> {
        // 工具实现逻辑
        let params: CalculatorParams = serde_json::from_value(params)
            .map_err(|e| ToolError::InvalidParams(e.to_string()))?;
            
        match params.operation.as_str() {
            "add" => Ok(json!(params.a + params.b)),
            "subtract" => Ok(json!(params.a - params.b)),
            "multiply" => Ok(json!(params.a * params.b)),
            "divide" => {
                if params.b == 0 {
                    return Err(ToolError::ExecutionError("除数不能为零".to_string()));
                }
                Ok(json!(params.a / params.b))
            },
            _ => Err(ToolError::InvalidParams("不支持的操作".to_string())),
        }
    }
}
```

### 资源处理

```rust
use mcp_core_fishcode2025::resource::{Resource, ResourceType, ResourceContent};

// 创建文本资源
let text_resource = Resource {
    id: "doc1".to_string(),
    type_: ResourceType::Text,
    name: "示例文档".to_string(),
    description: Some("这是一个示例文本文档".to_string()),
    content: ResourceContent::Text("这是文档的内容".to_string()),
    metadata: None,
};

// 创建文件资源
let file_resource = Resource {
    id: "file1".to_string(),
    type_: ResourceType::File,
    name: "示例文件".to_string(),
    description: Some("这是一个示例文件".to_string()),
    content: ResourceContent::File {
        path: "/path/to/file.txt".to_string(),
        mime_type: Some("text/plain".to_string()),
    },
    metadata: None,
};
```

## 高级用法

### 自定义错误处理

```rust
use mcp_core_fishcode2025::error::{Error, ErrorCode};
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("资源不存在: {0}")]
    ResourceNotFound(String),
    
    #[error("无效的参数: {0}")]
    InvalidParameter(String),
}

// 转换为 MCP 错误
impl From<MyError> for Error {
    fn from(err: MyError) -> Self {
        match err {
            MyError::ResourceNotFound(msg) => Error {
                code: ErrorCode::ResourceNotFound as i32,
                message: format!("资源不存在: {}", msg),
                data: None,
            },
            MyError::InvalidParameter(msg) => Error {
                code: ErrorCode::InvalidParams as i32,
                message: format!("无效的参数: {}", msg),
                data: None,
            },
        }
    }
}
```

## 更多信息

更多示例和详细文档请参考 [项目文档](https://docs.rs/mcp-core-fishcode2025) 或查看 examples 目录。 