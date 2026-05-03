# 第11课：字符串

## 本课目标

- 理解 `String` 和 `&str` 的区别
- 掌握字符串的常见操作
- 了解 UTF-8 编码对字符串处理的影响

## 两种字符串类型

| 类型 | 存储位置 | 可变 | 所有权 |
|------|----------|------|--------|
| `&str` | 栈/二进制 | 不可变 | 借用 |
| `String` | 堆 | 可变 | 拥有 |

```rust
let literal: &str = "hello";             // 字符串字面量，&str
let owned: String = String::from("hello"); // 堆上的 String
```

**核心理解**：
- `&str` 是"别人拥有的字符串的切片"，你只是借用
- `String` 是你拥有的字符串，可以随意修改

## 创建 String

```rust
let s1 = String::new();                // 空字符串
let s2 = String::from("hello");        // 从字面量
let s3 = "hello".to_string();          // to_string()
let s4: String = "hello".into();       // into()
```

## 字符串拼接

```rust
// + 运算符（会获取第一个字符串的所有权）
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 被移动了！s3 = "Hello, world!"
// println!("{s1}");  // ❌ s1 已失效

// format! 宏（推荐，不获取所有权）
let s1 = String::from("Hello");
let s2 = String::from("world");
let s3 = format!("{s1}, {s2}!");  // s1 和 s2 都还可以用
```

> `+` 的签名是 `fn add(self, s: &str) -> String`，所以第二个参数需要 `&String`（会自动解引用为 &str）

## 字符串不可索引访问

Rust 不允许 `s[0]` 这样的方式访问字符串：

```rust
let s = String::from("你好");
// let c = s[0];  // ❌ 编译错误！
```

**原因**：UTF-8 编码中一个字符可能占多个字节。`s[0]` 可能返回一个无效的字节。

## UTF-8 遍历

```rust
let text = String::from("你好Rust");

// 按字符遍历
for c in text.chars() {
    println!("{c}");
}

// 按字节遍历
for b in text.bytes() {
    println!("{b}");
}
```

- "你" 占 3 个字节，.chars() 正确返回一个字符
- .bytes() 会返回原始字节序列

## 常用方法

```rust
let s = String::from("  Rust  ");

s.len()           // 字节长度
s.is_empty()      // 是否为空
s.trim()          // 去除首尾空白 → &str
s.to_uppercase()  // 转大写
s.contains("Ru")  // 是否包含
s.replace("R", "T") // 替换
s.split_whitespace() // 按空白分割
```

## 练习指南

打开 `src/bin/ex11_strings.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex11_strings`

---

**下一步**：存储列表数据 → [第12课：向量](12_vectors.md)
