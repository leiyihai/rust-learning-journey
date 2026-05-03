// ex07_references.rs — 第7课：引用与借用
//
// 目标：
// 1. 理解 & (共享引用) 和 &mut (可变引用)
// 2. 掌握借用规则
// 3. 了解切片
//
// 借用规则：
// - 同一时间，要么一个可变引用，要么任意多个共享引用
// - 引用必须始终有效（不能悬垂）

fn main() {
    // ==================== 任务 1：不可变引用 ====================
    let s1 = String::from("hello");
    let len = calc_length(&s1);  // &s1 创建了一个指向 s1 的引用
                                  // s1 的所有权没有被移动！
    println!("'{s1}' 的长度是 {len}");  // s1 仍然可用

    // 如果一个函数接受 &T 而非 T，称为"借用"（borrowing）

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：可变引用 ====================
    // TODO: 完成 append_world 函数，让它给字符串追加 " world"
    // 然后取消下面的注释
    let mut message = String::from("hello");
    // append_world(&mut message);  // &mut 创建可变引用
    // println!("修改后: {message}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：借用规则实践 ====================
    // TODO: 取消下面的注释，观察编译错误
    // 这违反了"不能同时有可变引用和不可变引用"的规则

    // let mut data = String::from("Rust");
    // let r1 = &data;       // 不可变引用
    // let r2 = &mut data;   // 可变引用 — 编译器会报错！
    // println!("{r1} {r2}");

    // 正确的做法：引用的使用范围不重叠
    let mut data = String::from("Rust");
    let r1 = &data;
    println!("{r1}");   // r1 在这里最后一次使用
    // r1 的作用域到此为止
    let r2 = &mut data;  // 现在可以创建可变引用了
    r2.push_str("acean");
    println!("{r2}");

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：字符串切片 ====================
    // TODO: 用切片获取字符串 "你好，世界！" 的前两个字（注意 UTF-8！）
    let greeting = String::from("你好，世界！");
    // let hello = &greeting[???];  // 填正确的字节范围
    // println!("切片: {hello}");

    println!("--- 任务 4 结束 ---");
}

// TODO: 实现 calc_length —— 接受 &String，返回 usize
// 不要获取所有权，只是借用
fn calc_length(s: &String) -> usize {
    todo!("调用 s.len() 返回字符串的字节长度")
}

// TODO: 实现 append_world —— 接受 &mut String，追加内容
fn append_world(_s: &mut String) {
    todo!("调用 _s.push_str(\" world\")")
}

// ==================== 思考题 ====================
// 1. 为什么 Rust 不允许同时存在可变引用和不可变引用？
// 2. 什么是"悬垂引用"（dangling reference）？Rust 如何防止它？
// 3. &String 和 &str 有什么区别？什么时候用哪个？

// 运行方法：cargo run --bin ex07_references
