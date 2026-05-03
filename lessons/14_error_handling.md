# 第14课：错误处理

## 本课目标

- 区分可恢复错误（`Result`）和不可恢复错误（`panic!`）
- 掌握 `match` 处理 `Result`
- 使用 `?` 运算符传播错误

## Rust 的两种错误

| 类型 | 工具 | 场景 |
|------|------|------|
| 不可恢复 | `panic!` | 程序无法继续（如数组越界） |
| 可恢复 | `Result<T, E>` | 合理的失败（如文件不存在） |

## panic!

```rust
panic!("程序崩溃");  // 立即终止当前线程

// 常见触发 panic 的情况：
let v = vec![1, 2, 3];
v[99];  // 索引越界 → panic
```

在 debug 模式下，Rust 默认开启栈展开，可以看到完整的调用栈。

## Result<T, E>

```rust
enum Result<T, E> {
    Ok(T),    // 成功，包含返回值
    Err(E),   // 失败，包含错误信息
}
```

```rust
use std::fs::File;

let f = File::open("hello.txt");  // 返回 Result<File, io::Error>

match f {
    Ok(file) => println!("文件打开成功"),
    Err(error) => println!("文件打开失败: {error}"),
}
```

## ? 运算符

`?` 是 Rust 最常用的错误处理方式：

```rust
use std::fs;
use std::io;

fn read_file() -> Result<String, io::Error> {
    let content = fs::read_to_string("hello.txt")?;  // 如果 Err，立即返回
    Ok(content)
}
```

等价于：

```rust
fn read_file() -> Result<String, io::Error> {
    let content = match fs::read_to_string("hello.txt") {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok(content)
}
```

`?` 可以链式使用：

```rust
fs::read_to_string("hello.txt")?.trim().to_string();
```

## unwrap 和 expect

```rust
let f = File::open("a.txt").unwrap();       // Ok 则取值，Err 则 panic
let f = File::open("a.txt").expect("打开失败");  // 同上，但可以自定义消息
```

这两个方法应该**谨慎使用**（原型开发、测试中可以用），生产代码中更推荐 `match` 或 `?`。

## 什么时候 panic？什么时候用 Result？

- **用 Result**：函数的使用者可以合理处理失败（如文件不存在）
- **用 panic**：出现了不应该发生的情况（如内部状态不一致）

## 练习指南

打开 `src/bin/ex14_error_handling.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex14_error_handling`

---

**下一步**：消除重复代码 → [第15课：泛型](15_generics.md)
