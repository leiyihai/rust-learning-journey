# 第6课：所有权（Ownership）

## 本课目标

- 理解 Rust 的所有权三大规则
- 掌握 Move（移动）语义
- 了解 Clone（克隆）和 Copy trait

## 为什么需要所有权？

传统的编程语言处理内存有三种方式：

| 方式 | 代表 | 问题 |
|------|------|------|
| 手动管理 | C/C++ | 容易 double free、内存泄漏 |
| 垃圾回收 | Java/Go/Python | 运行时开销、"Stop the world" |
| **所有权** | **Rust** | **编译时检查，零运行时开销** |

## 所有权三大规则

1. Rust 中每一个**值**都有一个**所有者**（owner）
2. 同一时间只能有**一个**所有者
3. 当所有者离开作用域，值会被**自动丢弃**（调用 `drop`）

## 作用域

```rust
{                          // s 还不存在
    let s = "hello";       // s 从这里开始有效
    // ... 使用 s
}                          // 作用域结束，s 被释放
```

## Move（移动）

对于 `String` 这样的堆分配类型：

```rust
let s1 = String::from("hello");
let s2 = s1;    // s1 的所有权**移动**到了 s2

println!("{s1}");  // ❌ 编译错误！s1 已经失效了
```

这避免了**双重释放**（double free）。如果用浅拷贝（像 C++ 那样），当 s1 和 s2 离开作用域时，它们都会尝试释放同一块内存。

> 记住：**赋值 = 移动所有权**（对非 Copy 类型）

## Clone（克隆）

如果你真的需要一份深拷贝：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();  // 在堆上复制一份新数据

println!("{s1}");  // ✓ s1 仍然有效
println!("{s2}");  // ✓
```

`clone()` 有运行时开销（需要分配新内存并复制数据）。

## Copy Trait

对于**存储在栈上**的简单类型，Rust 会自动实现 `Copy` trait，赋值时直接复制而不会移动：

```rust
let x = 5;
let y = x;         // x 被**复制**了，没有移动
println!("{x}");   // ✓ 仍然有效
```

实现 `Copy` 的类型（部分）：
- 所有整数类型（`i32`、`u64` 等）
- 所有浮点数类型（`f64`、`f32`）
- `bool`
- `char`
- 元组（如果所有元素都实现了 Copy）

**不会**实现 Copy 的类型：
- `String`
- `Vec<T>`
- 任何实现了 `Drop` trait 的类型

## 函数与所有权

```rust
fn main() {
    let s = String::from("hello");
    take_ownership(s);     // s 的所有权移动到函数内部
    // println!("{s}");    // ❌ s 已经无效

    let x = 5;
    make_copy(x);          // x 是 i32，实现了 Copy，所以只是复制
    println!("{x}");       // ✓ 仍然有效
}
```

## 所有权的好处

1. **不需要手动 free** —— 自动释放，不依赖 GC
2. **编译时检查** —— 错误在编译时暴露，不等到运行时
3. **线程安全** —— 所有权规则天然防止数据竞争

## 练习指南

打开 `src/bin/ex06_ownership.rs`，完成里面的 TODO 任务。

这是 Rust 最重要的概念，多花时间理解！

运行：`cargo run --bin ex06_ownership`

---

**下一步**：借而不移 → [第7课：引用与借用](07_references.md)
