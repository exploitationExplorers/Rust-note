# 📦 Rust 使用 mod 和 use 关键字管理模块

```rust
// lib.rs - 主模块文件
mod math; // 声明 math 模块
mod utils; // 声明 utils 模块

// 重新导出模块内容
pub use math::{add, multiply, PI, Calculator};
pub use utils::format_number;

// math.rs - 数学模块
pub const PI: f64 = 3.14159;

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// 结构体（类似 JavaScript 的类）
pub struct Calculator {
    result: i32,
}

impl Calculator {
    // 构造函数
    pub fn new() -> Self {
        Calculator { result: 0 }
    }
    
    pub fn add(&mut self, x: i32) -> &mut Self {
        self.result += x;
        self
    }
    
    pub fn get_result(&self) -> i32 {
        self.result
    }
}

// utils.rs - 工具模块
pub fn format_number(num: f64) -> String {
    format!("{:.2}", num)
}

// main.rs - 主程序
// 假设 crate 名称为 langshift-project
use langshift_project::math::{add, multiply, PI, Calculator};
use langshift_project::utils::format_number;

fn main() {
    println!("PI = {}", PI); // PI = 3.14159
    println!("add(5, 3) = {}", add(5, 3)); // add(5, 3) = 8
    println!("multiply(4, 2) = {}", multiply(4, 2)); // multiply(4, 2) = 8
    
    let mut calc = Calculator::new();
    calc.add(10).add(5);
    println!("calc result = {}", calc.get_result()); // calc result = 15
    
    println!("formatted: {}", format_number(3.14159)); // formatted: 3.14
}
```


## 🔄 模块系统差异
- 📁 **文件组织**: Rust 使用 mod 声明模块，JavaScript 使用文件路径
- 🔒 **可见性**: Rust 需要显式声明 pub 来公开函数和结构体
- 📥 **导入语法**: Rust 使用 use 关键字，JavaScript 使用 import
- 📤 **默认导出**: Rust 没有默认导出概念，需要显式导入


## Rust 的 Cargo

```rust
// Cargo.toml - 项目配置文件
[package]
name = "langshift-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "JavaScript 到 Rust 学习项目"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }

[dev-dependencies]
tokio-test = "0.4"

# 安装依赖
# cargo add serde tokio reqwest
# cargo add --dev tokio-test

// main.rs - 使用依赖
use serde::{Deserialize, Serialize};
use tokio;

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[tokio::main]
async fn main() {
    let user = User {
        name: String::from("Rust"),
        age: 25,
    };
    
    // 序列化为 JSON
    let json = serde_json::to_string(&user).unwrap();
    println!("JSON: {}", json);
    
    // 异步任务
    tokio::spawn(async {
        println!("异步任务执行中...");
    });
}
```

## 包管理差异
配置文件: npm 使用 package.json，Cargo 使用 Cargo.toml
依赖管理: Cargo 的依赖管理更严格，版本冲突处理更好
构建工具: Cargo 集成了构建、测试、文档生成等功能
特性系统: Rust 支持条件编译和特性标志



## Rust 项目结构

rust-project/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs          # 二进制程序入口
│   ├── lib.rs           # 库入口
│   ├── math.rs
│   ├── utils.rs
│   └── components/
│       ├── mod.rs       # 子模块声明
│       ├── calculator.rs
│       └── formatter.rs
├── tests/
│   ├── integration_test.rs
│   └── common/
│       └── mod.rs
├── examples/
│   └── basic_usage.rs
├── benches/
│   └── benchmark.rs
├── .gitignore
└── README.md