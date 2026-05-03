# 第13课：哈希映射（HashMap<K,V>）

## 本课目标

- 创建和初始化 `HashMap`
- 掌握插入、访问、更新操作
- 了解强大的 `entry` API

## 什么是 HashMap？

`HashMap<K,V>` 存储**键值对**的映射关系，通过键快速查找值。

```rust
use std::collections::HashMap;  // 需要手动导入！

let mut scores = HashMap::new();

scores.insert(String::from("蓝队"), 10);
scores.insert(String::from("红队"), 15);
```

## 创建方式

```rust
// 空的 HashMap
let mut map: HashMap<String, i32> = HashMap::new();

// 从迭代器创建
let teams = vec![String::from("蓝队"), String::from("红队")];
let scores = vec![10, 15];
let map: HashMap<_, _> = teams.into_iter().zip(scores).collect();
```

## 访问值

```rust
let team = String::from("蓝队");
let score = map.get(&team);  // 返回 Option<&V>

match score {
    Some(v) => println!("蓝队得了 {v} 分"),
    None => println!("蓝队不存在"),
}
```

## 更新策略

```rust
// 1. 覆盖
map.insert(String::from("蓝队"), 25);  // 覆盖旧值

// 2. 只在键不存在时插入
map.entry(String::from("黄队")).or_insert(12);

// 3. 基于旧值更新
let count = map.entry(String::from("蓝队")).or_insert(0);
*count += 1;  // 累加
```

## entry API（重点！）

`entry` 是处理 HashMap 更新的利器：

```rust
// 统计词频
let text = "hello world hello";
let mut word_count = HashMap::new();

for word in text.split_whitespace() {
    let count = word_count.entry(word).or_insert(0);
    *count += 1;  // 如果是新词插入 0 然后 +1
                  // 如果是旧词就 +1
}
```

`entry` 返回 `Entry` 枚举：
- `Occupied(entry)` —— 键已存在，可以读取/修改
- `Vacant(entry)` —— 键不存在，可以插入新值

## HashMap 和所有权

```rust
let key = String::from("最喜欢的颜色");
let value = String::from("蓝色");

let mut map = HashMap::new();
map.insert(key, value);
// println!("{key}");  // ❌ key 和 value 的所有权被移入 HashMap

// 对于实现了 Copy 的类型（如 i32），值会被复制
```

## 练习指南

打开 `src/bin/ex13_hashmaps.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex13_hashmaps`

---

**下一步**：优雅地处理错误 → [第14课：错误处理](14_error_handling.md)
