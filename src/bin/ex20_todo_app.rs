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
//
// 你将用到的知识：
// - 结构体 + impl（TodoList）
// - Vec<T>（存储任务）
// - String 操作
// - 模式匹配（match）
// - 控制流（loop）
// - 标准 I/O（stdin/stdout）
// - 错误处理

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
        TodoItem {
            id,
            title,
            completed: false,
        }
    }
}

/// 待办事项列表
struct TodoList {
    items: Vec<TodoItem>,
    next_id: u32,
}

impl TodoList {
    // TODO: 实现 new() —— 创建空的 TodoList
    fn new() -> Self {
        todo!("返回 TodoList，items 初始为空 Vec，next_id 从 1 开始")
    }

    // TODO: 实现 add —— 添加一个任务
    fn add(&mut self, _title: String) {
        todo!("创建一个 TodoItem，push 到 items，next_id 加 1")
    }

    // TODO: 实现 list —— 列出所有任务
    fn list(&self) {
        todo!("遍历 items，用 status 标记 [✓] 或 [ ]，打印每个任务")
    }

    // TODO: 实现 done —— 将指定 id 的任务标记为完成
    fn done(&mut self, _id: u32) {
        todo!("查找 id 对应的任务，设置 completed = true，找不到打印提示")
    }

    // TODO: 实现 remove —— 删除指定 id 的任务
    fn remove(&mut self, _id: u32) {
        todo!("查找 id 对应的任务，用 Vec::remove 删除，找不到打印提示")
    }
}

fn main() {
    let mut todo_list = TodoList::new();

    println!("=== Rust 待办事项管理器 ===");
    println!("命令: add <描述> | list | done <id> | remove <id> | help | quit");

    loop {
        // 打印提示符
        print!("> ");
        let _ = io::stdout().flush();

        // TODO: 读取用户输入
        let mut input = String::new();
        // io::stdin().read_line(&mut input).expect("读取输入失败");

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
                // TODO: 提取 parts[1] 中任务描述，调用 todo_list.add()
                // 如果没有描述，打印用法提示
            }

            "done" => {
                // TODO: 提取 id 参数，调用 todo_list.done()
            }

            "remove" => {
                // TODO: 提取 id 参数，调用 todo_list.remove()
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
// 1. 保存和加载：把 tasks 保存到 JSON 文件，启动时加载
//    提示：用 serde_json crate
// 2. 任务优先级：每个任务有高/中/低优先级
// 3. 截止日期：每个任务有可选的截止日期
// 4. 排序：按优先级、日期、完成状态排序
// 5. 编辑：修改已有任务的描述

// ==================== 恭喜！====================
// 如果你完成了这个项目，你已经掌握了 Rust 的核心概念！

// 运行方法：cargo run --bin ex20_todo_app
