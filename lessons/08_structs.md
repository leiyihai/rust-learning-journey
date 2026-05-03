# 第8课：结构体（Structs）

## 本课目标

- 定义和实例化结构体
- 实现结构体方法（`impl` 块）
- 理解关联函数（"静态方法"）

## 定义结构体

```rust
struct User {
    name: String,       // 注意：用逗号分隔
    email: String,
    age: u8,
    active: bool,
}
```

## 创建实例

```rust
let user1 = User {
    name: String::from("Alice"),
    email: String::from("alice@example.com"),
    age: 25,
    active: true,
};
```

注意：Rust **没有** `new` 关键字。结构体的实例化直接写。

## 字段访问和修改

```rust
println!("{}", user1.name);  // 访问

let mut user1 = user1;  // 整个实例需要是 mut 才能修改字段
user1.age = 26;         // 修改
```

## 字段初始化简写

```rust
fn build_user(name: String, email: String) -> User {
    User {
        name,      // 等价于 name: name
        email,     // 等价于 email: email
        age: 20,
        active: true,
    }
}
```

## 结构体更新语法

```rust
let user2 = User {
    name: String::from("Bob"),
    ..user1  // 其余字段从 user1 复制
};
// 注意：这会移动 user1 的 String 字段（email），
// 所以 user1 此后不能使用
```

## 元组结构体

没有命名字段的结构体：

```rust
struct Color(u8, u8, u8);
struct Point(i32, i32);

let red = Color(255, 0, 0);
let origin = Point(0, 0);

println!("R = {}", red.0);  // 用索引访问
```

## 方法（impl 块）

```rust
impl User {
    // 方法：第一个参数是 &self
    fn say_hello(&self) {
        println!("你好，我是 {}", self.name);
    }

    // 关联函数：没有 self 参数（类似"静态方法"）
    fn new(name: String, email: String) -> User {
        User { name, email, age: 0, active: true }
    }
}

// 调用
user1.say_hello();                    // 方法
let user = User::new("Bob".into(), "b@e.com".into());  // 关联函数
```

| 语法 | 含义 |
|------|------|
| `&self` | 不可变借用，只读 |
| `&mut self` | 可变借用，可修改 |
| `self` | 获取所有权（罕见） |

## 练习指南

打开 `src/bin/ex08_structs.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex08_structs`

---

**下一步**：用枚举描述可选值 → [第9课：枚举与模式匹配](09_enums.md)
