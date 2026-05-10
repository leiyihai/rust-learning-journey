// ex14_error_handling.rs — 第14课：错误处理
//
// 目标：
// 1. 区分可恢复错误（Result）和不可恢复错误（panic!）
// 2. 掌握 match 处理 Result
// 3. 使用 ? 运算符传播错误
use std::fs;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    // ==================== 任务 1：panic! 宏 ====================
    // 试试 panic! 的效果（在 debug 和 release 模式下各跑一次）
    // panic!("程序崩溃！");
    // let test = [1, 2, 3, 4, 5];
    // test[99];

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：Result 类型 ====================
    // 实现 divide 函数（在文件末尾定义）
    // 用 match 调用它，分别测试 10/2 和 10/0 两种情况
    // 提示：返回 Result<f64, String>
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：? 运算符 ====================
    // 实现 read_file 函数（在文件末尾定义）
    // 用 ? 运算符读取文件内容，用 match 测试"读取 Cargo.toml"和"读取不存在的文件"
    // 提示：File::open(path)? 然后用 .read_to_string(&mut s)?
    match read_file("Cargo.toml") {
        Ok(content) => println!("读取成功! 内容：\n{}", content),
        Err(e) => println!("读取失败：{}", e),
    }

    match read_file("不存在的文件.txt") {
        Ok(content) => println!("读取成功! 内容：\n{}", content),
        Err(e) => println!("读取失败：{}", e),
    }

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：unwrap 和 expect ====================
    // 试试 unwrap 和 expect 的区别
    let unwrap1 = fs::read_to_string(r#"C:\Users\WIN11\Desktop\G.txt"#).unwrap();
    println!("{}", unwrap1);
    // fs::read_to_string(r#"C:\Users\WIN11\Desktop\不存在的文本.txt"#).unwrap(); // 直接panic,怎么看报错是不是panic

    println!("--- 任务 4 结束 ---");
}

// ==================== 需要完成的函数 ====================

fn divide(a: f64, b: f64) -> Result<f64, String> {
    match b == 0.0 {
        true => Err("除数不能为0".to_string()),
        false => Ok(a / b),
    }
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// ==================== 思考题 ====================
// 1. panic! 和 Result 分别适用于什么场景？
// panic!用于原型和测试,Result用于处理可接受的可预知的错误。
// 2. ? 运算符只能在返回 Result 的函数中使用吗？
// 是的，不是说是语法糖嘛，本质还是返回Option<T>
// 3. unwrap() 在生产代码中应该避免使用，为什么？
// 一旦得到Err就会panic!

// 运行方法：cargo run --bin ex14_error_handling
