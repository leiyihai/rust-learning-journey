# 第1课：Hello World & Cargo

## 本课目标

- 了解 Rust 工具链（rustc、cargo）
- 创建和运行第一个 Rust 程序
- 认识 `println!` 宏

## 什么是 Rust？

Rust 是一门**系统编程语言**，由 Mozilla 开发，注重三个目标：

1. **安全**（Safety）—— 编译器消灭大部分内存 bug
2. **并发**（Concurrency）—— 没有数据竞争的并行编程
3. **性能**（Performance）—— 媲美 C/C++，零成本抽象

## 工具链介绍

| 工具 | 作用 |
|------|------|
| `rustc` | Rust 编译器，将 .rs 文件编译为可执行文件 |
| `cargo` | Rust 的包管理器和构建系统（类似 npm + webpack） |
| `rustup` | Rust 工具链管理器（安装、更新、切换版本） |

## 第一个程序

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn main()` —— 程序的入口点（类似 C 的 `int main()`）
- `println!` —— 这是一个**宏**（macro），注意末尾的 `!`
- `"Hello, world!"` —— 字符串字面量
- `;` —— 语句必须以分号结尾

## println! 宏

`println!` 支持格式化输出，类似 C 的 `printf`：

```rust
let name = "Alice";
let age = 20;
println!("{} 今年 {} 岁", name, age);
// 输出：Alice 今年 20 岁
```

- `{}` —— 占位符，会被后面的参数替换
- 自动换行；如果不想换行，用 `print!`

## Cargo 常用命令

```bash
cargo new project_name    # 创建新项目
cargo build               # 编译项目（debug 模式）
cargo build --release     # 编译项目（release 模式，优化）
cargo run                 # 编译并运行
cargo run --bin target    # 运行特定的二进制文件
cargo check               # 只检查编译错误，不生成可执行文件（快）
cargo clean               # 删除 target 目录
```

## Cargo 项目结构

```
my_project/
├── Cargo.toml     # 项目元数据和依赖
├── .gitignore     # Git 忽略规则
└── src/
    └── main.rs    # 入口文件
```

## 小技巧：善用编译器错误信息

Rust 的编译器以**友善且详细的错误信息**著称。当你写错代码时：

1. 仔细阅读错误信息
2. 编译器通常会告诉你**问题在哪**以及**建议怎么修复**
3. 很多时候，照着编译器说的改就行了！

试试故意删除 `println!` 的 `!`，看看编译器怎么说。

## 练习指南

打开 `src/bin/ex01_hello.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex01_hello`

---

**下一步**：掌握变量绑定 → [第2课：变量与可变性](02_variables.md)
