# 第5课：控制流

## 本课目标

- 掌握 `if` / `else if` / `else`
- 理解 `if` 是表达式（可以返回值）
- 掌握三种循环：`loop`、`while`、`for`

## if / else

```rust
let number = 6;

if number % 4 == 0 {
    println!("能被 4 整除");
} else if number % 3 == 0 {
    println!("能被 3 整除");
} else if number % 2 == 0 {
    println!("能被 2 整除");
} else {
    println!("不能被 4、3、2 整除");
}
```

**注意**：条件必须是 `bool` 类型，不会自动转换：

```rust
let x = 3;
if x {  // ❌ 编译错误！x 是 i32，不是 bool
    println!("x 是三");
}
```

## if 是表达式

在 Rust 中，`if` 可以**返回值**：

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

println!("数值是 {number}");  // 5
```

每个分支必须返回**相同类型**：

```rust
let x = if true { 5 } else { "six" };  // ❌ 类型不匹配
```

## loop 循环

`loop` 会无限循环，直到遇到 `break`：

```rust
let mut count = 0;
let result = loop {
    count += 1;
    if count == 10 {
        break count * 2;  // break 可以带返回值！
    }
};
println!("结果: {result}");  // 20
```

## while 循环

```rust
let mut number = 3;
while number != 0 {
    println!("{number}...");
    number -= 1;
}
println!("发射！");
```

## for 循环

`for` 是 Rust 中最常用的循环，用于遍历集合：

```rust
let a = [10, 20, 30, 40, 50];

// 遍历元素
for element in a {
    println!("{element}");
}

// 遍历范围的推荐方式
for number in (1..4).rev() {  // .rev() 反转
    println!("{number}...");  // 3, 2, 1
}
```

**范围语法**：
- `1..4` —— 包含 1, 2, 3（不含 4）
- `1..=4` —— 包含 1, 2, 3, 4（含 4）

## 循环总结

| 循环 | 适用场景 |
|------|----------|
| `loop` | 需要无限循环或 break 带值 |
| `while` | 基于条件的循环 |
| `for` | 遍历集合（**最常用**） |

## 练习指南

打开 `src/bin/ex05_control_flow.rs`，完成里面的 TODO 任务。FizzBuzz 挑战不要跳过！

运行：`cargo run --bin ex05_control_flow`

---

**下一步**：理解 Rust 最核心的概念 → [第6课：所有权](06_ownership.md)
