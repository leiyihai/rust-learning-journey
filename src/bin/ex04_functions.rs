// ex04_functions.rs — 第4课：函数
//
// 目标：
// 1. 掌握函数定义和调用
// 2. 理解参数和返回值
// 3. 区分语句和表达式

// ==================== 任务 1：编写你的第一个函数 ====================
// TODO: 完成 greet 函数，接受 &str 类型的名字，打印问候语
fn greet(name: &str) {
    todo!("在函数体中写一行 println!，输出 \"你好，{name}！\"")
}

// ==================== 任务 2：带返回值的函数 ====================
// TODO: 完成 add 函数，接受两个 i32，返回它们的和
fn add(a: i32, b: i32) -> i32 {
    todo!("返回 a + b，注意不要加分号！")
}

// ==================== 任务 3：提前返回 ====================
// TODO: 完成 is_even 函数，判断一个数是否为偶数
fn is_even(n: i32) -> bool {
    todo!("如果 n 除以 2 的余数是 0，返回 true，否则返回 false")
}

fn main() {
    // 调用任务 1 的函数
    greet("Rust 学习者");

    // 调用任务 2 的函数
    let result = add(3, 5);
    println!("3 + 5 = {result}");

    // 调用任务 3 的函数
    println!("4 是偶数吗？{}", is_even(4));
    println!("7 是偶数吗？{}", is_even(7));

    // ==================== 任务 4（挑战）：摄氏度转华氏度 ====================
    // TODO: 取消下面的注释，完成 celsius_to_fahrenheit 函数
    // 公式：F = C * 9/5 + 32
    // let f = celsius_to_fahrenheit(100.0);
    // println!("100°C = {f}°F");
}

// TODO: 完成这个函数
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    todo!("返回 celsius * 9.0 / 5.0 + 32.0")
}

// ==================== 思考题 ====================
// 1. 语句和表达式的区别是什么？为什么 Rust 要区分它们？
// 2. Rust 函数返回值可以不写 return，这有什么好处？

// 运行方法：cargo run --bin ex04_functions
