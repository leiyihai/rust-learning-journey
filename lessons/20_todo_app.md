# 第20课：综合项目 —— 命令行待办事项

## 本课目标

构建一个完整的命令行待办事项管理器，综合运用前面所学的全部知识。

## 功能需求

| 命令 | 功能 |
|------|------|
| `add <描述>` | 添加新任务 |
| `list` | 列出所有任务（显示完成状态） |
| `done <id>` | 标记任务为完成 |
| `remove <id>` | 删除任务 |
| `help` | 显示帮助 |
| `quit` | 退出程序 |

## 你将用到的知识

| 知识 | 用在哪里 |
|------|----------|
| 结构体 + `impl` | `TodoItem` 和 `TodoList` |
| `Vec<T>` | 存储任务列表 |
| `String` 操作 | 读取输入、分割命令 |
| `match` | 解析命令 |
| `loop` | 主程序循环 |
| 标准 I/O | `stdin()` / `stdout()` |
| `if let` | 解析可选参数 |

## 项目设计

```
待办事项管理器
├── TodoItem { id, title, completed }
│   ├── new(id, title) → TodoItem
│   └── (将来可扩展：优先级、截止日期)
│
└── TodoList { items, next_id }
    ├── new() → TodoList
    ├── add(title)
    ├── list()
    ├── done(id)
    └── remove(id)
```

## 提示

1. **先设计，再编码**：数据结构定下来后，逐个实现方法
2. **小步前进**：实现一个功能就测试一个
3. **用 Rust 的方式思考**：
   - 用 `match` 处理所有可能的命令
   - 用 `Option` 表示可能没有的参数
   - 用 `&self` / `&mut self` 控制访问权限

## 挑战扩展（可选）

完成基础功能后，试试以下扩展：

1. **持久化**：用 `serde_json` 将任务保存为 JSON 文件
2. **优先级**：每个任务有高/中/低优先级
3. **排序**：按优先级、创建时间排序
4. **编辑**：修改已有任务
5. **彩色输出**：用 `colored` crate

## 扩展涉及的 Crate

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
colored = "2"
```

---

**恭喜！这是学习路线的最后一课。**

完成这个项目后，你已经掌握了 Rust 的核心概念，可以自信地探索更深入的方向：
- 并发编程（`std::thread`、`Arc<Mutex<T>>`、`mpsc`）
- 异步编程（`async`/`await`、`tokio`）
- Web 开发（`actix-web`、`axum`）
- 嵌入式开发（`no_std`、`embedded-hal`）

## 练习指南

打开 `src/bin/ex20_todo_app.rs`，完成里面的 TODO 任务。

运行：`cargo run --bin ex20_todo_app`

---

**总目录**：[返回学习路线图](../README.md)
