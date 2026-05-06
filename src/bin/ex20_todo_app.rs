// ex20_todo_app.rs — 第20课：综合项目 / 命令行待办事项
//
// 这是学习路线的收官项目！你将综合运用前面所学的知识，
// 构建一个完整的命令行待办事项管理器。
//
// 功能需求：
// 1. 添加任务（add）
// 2. 列出所有任务（list）
// 3. 完成任务（done）
// 4. 删除任务（remove）
// 5. 退出程序（quit）

use std::io::{self, Write};

// ==================== 数据结构 ====================

/// 单个待办事项
#[derive(Debug, Clone)]
struct TodoItem {
    id: u32,
    title: String,
    completed: bool,
}

impl TodoItem {
    fn new(id: u32, title: String) -> Self {
        TodoItem { id, title, completed: false }
    }
}

/// 待办事项列表
struct TodoList {
    items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    // TODO: 实现 new() —— 创建空的 TodoList（items = vec![], next_id = 1）
    fn new() -> Self {
        todo!("创建空的 TodoList")
    }

    // TODO: 实现 add —— 用 title 创建 TodoItem，push 到 items，next_id += 1
    fn add(&mut self, _title: String) {
        todo!("添加新任务")
    }

    // TODO: 实现 list —— 遍历 items，[✓] 已完成 [ ] 未完成
    fn list(&self) {
        todo!("列出所有任务")
    }

    // TODO: 实现 done —— 找到 id 对应的任务，设置 completed = true
    fn done(&mut self, _id: u32) {
        todo!("标记任务为完成")
    }

    // TODO: 实现 remove —— 找到 id 对应的任务，删除它
    fn remove(&mut self, _id: u32) {
        todo!("删除任务")
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    println!("=== Rust 待办事项管理器 ===");
    println!("命令: add <描述> | list | done <id> | remove <id> | help | quit");

    loop {
        print!("> ");
        let _ = io::stdout().flush();

        // TODO: 用 io::stdin().read_line(&mut input) 读取一行输入
        let mut input = String::new();

        // 解析命令
        let parts: Vec<&str> = input.trim().splitn(2, ' ').collect();
        let command = parts.first().map(|s| s.to_lowercase()).unwrap_or_default();

        match command.as_str() {
            "" => continue,

            "help" => {
                println!("  add <描述>  —  添加新任务");
                println!("  list       —  列出所有任务");
                println!("  done <id>  —  标记任务为完成");
                println!("  remove <id> — 删除任务");
                println!("  help       —  显示此帮助");
                println!("  quit       —  退出程序");
            }

            "list" => {
                todo_list.list();
            }

            "add" => {
                // TODO: 从 parts 中提取描述，调用 todo_list.add()
            }

            "done" => {
                // TODO: 从 parts 中提取 id 并解析，调用 todo_list.done()
            }

            "remove" => {
                // TODO: 从 parts 中提取 id 并解析，调用 todo_list.remove()
            }

            "quit" => {
                println!("再见！");
                break;
            }

            _ => {
                println!("未知命令: '{command}'，输入 'help' 查看帮助");
            }
        }
    }
}

// ==================== 挑战扩展（可选）====================
// 1. 保存和加载：用 serde_json 将任务保存为 JSON 文件
// 2. 任务优先级：高/中/低
// 3. 截止日期
// 4. 排序：按优先级、日期、状态排序

// 运行方法：cargo run --bin ex20_todo_app
