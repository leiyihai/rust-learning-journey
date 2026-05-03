# 第19课：测试

## 本课目标

- 编写单元测试
- 使用断言宏
- 理解 `#[cfg(test)]` 和 `#[test]`

## 测试文件位置

Rust 鼓励把测试代码放在**和源代码一起**：

```rust
// src/lib.rs (或任何源文件)

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]   // 只在 cargo test 时编译
mod tests {     // 惯例：测试模块命名为 tests
    use super::*;  // 导入父模块的所有公开项

    #[test]    // 标注测试函数
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

## 运行测试

```bash
cargo test                      # 运行所有测试
cargo test --bin ex19_testing   # 运行特定二进制文件的测试
cargo test test_add             # 只运行名称包含 test_add 的测试
cargo test -- --nocapture       # 显示测试中的 println 输出
```

## 断言宏

```rust
assert!(condition);                // 断言为真
assert!(condition, "自定义消息");   // 带消息

assert_eq!(left, right);           // 断言相等
assert_eq!(left, right, "msg");    // 带消息

assert_ne!(left, right);           // 断言不等
```

## 常用断言模式

```rust
// 布尔
assert!(is_even(4));

// 相等
assert_eq!(add(2, 3), 5);

// Option
assert_eq!(safe_divide(10.0, 2.0), Some(5.0));
assert_eq!(safe_divide(10.0, 0.0), None);

// Result
assert!(result.is_ok());
assert!(result.is_err());

// 字符串
assert_eq!(reverse("hello"), "olleh");
```

## #[should_panic]

测试函数应该 panic：

```rust
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_out_of_bounds() {
    let v = vec![1, 2, 3];
    let _x = v[99];  // 这会 panic
}
```

`expected` 参数是可选的，但加上它可以确保 panic 是**你期望的那个原因**。

## 集成测试

集成测试放在项目根目录的 `tests/` 文件夹下，每个 `.rs` 文件都是一个独立的 crate：

```
project/
├── src/
│   └── lib.rs
└── tests/
    └── integration_test.rs
```

```rust
// tests/integration_test.rs
use my_project::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

## 练习指南

打开 `src/bin/ex19_testing.rs`，完成里面的 TODO 任务。

**注意**：这个练习的重点不是 `cargo run`，而是 `cargo test`！

运行测试：`cargo test --bin ex19_testing`

---

**下一步**：综合实战 → [第20课：命令行待办事项应用](20_todo_app.md)
