# 第17课：生命周期（Lifetimes）

## 本课目标

- 理解生命周期的作用：防止悬垂引用
- 掌握生命周期注解语法
- 了解生命周期省略规则

## 生命周期解决什么问题？

生命周期确保**引用在需要时始终有效**，防止悬垂引用：

```rust
let r;
{
    let x = 5;
    r = &x;  // ❌ x 活不到外面
}
println!("{r}");  // 悬垂引用！编译器阻止了这段代码
```

## 生命周期注解

当函数涉及多个引用时，需要生命周期注解来告诉编译器它们之间的关系：

```rust
// 语法：<'a> 类似于泛型
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

这里 `'a` 的含义是：**返回值的生命周期是两个参数中较短的那个**。

```rust
let s1 = String::from("短");
let s2 = String::from("长字符串");
let result = longest(&s1, &s2);  // 'a = min(s1的生命周期, s2的生命周期)
```

## 生命周期注解不会改变行为

生命周期注解只是给编译器的"证明"，**不影响编译后的代码**。编译器用它来检查：
- 引用是否在需要时还活着
- 引用之间是否存在合理的关系

## 结构体中的生命周期

```rust
// 如果结构体包含引用，必须标注生命周期
struct Excerpt<'a> {
    part: &'a str,  // 这个引用必须活得比结构体长
}

let novel = String::from("第一章...");
let first = novel.split('。').next().unwrap();
let excerpt = Excerpt { part: first };  // OK: first 和 novel 活得够长
```

## 生命周期省略规则

大多数情况下你不需要写生命周期，Rust 有三条自动推断规则：

1. **每个引用参数都获得独立的生命周期**
2. **如果只有一个输入生命周期，它被自动赋给所有输出**
3. **如果 `&self` 或 `&mut self` 是参数，它的生命周期赋给所有输出**

```rust
// 你写的是：
fn first_word(s: &str) -> &str { ... }

// 编译器自动推断为：
fn first_word<'a>(s: &'a str) -> &'a str { ... }
```

## 静态生命周期

`'static` 是整个程序运行期间都存在：

```rust
let s: &'static str = "我是字符串字面量";  // 存在二进制中，贯穿整个程序
```

## 练习指南

打开 `src/bin/ex17_lifetimes.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex17_lifetimes`

---

**下一步**：函数式编程风格 → [第18课：闭包与迭代器](18_closures.md)
