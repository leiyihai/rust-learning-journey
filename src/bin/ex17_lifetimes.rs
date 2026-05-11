// ex17_lifetimes.rs — 第17课：生命周期（Lifetimes）
//
// 目标：
// 1. 理解生命周期的作用：防止悬垂引用
// 2. 掌握生命周期注解语法
// 3. 了解生命周期省略规则（lifetime elision）

fn main() {
    // ==================== 任务 1：悬垂引用 ====================
    // 试试写出一个悬垂引用的场景，观察编译错误
    // 提示：在内部作用域创建变量，在外部作用域引用它

    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // x活的不够长
    // }
    // println!("{:?}", r);

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：生命周期注解 ====================
    // 实现 longest 函数（在文件末尾），返回两个 &str 中较长的那个
    // 在 main 中创建两个 String，调用 longest 测试
    // 提示：函数签名需要生命周期注解 'a

    let s1 = String::from("短");
    let s2 = String::from("长字符串");

    let result = longest(&s1, &s2);
    println!("较长的字符串: {result}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：结构体中的生命周期 ====================
    // 定义一个 ImportantExcerpt 结构体，持有对 str 的引用
    // 从一段文本中提取第一句话，存入该结构体
    // 提示：结构体含引用时必须标注生命周期

    struct ImportantExcerpt<'a>{
        x: &'a str,
    }

    let text = String::from("你好！Rust!");
    let x = ImportantExcerpt{x: &text[0..6]};
    println!("{}", x.x);

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：生命周期省略规则 ====================
    // 下面的 first_word 函数为什么不需要写生命周期注解？
    // 因为s是从函数外引用的，引用的s生命周期大于函数。
    // 试着写几个符合省略规则的函数，验证编译通过

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    let test = "Hello World";
    println!("第一个单词: {}", first_word(test));

    println!("--- 任务 4 结束 ---");
}

// ==================== 需要完成的函数 ====================

// 提示：fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}


// ==================== 思考题 ====================
// 1. 生命周期注解会改变代码的运行行为吗？
// 不会
// 2. 什么时候需要手动写生命周期？什么时候可以省略？
// 函数中有多个引用参数的时候。只有一个引用传参或者传参的是&self本身
// 3. 'static 生命周期是什么意思？
// 全局声明周期，程序在声明的'static的变量就在。
// 4. 如果 longest 函数只需要返回第一个参数，还需要生命周期注解吗？
// 需要,因为生命周期不看函数内部实现，只看函数签名。

// 运行方法：cargo run --bin ex17_lifetimes
