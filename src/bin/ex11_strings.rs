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


    // 1. String::from("...")
    // 2. "...".to_string()
    // 3. "...".into()
    // 打印出来验证
    println!("{}", String::from("创建String方式一"));
    println!("{}", "创建String方式二".to_string());
    let s:String= "创建String方式三".into();
    println!("{}", s);

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：字符串拼接 ====================
    // TODO: 用 + 运算符连接两个字符串
    // 注意：+ 会获取第一个字符串的所有权！
    let hello = String::from("你好");
    let world = String::from("世界");

    let hello_world = hello + &world;  // 填上正确的写法
    println!("{hello_world}");
    // println!("{hello}");  // 编译错误：hello 的所有权被移走了

    // TODO: 用 format! 宏拼接（推荐方式，不会转移所有权）
    let name = "Alice";
    let age = 20;
    let info = format!("我叫{name}，今年{age}岁");  // 生成 "我叫 Alice，今年 20 岁"
    println!("{info}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：字符串遍历（UTF-8 警告！）====================
    let text = String::from("你好Rust");

    // Rust 的字符串是 UTF-8 编码，每个字符可能占 1~4 字节
    // .chars() 按"字符"遍历（Unicode 标量值）
    // .bytes() 按"字节"遍历


    println!("按字符遍历 {}:", text);
    for c in text.chars() { println!("{c}") }

    println!("按字节遍历 {}:", text);
    for b in text.bytes() { println!("{b}") }

    // 观察："你" 一个字符占了几个字节？
    // 将text赋值成"你" 输出228 189 160  3个字节


    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：字符串常用方法 ====================
    let s = String::from("  Rust Programming!  ");


    println!("转大写: {}", s.to_uppercase());
    println!("转小写: {}", s.to_lowercase());
    println!("去除空白: '{}'", s.trim());
    println!("包含 'Rust'? {}", s.contains("Rust"));
    println!("替换: {}", s.replace("Rust", "Type"));
    println!("是否以 ' ' 开头? {}", s.starts_with("  "));

    println!("--- 任务 4 结束 ---");
    let s1 = first_letter(&String::from("我要"));
    match s1 { Some(c) => println!("{}", c),None => println!("空") };
    let s2 = first_letter(&String::from("RUST"));
    match s2 { Some(c) => println!("{}", c),None => println!("空") };
}

// ==================== 任务 5（挑战）：提取首字母 ====================
// 写一个函数 first_letter，接受 &str，返回 Option<char>
// 如果字符串为空，返回 None
// 如果第一个字符是多字节的（如中文），也要正确处理！

fn first_letter(s: &str) -> Option<char> {
    s.chars().next()
}

// ==================== 思考题 ====================
// 1. 为什么 Rust 要区分 String 和 &str？这体现了什么设计理念？
// 区分可变字符串和不可变字符串？可以根据需求将字符串放在堆和栈中。体现极致的性能？瞎猜的我也不知道
// 2. 为什么 Rust 不让你直接通过索引访问字符串 s[0]？
// 因为Rust用的Unicode标准，索引访问会出问题，访问到字符的某个字节而不是那个字符
// 3. format! 宏和 println! 宏有什么相似之处？
// 都是用来格式化字符串的

// 运行方法：cargo run --bin ex11_strings
