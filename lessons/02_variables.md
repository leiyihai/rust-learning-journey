# 第2课：变量与可变性

## 本课目标

- 掌握 `let` 变量绑定
- 理解 `mut` 可变性
- 了解 `const` 常量
- 理解变量遮蔽（Shadowing）

## 不可变变量（默认）

在 Rust 中，变量**默认不可变**：

```rust
let x = 5;
x = 6;  // ❌ 编译错误！x 是不可变的
```

这看起来不方便，但它是 Rust 安全哲学的核心：
- 你不需要跟踪哪些变量会变、哪些不会
- 编译器会帮你检查
- 这让并发的代码天然安全

## 可变变量：`mut`

如果你确实需要修改变量，加 `mut` 关键字：

```rust
let mut x = 5;
x = 6;  // ✓ 合法
println!("{x}");  // 输出 6
```

## 常量：`const`

```rust
const MAX_POINTS: u32 = 100_000;
```

常量和不可变变量的区别：

| 特性 | `let` | `const` |
|------|-------|---------|
| 类型标注 | 可选（可推断） | **必须**显式标注 |
| 值 | 可以是运行时计算 | **必须**是编译时常量 |
| 作用域 | 可以在任何作用域 | 可以在任何作用域 |
| 命名约定 | snake_case | SCREAMING_SNAKE_CASE |

## 变量遮蔽（Shadowing）

可以用 `let` 重新声明同名变量，遮蔽前一个：

```rust
let x = 5;
let x = x + 1;  // x = 6
let x = x * 2;  // x = 12

// Shadowing 甚至可以改变类型！
let spaces = "   ";
let spaces = spaces.len();  // 从 &str 变成了 usize
```

这和 `mut` 有本质区别：

| `mut` | Shadowing |
|-------|-----------|
| 修改同一个变量 | 创建**新的**变量 |
| 类型不能变 | 类型**可以**变 |
| 需要 `mut` 关键字 | 只需要 `let` |

## 练习指南

打开 `src/bin/ex02_variables.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex02_variables`

---

**下一步**：了解 Rust 有哪些数据类型 → [第3课：数据类型](03_types.md)
