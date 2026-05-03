# 第18课：闭包与迭代器

## 本课目标

- 理解闭包的语法和用途
- 掌握 Fn 系列 trait
- 熟练使用迭代器方法链

## 闭包（Closure）

闭包是**可以捕获环境的匿名函数**：

```rust
let x = 10;
let add_x = |n: i32| -> i32 { n + x };  // 捕获了 x

println!("{}", add_x(5));  // 15
```

闭包语法可以逐步简化：

```rust
fn  add_one_v1   (x: i32) -> i32 { x + 1 }  // 普通函数
let add_one_v2 = |x: i32| -> i32 { x + 1 }; // 完整闭包
let add_one_v3 = |x|             { x + 1 }; // 省略类型
let add_one_v4 = |x|               x + 1  ; // 省略大括号
```

## 闭包捕获环境

闭包有三种捕获方式（由编译器根据闭包体自动推断）：

| Trait | 捕获方式 | 说明 |
|-------|----------|------|
| `FnOnce` | 获取所有权 | 消费捕获的变量（只能调用一次） |
| `FnMut` | `&mut` 借用 | 修改捕获的变量 |
| `Fn` | `&` 借用 | 只读捕获的变量 |

```rust
let mut count = 0;
let mut inc = || { count += 1; println!("{count}"); };
// inc 是 FnMut，因为修改了 count
inc();  // 1
inc();  // 2
```

## 把闭包传给函数

```rust
fn apply<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let triple = |x| x * 3;
println!("{}", apply(triple, 7));  // 21
```

## 迭代器（Iterator）

迭代器是 Rust 中处理序列的核心抽象，**惰性**执行：

```rust
let v = vec![1, 2, 3, 4, 5];

// 创建迭代器（还没执行）
let iter = v.iter();

// 消费迭代器（触发执行）
let sum: i32 = iter.sum();
```

## 迭代器方法链

这是 Rust 中非常强大的编程模式：

```rust
let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

let result: Vec<i32> = nums.iter()
    .filter(|&&x| x % 2 == 0)  // 过滤偶数
    .map(|&x| x * 3)            // 每个乘以 3
    .take(3)                    // 只要前 3 个
    .collect();                 // 收集到 Vec
// 结果：[6, 12, 18]
```

## 常用迭代器方法

| 方法 | 作用 | 类型 |
|------|------|------|
| `map` | 转换每个元素 | 适配器 |
| `filter` | 过滤元素 | 适配器 |
| `take` | 取前 n 个 | 适配器 |
| `skip` | 跳过前 n 个 | 适配器 |
| `enumerate` | 添加索引 | 适配器 |
| `sum` | 求和 | 消费者 |
| `collect` | 收集到集合 | 消费者 |
| `fold` | 累加/累乘 | 消费者 |
| `any` / `all` | 存在/全部满足 | 消费者 |
| `find` | 查找第一个 | 消费者 |
| `count` | 计数 | 消费者 |

**适配器**返回迭代器（惰性），**消费者**触发执行。

## 练习指南

打开 `src/bin/ex18_closures.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex18_closures`

---

**下一步**：保证代码正确 → [第19课：测试](19_testing.md)
