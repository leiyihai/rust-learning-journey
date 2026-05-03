# 第4课：函数

## 本课目标

- 掌握函数定义和调用
- 理解参数和返回值
- 区分**语句**和**表达式**

## 函数定义

```rust
fn function_name(param1: Type1, param2: Type2) -> ReturnType {
    // 函数体
}
```

Rust 使用**蛇形命名法**（snake_case）：所有字母小写，单词用下划线分隔。

```rust
fn greet(name: &str) {
    println!("你好，{}！", name);
}

fn main() {
    greet("Alice");  // 调用函数
}
```

## 参数

函数参数**必须**标注类型：

```rust
fn add(a: i32, b: i32) {   // ✓ 正确
    println!("{}", a + b);
}

fn add(a, b) {              // ❌ 编译错误！
    println!("{}", a + b);
}
```

## 语句和表达式（重要！）

这是 Rust 区别于其他语言的关键概念：

- **语句**（Statement）—— 执行操作，**不返回值**，以分号结尾
- **表达式**（Expression）—— 计算并**返回值**，没有分号

```rust
let x = 5;          // 语句（let 语句不返回值）
5 + 6;              // 语句（表达式 + 分号 = 语句）
5 + 6               // 表达式（没有分号，返回值 11）
```

你不能写 `let x = (let y = 6);`，这在 Rust 中不合法，因为 `let` 是语句，不返回值。

而在 C 语言中 `x = y = 6` 是合法的（赋值是表达式）。

## 返回值

函数的返回值是**最后一个表达式的值**：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // 没有分号！这是表达式，作为返回值
}
```

也可以用 `return` 提前返回：

```rust
fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true;
    }
    false  // 最后一个表达式，不需要 return
}
```

## 练习指南

打开 `src/bin/ex04_functions.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex04_functions`

---

**下一步**：让程序做判断 → [第5课：控制流](05_control_flow.md)
