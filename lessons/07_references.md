# 第7课：引用与借用

## 本课目标

- 理解 `&`（共享引用）和 `&mut`（可变引用）
- 掌握借用规则
- 了解切片类型

## 引用：不获取所有权的访问

如果你只想**使用**一个值而不**拥有**它，可以用引用：

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calc_length(&s1);   // &s1 创建了一个引用
    println!("{s1} 的长度是 {len}");  // s1 仍然有效！
}

fn calc_length(s: &String) -> usize {
    // s 只是借用了 String，不会在函数结束时释放它
    s.len()
}
```

这种操作叫做**借用**（borrowing）：就像你借一本书，看完要还回去。

## 可变引用

默认引用是**不可变的**。如果你需要修改借用的值：

```rust
let mut s = String::from("hello");
change(&mut s);  // 创建可变引用

fn change(s: &mut String) {
    s.push_str(", world");  // 可以修改
}
```

## 借用规则（核心！记住它！）

**在同一作用域内**：

1. 要么有**一个**可变引用
2. 要么有**任意多个**不可变引用
3. **不能同时**拥有可变和不可变引用

```rust
let mut s = String::from("hello");

let r1 = &s;        // ✓
let r2 = &s;        // ✓ (两个不可变引用没问题)
let r3 = &mut s;    // ❌ 不能同时有不可变和可变引用！
```

```rust
let mut s = String::from("hello");

let r1 = &mut s;    // ✓
let r2 = &mut s;    // ❌ 不能有两个可变引用！
```

## 为什么有这些规则？

这是为了防止**数据竞争**（data race）：
- 两个或多个指针同时访问同一数据
- 至少有一个在写入
- 没有同步机制

数据竞争会导致未定义行为——Rust 在编译时就消除了这种可能性。

## 引用必须始终有效

Rust 编译器保证不会出现**悬垂引用**（dangling reference）：

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ❌ s 会在函数结束时被释放，返回的引用悬空！
}
```

## 切片（Slice）

切片是对集合中**部分元素**的引用：

```rust
let s = String::from("hello world");

let hello = &s[0..5];    // "hello"
let world = &s[6..11];   // "world"

// 语法糖
let hello = &s[..5];     // 从开头到索引 5
let world = &s[6..];     // 从索引 6 到末尾
let whole = &s[..];      // 整个字符串
```

切片的类型是 `&str`，也叫"字符串切片"。

## 练习指南

打开 `src/bin/ex07_references.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex07_references`

---

**下一步**：用结构体组织相关数据 → [第8课：结构体](08_structs.md)
