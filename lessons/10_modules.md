# 第10课：模块系统

## 本课目标

- 理解 `mod`、`use`、`pub`
- 了解 crate 和模块树
- 掌握可见性控制

## Rust 的模块系统

Rust 用**模块**（module）来组织代码，由以下部分组成：

| 关键字 | 作用 |
|--------|------|
| `mod module_name { }` | **声明/定义**一个模块 |
| `mod module_name;` | **引入**外部文件作为模块 |
| `use path` | **导入**路径，缩短调用链 |
| `pub` | 让模块内的项**公开**可见 |

## 创建模块

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {  // pub = 外部可见
        a + b
    }

    fn helper() {  // 默认私有，仅模块内部可见
        println!("内部函数");
    }
}

fn main() {
    let result = math::add(3, 5);  // 用 :: 访问
    // math::helper();  // ❌ 私有的，不能访问
}
```

## 可见性规则

| 声明 | 含义 |
|------|------|
| （不加 pub） | **私有**：只有当前模块（及子模块）可访问 |
| `pub` | **公开**：外部代码可访问 |
| `pub(crate)` | crate 内部公开 |
| `pub(super)` | 父模块可见 |

## use 关键字

```rust
use math::add;           // 直接导入函数名
use std::collections::HashMap;  // 导入类型
use std::io::{self, Read};      // 一次导入多个

let result = add(3, 5);  // 不需要写 math::add 了
```

**惯例**：
- 函数：`use` 到父模块，调用时保留一层路径（如 `math::add`）
- 类型/结构体：`use` 到完整路径（如 `HashMap`，不是 `collections::HashMap`）

## 模块和文件

模块可以放在单独的文件里：

```
src/
├── main.rs
└── math_utils.rs       // math_utils 模块
```

```rust
// main.rs
mod math_utils;         // Rust 会自动找 math_utils_new 或 math_utils/mod.rs
use math_utils::add;
```

## 包（Package）和 Crate

- **Crate**：Rust 的编译单元。一个 crate 可以是一个**二进制文件**或一个**库**。
- **Package**：一个 Cargo.toml 文件定义的项目，可以包含多个 crate。
- `src/main.rs` 是**二进制 crate 根**（binary crate root）
- `src/lib.rs` 是**库 crate 根**（library crate root）

## 练习指南

打开 `src/bin/ex10_modules.rs`，完成里面的 TODO 任务。挑战部分会引导你把模块拆到单独文件。

运行：`cargo run --bin ex10_modules`

---

**第一阶段完成！** 你已经掌握了 Rust 的基础语法和所有权系统。

**下一步**：深入集合类型 → [第11课：字符串](11_strings.md)
