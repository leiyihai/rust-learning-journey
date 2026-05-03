// ex14_error_handling.rs — 第14课：错误处理
//
// 目标：
// 1. 区分可恢复错误（Result）和不可恢复错误（panic!）
// 2. 掌握 match 处理 Result
// 3. 使用 ? 运算符传播错误

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // ==================== 任务 1：panic! 宏 ====================
    // 当程序遇到无法恢复的错误时使用
    // 取消注释试试：
    // panic!("程序崩溃！");  // 试试在 debug 和 release 模式下的区别

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：Result 类型 ====================
    // Result<T, E>:
    //   Ok(T)  — 操作成功，包含返回值
    //   Err(E) — 操作失败，包含错误信息

    // TODO: 完成 divide 函数（在本文件末尾定义）
    // 然后取消下面的注释

    // match divide(10.0, 2.0) {
    //     Ok(result) => println!("10 / 2 = {result}"),
    //     Err(e) => println!("错误: {e}"),
    // }

    // match divide(10.0, 0.0) {
    //     Ok(result) => println!("10 / 0 = {result}"),
    //     Err(e) => println!("错误: {e}"),
    // }

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：? 运算符 ====================
    // ? 是 Rust 中最常用的错误处理方式
    // 如果 Result 是 Ok，提取值；如果是 Err，立即从函数返回

    // TODO: 完成 read_file 函数（使用 ? 运算符）
    // match read_file("Cargo.toml") {
    //     Ok(contents) => println!("Cargo.toml 的内容:\n{contents}"),
    //     Err(e) => println!("读取失败: {e}"),
    // }

    // match read_file("不存在的文件.txt") {
    //     Ok(contents) => println!("{contents}"),
    //     Err(e) => println!("读取失败: {e}"),
    // }

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：unwrap 和 expect ====================
    // 在确定不会出错时使用，或用于快速原型

    let definitely_twelve: Result<i32, &str> = Ok(12);
    let value = definitely_twelve.unwrap();  // 如果是 Err 会 panic
    println!("unwrap: {value}");

    // TODO: 试试 expect（可以自定义 panic 消息）
    // let x: Result<i32, &str> = Err("出错了");
    // let value = x.expect("计算失败：");  // 观察 panic 消息

    println!("--- 任务 4 结束 ---");
}

// TODO: 实现 divide(a: f64, b: f64) -> Result<f64, String>
// 如果 b == 0.0，返回 Err
// 否则返回 Ok(a / b)

// TODO: 实现 read_file(path: &str) -> Result<String, io::Error>
// 用 ? 运算符读取文件内容
// 提示：用 File::open(path)? 和 read_to_string

// ==================== 思考题 ====================
// 1. panic! 和 Result 分别适用于什么场景？
// 2. ? 运算符只能在返回 Result 的函数中使用吗？
// 3. unwrap() 在生产代码中应该避免使用，为什么？

// 运行方法：cargo run --bin ex14_error_handling
