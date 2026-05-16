use std::io;

// 单个代办任务
#[derive(Debug)]
struct Todoitem{
    id: u32,
    title: String,
    completed: bool,
}

// 任务列表管理器
#[derive(Debug)]
struct TodoList {
    items: Vec<Todoitem>,
    next_id: u32, // 自增id
}
impl TodoList {
    // 创建空的任务列表
    fn new() -> Self {
        TodoList{
            items: Vec::new(),
            next_id: 1,
        }
    }

    // 添加任务
    fn add(&mut self, title: String) {
        let item = Todoitem{
            id: self.next_id,
            title,
            completed: false,
        };
        self.items.push(item);
        self.next_id += 1;
        println!("✅ 任务添加成功");
    }

    // 列出所有任务
    fn list(&self) {
        if self.items.is_empty(){
            println!("📭 暂无待办任务");
        }
        println!("==================== 任务列表 ====================");
        for item in &self.items {
            let status = if item.completed { "✅ 已完成" } else { "🔸 未完成" };
            println!("ID:{} | {} | 内容:{}", item.id, status, item.title);
        }
        println!("=================================================");
    }

    // 标记完成任务
    fn done(&mut self, target_id: u32) {
        match self.items.iter_mut().find(|x|x.id == target_id) {
            Some(item) => {
                item.completed = true;
                println!("🎉 任务{}已标记完成", target_id);
            },
            None => println!("❌ 不存在该ID任务"),
        }
    }

    // 删除任务
    fn remove(&mut self, target_id: u32) {
        let before_length = self.items.len();
        self.items.retain(|x|x.id != target_id);
        if before_length > self.items.len(){
            println!("🗑️  任务{}删除成功", target_id);
        }else {
            println!("❌ 不存在该ID任务");
        }
    }
}

// 打印帮助命令
fn print_help() {
    println!("============= 命令列表 =============");
    println!("add 内容      - 添加新待办任务");
    println!("list          - 查看所有任务");
    println!("done 编号     - 标记任务完成");
    println!("remove 编号   - 删除指定任务");
    println!("help          - 查看帮助");
    println!("quit          - 退出程序");
    println!("====================================");
}

fn main() {
    // 初始化任务管理器
    let mut todo_list = TodoList::new();
    println!("===== 简易待办管理器 =====");
    println!("输入 help 查看所有命令\n");

    loop{
        // 定义储存输入的字符串
        let mut input = String::new();
        println!("请输入指令：");

        // 读取控制台输入
        io::stdin().read_line(&mut input).expect("读取输入失败");

        // 去除首尾空格、换行符
        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        // 分割命令
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts[0];

        // 匹配所有命令
        match cmd {
            "quit" => {
                println!("👋 退出程序，再见！");
                break;
            },
            "help" => print_help(),
            "list" => todo_list.list(),
            "add" => {
                if parts.len() < 2 {
                    println!("⚠️ 用法：add 任务描述");
                }else {
                    let content = parts[1..].join(" ");
                    todo_list.add(content);
                }
            }
            "done" => {
                if parts.len() < 2 {
                    println!("⚠️ 用法：done 任务ID");
                }else if let Ok(id) = parts[1].parse::<u32>(){
                    todo_list.done(id);
                }else {
                    println!("⚠️ ID必须是数字");
                }
            }
            "remove" => {
                if parts.len() != 2 {
                    println!("⚠️ 用法：remove 任务ID");
                } else if let Ok(id) = parts[1].parse::<u32>() {
                    todo_list.remove(id);
                } else {
                    println!("⚠️ ID必须是数字");
                }
            }
            _ => println!("❌ 未知命令，输入 help 查看帮助"),
        }
        println!();
    }
}