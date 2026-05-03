// ex17_lifetimes.rs — 第17课：生命周期（Lifetimes）
//
// 目标：
// 1. 理解生命周期的作用：防止悬垂引用
// 2. 掌握生命周期注解语法
// 3. 了解生命周期省略规则（lifetime elision）

fn main() {
    // ==================== 任务 1：悬垂引用 ====================
    // Rust 编译器防止你写出如下代码。取消注释看看错误信息：

    // let r;
    // {
    //     let x = 5;
    //     r = &x;  // x 活不到外面！
    // }
    // println!("{r}");  // 编译错误：x 的寿命不够长

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：生命周期注解 ====================
    // 当函数涉及多个引用时，编译器需要知道它们的关系

    // TODO: 完成 longest 函数（在文件末尾定义）
    // 它接受两个 &str，返回较长的那个

    let s1 = String::from("短");
    let s2 = String::from("长字符串");
    // let result = longest(&s1, &s2);
    // println!("较长的字符串: {result}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：结构体中的生命周期 ====================
    // 如果结构体包含引用，就必须标注生命周期

    // TODO: 完成 ImportantExcerpt 结构体，它持有对 str 的引用

    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }

    // let novel = String::from("第一章：开始。很久很久以前……");
    // let first_sentence = novel.split('。').next().unwrap();
    // let excerpt = ImportantExcerpt { part: first_sentence };
    // println!("摘录: {}", excerpt.part);

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：生命周期省略规则 ====================
    // Rust 有三条省略规则，大多数情况下你不需要写生命周期

    // 规则 1：每个引用参数都有独立的生命周期
    fn first_word(s: &str) -> &str {  // 等价于 fn first_word<'a>(s: &'a str) -> &'a str
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

    // 为什么这里不用写生命周期？因为省略规则自动补全了。

    println!("--- 任务 4 结束 ---");
}

// ==================== 需要完成的函数 ====================

// TODO: 完成 longest 函数，返回两个字符串切片中较长的那个
// 需要注意：生命周期注解'a 告诉编译器，返回值的生命周期
// 和两个参数中较短的那个一样
// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() { x } else { y }
// }

// 试试看：如果去掉生命周期注解 'a，编译器会怎么报错？
// 读读错误信息，它说得非常清楚！

// ==================== 思考题 ====================
// 1. 生命周期注解会改变代码的运行行为吗？（提示：不会，它只是给编译器的"证明"）
// 2. 什么时候需要手动写生命周期？什么时候可以省略？
// 3. 'static 生命周期是什么意思？
// 4. 如果 longest 函数只需要返回第一个参数，还需要生命周期注解吗？

// 运行方法：cargo run --bin ex17_lifetimes
