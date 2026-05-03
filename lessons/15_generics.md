# 第15课：泛型（Generics）

## 本课目标

- 理解泛型的作用：用同一套逻辑处理不同类型
- 编写泛型函数和泛型结构体
- 了解单态化（Monomorphization）

## 为什么需要泛型？

没有泛型时，你需要为每种类型写一遍相同的逻辑：

```rust
fn largest_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
fn largest_f64(a: f64, b: f64) -> f64 { if a > b { a } else { b } }
fn largest_char(a: char, b: char) -> char { if a > b { a } else { b } }
```

用泛型，只需要写一个函数：

```rust
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}
```

## 泛型函数

```rust
// T 是类型参数（Type Parameter）
fn identity<T>(x: T) -> T {
    x
}

// 使用
let a = identity(5);        // T = i32
let b = identity("hello");  // T = &str
```

## 泛型结构体

```rust
struct Point<T> {
    x: T,
    y: T,
}

let p1 = Point { x: 5, y: 10 };       // Point<i32>
let p2 = Point { x: 1.2, y: 3.4 };    // Point<f64>
```

多个类型参数：

```rust
struct Pair<K, V> {
    key: K,
    value: V,
}
```

## 泛型方法

```rust
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
```

## 单态化（Monomorphization）

Rust 在**编译时**为每个使用的具体类型生成独立的代码：

```rust
let a = identity(5);       // 编译器生成 identity_i32
let b = identity(3.14);    // 编译器生成 identity_f64
```

这是**零成本抽象**——泛型在运行时没有任何性能开销，和手写具体类型一样快。

对比：
- Java/C#：泛型在运行时擦除类型
- C++ 模板：也是编译时展开，但没有 trait 约束
- Rust：编译时单态化 + trait 约束 = 安全 + 高效

## 泛型栈（Stack<T>）示例

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
}
```

## 练习指南

打开 `src/bin/ex15_generics.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex15_generics`

---

**下一步**：抽象共享行为 → [第16课：特性](16_traits.md)
