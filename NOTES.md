# 踩坑日志

> 学习过程中遇到的概念难点、错误和心得。随学随记。

---

## 概念辨析

### 语句 vs 表达式（ex04）

- **语句**：不返回值，以 `;` 结尾（`let x = 5;` 是语句）
- **表达式**：返回值，无 `;`（`a + b` 是表达式）
- Rust 的 `if`、`loop`、代码块 `{}` 都是表达式，可以直接赋值
- 这就是为什么 Rust 不需要三元运算符：`let x = if cond { a } else { b };`

### println! 的 ln

`ln` = line，和 C# `Console.WriteLine()` 对应，`print!` = `Console.Write()`。

### `for x in (1..4)` 不是遍历数组

`(1..4)` 是 **Range 类型**，自己就能生成序列。和数组无关，不需要先定义一个数组再遍历。

`1..4` = [1, 4)（不包含 4），`1..=4` = [1, 4]（包含 4）。

### 判断偶数的正确写法（ex04）

❌ `n / 2 == 0` — 除法，判断商是否为 0（4/2=2，2≠0，误判为非偶数）
✅ `n % 2 == 0` — 取余，判断余数是否为 0

### 所有权：Move 在做什么

`let s2 = s1;` 不是复制数据，只是在栈上复制了 ptr + len + cap（24字节），堆上数据纹丝不动。然后 s1 被标记为失效。所以 Move 很快。

### String vs &str 的本质区别

- **String**：拥有者。栈结构体（ptr + len + cap）指向堆数据。离开作用域时释放堆内存。
- **&str**：借用者。栈结构体（ptr + len），没有 cap。离开作用域时什么也不释放。

`&String` 几乎不用，总是用 `&str` 更通用。

### 为什么 `&String` 能传给 `&str`？（Deref 强制转换）

```rust
fn bar(s: &str) {
    println!("{s}");
}

let s = String::from("hello");
bar(&s);  // &s 是 &String 类型，为什么可以传进去？
```

Rust 编译器会自动做 **Deref 强制转换**（Deref coercion）：

```rust
// 你写的：
bar(&s);

// 编译器自动做的（等价于）：
bar(&s[..]);          // 取 String 的全部切片
// 或者理解为自动调了 String.deref() → &str
```

所以 `&str` 参数能接收三种东西：

```rust
bar("字面量");            // &str → 直接匹配
bar(&String::from("x")); // &String → deref 自动转 &str
bar(&s[0..3]);           // 切片 → &str
```

一个签名兼容所有情况。Deref trait 会在第 16 课深入学习。

### 悬垂引用（Dangling Reference）

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s  // ❌ s 在函数结束时被释放，返回的引用指向已释放内存
}
```

编译器会自动阻止这种代码。

---

## UTF-8 字节与切片

### 各字符占用字节数

| 字符类型 | 示例 | 字节数 |
|----------|------|--------|
| 英文/数字 | `a`, `1` | 1 |
| 常见中文 | `你`, `好` | 3 |
| 常见 emoji | `🐶`, `🦀` | 4 |
| 特殊 emoji | `👨‍👩‍👧` | 可能更多（组合序列） |

### 安全的切片方式

**永远别用字节索引去切混编字符串！**

```rust
let s = "Rust🦀中文";

// ❌ 危险：手动算字节容易切到字符中间 → panic
let bad = &s[0..5];  // 如果正好切在 emoji 中间就崩了

// ✅ 安全：用 .chars() 按字符操作
let good: String = s.chars().take(5).collect();  // "Rust🦀"
let good: String = s.chars().skip(2).take(3).collect();  // 第3~5个字符
```

### 中文字符串切片踩坑（ex07）

```rust
let s = "你好，世界！";

// ❌ s[0..3] → 只取了"你"（一个中文字 = 3 字节，所以 0..3 只是一个字）
// ✅ s[0..6] → 取了"你好"（两个字 = 6 字节）
```

结论：别自己算字节，用 `.chars()`。

---

## 借用规则速记

- 同一作用域内：
  - 可以有**多个**不可变引用 `&T`
  - 只能有**1 个**可变引用 `&mut T`
  - **不能**同时有可变和不可变引用
- 引用的作用域到**最后一次使用**为止，之后可以创建新引用
- 这些规则是为了在**编译时**消除数据竞争

---

## Debug vs Release

```bash
cargo run --bin ex03_types            # debug 模式（默认，溢出会 panic）
cargo run --bin ex03_types --release  # release 模式（溢出回绕，255+1=0）
```

---

## 杂项

- `print!` 输出后不换行，`println!` 输出后换行
- Cargo.toml 中 `[[bin]]` 每行声明一个可执行文件，`cargo run --bin xxx` 运行
- 函数指定了返回类型但空函数体会编译失败，用 `todo!()` 占位可先让代码编译通过
