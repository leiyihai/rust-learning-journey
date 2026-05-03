# 第9课：枚举与模式匹配

## 本课目标

- 定义枚举类型
- 掌握 `match` 表达式
- 了解 `if let` 语法糖
- 认识 `Option<T>` —— 消除空指针的关键

## 枚举定义

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

let light = TrafficLight::Red;
```

Rust 的枚举更强大：**每个成员可以携带不同类型的数据**！

```rust
enum Message {
    Quit,                          // 不携带数据
    Write(String),                 // 携带 String
    ChangeColor(u8, u8, u8),      // 携带三个 u8
    Move { x: i32, y: i32 },      // 携带命名字段
}
```

## match 表达式

`match` 是 Rust 中最强大的控制流结构：

```rust
fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停下！",
        TrafficLight::Yellow => "准备～",
        TrafficLight::Green => "通行！",
    }
}
```

`match` 是**穷尽的**（exhaustive）—— 必须覆盖所有可能的分支，否则编译不通过。

## 匹配带数据的枚举

```rust
fn process(msg: Message) {
    match msg {
        Message::Quit => println!("退出"),
        Message::Write(text) => println!("消息: {text}"),
        Message::ChangeColor(r, g, b) => println!("颜色: ({r},{g},{b})"),
        Message::Move { x, y } => println!("移动到 ({x},{y})"),
    }
}
```

## Option<T>——没有 null 的世界！

Rust **没有** null！替代方案是 `Option<T>` 枚举：

```rust
enum Option<T> {
    None,      // 表示"没有值"
    Some(T),   // 包含一个类型为 T 的值
}
```

`Option<T>` 和 `<T>` 是**不同的类型**，你不能把 `Option<T>` 直接当 `T` 使用：

```rust
let x: i32 = 5;
let y: Option<i32> = Some(10);

let z = x + y;  // ❌ 编译错误！类型不匹配
```

编译器强制你处理 `None` 的情况——这是 Rust 安全保证的关键。

```rust
let maybe_value: Option<i32> = Some(42);

match maybe_value {
    Some(v) => println!("值是 {v}"),
    None => println!("没有值"),
}
```

## if let 语法糖

当你只关心一种匹配模式时，`if let` 更简洁：

```rust
let maybe_name = Some(String::from("Alice"));

// 等价于 match 但更简洁
if let Some(name) = maybe_name {
    println!("名字是 {name}");
}
```

## 练习指南

打开 `src/bin/ex09_enums.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex09_enums`

---

**下一步**：组织代码结构 → [第10课：模块系统](10_modules.md)
