# 第12课：向量（Vec<T>）

## 本课目标

- 创建和初始化 `Vec<T>`
- 掌握增删改查操作
- 遍历 Vec 的各种方式

## 什么是 Vec？

`Vec<T>` 是 Rust 中的**动态数组**——可以在运行时增长或缩小的列表。

```rust
let v: Vec<i32> = Vec::new();  // 空 Vec
let v = vec![1, 2, 3];         // vec! 宏创建
let v = vec![0; 5];            // [0, 0, 0, 0, 0]
```

## 增删改查

```rust
let mut v = vec![1, 2, 3];

// 增
v.push(4);           // 追加到末尾 → [1, 2, 3, 4]
v.insert(0, 0);      // 插入到指定位置 → [0, 1, 2, 3, 4]

// 删
let last = v.pop();  // 移除最后一个元素 → Some(4)
v.remove(1);         // 移除索引 1 的元素 → [0, 2, 3]

// 查
let third = v[2];              // 索引访问（越界会 panic）
let third = v.get(2);          // 安全访问 → Some(3)
let third = v.get(100);        // → None（不 panic）

// 改
v[0] = 100;                    // 直接赋值
if let Some(first) = v.get_mut(0) {
    *first = 200;              // 需要解引用
}
```

## [] vs get()

| 方式 | 越界时 | 返回值 |
|------|--------|--------|
| `v[i]` | **panic!** | `T` |
| `v.get(i)` | 返回 None | `Option<&T>` |

推荐用 `get()`，因为它强制你处理可能越界的情况，更安全。

## 遍历

```rust
let v = vec![10, 20, 30];

// 不可变遍历
for n in &v {
    println!("{n}");
}

// 可变遍历
let mut v = v;
for n in &mut v {
    *n *= 2;  // 每个元素翻倍
}

// 带索引
for (i, n) in v.iter().enumerate() {
    println!("v[{i}] = {n}");
}
```

## Vec 和所有权

```rust
let v = vec![String::from("hello")];
let s = v[0];  // ❌ 不能移出 Vec 中的元素
```

要移出元素，用 `v.remove(0)` 或 `v.pop()`。

## 练习指南

打开 `src/bin/ex12_vectors.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex12_vectors`

---

**下一步**：键值存储 → [第13课：哈希映射](13_hashmaps.md)
