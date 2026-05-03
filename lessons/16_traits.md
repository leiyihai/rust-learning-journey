# 第16课：特性（Traits）

## 本课目标

- 定义和实现 trait
- trait bound：约束泛型的行为
- `derive` 宏和常见标准库 trait

## 什么是 Trait？

Trait 定义了**共享的行为**——类似于其他语言的接口（interface）。

```rust
// 定义一个 trait
trait Speak {
    fn speak(&self) -> &str;               // 方法签名（没有实现）
    fn introduce(&self) {                  // 可以有默认实现
        println!("我是谁？");
    }
}
```

## 实现 Trait

```rust
struct Dog { name: String }
struct Cat { name: String }

impl Speak for Dog {
    fn speak(&self) -> &str {
        "汪汪！"
    }
}

impl Speak for Cat {
    fn speak(&self) -> &str {
        "喵喵！"
    }
}
```

## Trait Bound

用 trait 约束泛型参数必须实现某些行为：

```rust
// 写法 1：impl Trait 语法
fn make_it_speak(animal: &impl Speak) {
    println!("{}", animal.speak());
}

// 写法 2：trait bound 语法（等价，但更灵活）
fn make_it_speak<T: Speak>(animal: &T) {
    println!("{}", animal.speak());
}

// 多个 trait bound
fn print_and_debug<T: Speak + std::fmt::Debug>(item: &T) {
    println!("{:?} 说: {}", item, item.speak());
}

// where 子句（更清晰的写法）
fn complex<T, U>(t: &T, u: &U) -> i32
where
    T: Speak + Clone,
    U: Clone + std::fmt::Debug,
{
    // ...
}
```

## Derive 宏

很多标准库 trait 可以通过 `#[derive]` 自动生成：

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    name: String,
    age: u8,
}
```

常见可派生 trait：

| Trait | 作用 |
|-------|------|
| `Debug` | `{:?}` 调试打印 |
| `Clone` | 显式复制（`.clone()`） |
| `Copy` | 隐式复制（慎用！） |
| `PartialEq` | `==` 和 `!=` 比较 |
| `Eq` | 等价关系（配合 PartialEq） |
| `PartialOrd` | `<` `>` `<=` `>=` 比较 |
| `Hash` | 可哈希（用于 HashMap） |
| `Default` | 默认值（`Default::default()`） |

## Display vs Debug

```rust
use std::fmt;

#[derive(Debug)]
struct Book { title: String, pages: u32 }

// Debug 是自动生成的，Display 需要手动实现
impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "《{}》({} 页)", self.title, self.pages)
    }
}

let book = Book { title: "Rust".into(), pages: 300 };
println!("{:?}", book);   // 输出：Book { title: "Rust", pages: 300 }
println!("{book}");       // 输出：《Rust》(300 页)
```

## 孤儿规则（Orphan Rule）

你只能在 trait 所在 crate 或类型所在 crate 实现 trait。这防止了冲突的实现。

```rust
// ✓ 可以：在自己的 crate 为自己的类型实现标准库 trait
impl Display for MyType { ... }

// ✓ 可以：在自己的 crate 为标准库类型实现自己的 trait
impl MyTrait for String { ... }

// ❌ 不可以：为外部类型实现外部 trait（孤儿规则）
impl Display for String { ... }
```

## 练习指南

打开 `src/bin/ex16_traits.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex16_traits`

---

**下一步**：理解引用的有效期 → [第17课：生命周期](17_lifetimes.md)
