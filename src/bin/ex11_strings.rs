// ex11_strings.rs — 第11课：字符串
//
// 目标：
// 1. 理解 String 和 &str 的区别
// 2. 掌握字符串的常见操作
// 3. 了解 UTF-8 编码的影响

fn main() {
    // ==================== 任务 1：String vs &str ====================
    // &str: 字符串切片，通常是借用的、不可变的引用
    // String: 堆上分配的、可增长的、拥有的字符串

    let literal: &str = "Hello, world!";  // 字符串字面量，存在二进制中
    let mut owned: String = String::from("Hello");  // 堆上的 String
    owned.push_str(", Rust!");  // 可以修改

    println!("字面量: {literal}");
    println!("String: {owned}");

    // TODO: 创建一个 String，用 3 种不同的方式
    // 1. String::from("...")
    // 2. "...".to_string()
    // 3. "...".into()
    // 打印出来验证

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：字符串拼接 ====================
    // TODO: 用 + 运算符连接两个字符串
    // 注意：+ 会获取第一个字符串的所有权！
    let hello = String::from("你好");
    let world = String::from("世界");

    // let hello_world = hello + ???;  // 填上正确的写法
    // println!("{hello_world}");
    // println!("{hello}");  // 编译错误：hello 的所有权被移走了

    // TODO: 用 format! 宏拼接（推荐方式，不会转移所有权）
    let name = "Alice";
    let age = 20;
    // let info = format!(???);  // 生成 "我叫 Alice，今年 20 岁"
    // println!("{info}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：字符串遍历（UTF-8 警告！）====================
    let text = String::from("你好Rust");

    // Rust 的字符串是 UTF-8 编码，每个字符可能占 1~4 字节
    // .chars() 按"字符"遍历（Unicode 标量值）
    // .bytes() 按"字节"遍历

    println!("按字符遍历 {}:", text);
    // TODO: 用 .chars() 遍历 text，打印每个字符
    // for c in text.chars() { ... }

    println!("按字节遍历 {}:", text);
    // TODO: 用 .bytes() 遍历 text，打印每个字节
    // for b in text.bytes() { ... }

    // 观察："你" 一个字符占了几个字节？

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：字符串常用方法 ====================
    let s = String::from("  Rust Programming!  ");

    // TODO: 试试这些方法，取消注释观察结果
    // println!("转大写: {}", s.to_uppercase());
    // println!("转小写: {}", s.to_lowercase());
    // println!("去除空白: '{}'", s.trim());
    // println!("包含 'Rust'? {}", s.contains("Rust"));
    // println!("替换: {}", s.replace("Rust", "Type"));
    // println!("是否以 ' ' 开头? {}", s.starts_with("  "));

    println!("--- 任务 4 结束 ---");
}

// ==================== 任务 5（挑战）：提取首字母 ====================
// 写一个函数 first_letter，接受 &str，返回 Option<char>
// 如果字符串为空，返回 None
// 如果第一个字符是多字节的（如中文），也要正确处理！

// fn first_letter(s: &str) -> Option<char> {
//     // TODO: 用 s.chars().next()
// }

// ==================== 思考题 ====================
// 1. 为什么 Rust 要区分 String 和 &str？这体现了什么设计理念？
// 2. 为什么 Rust 不让你直接通过索引访问字符串 s[0]？
// 3. format! 宏和 println! 宏有什么相似之处？

// 运行方法：cargo run --bin ex11_strings
