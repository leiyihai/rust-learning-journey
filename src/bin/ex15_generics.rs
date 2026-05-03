// ex15_generics.rs — 第15课：泛型（Generics）
//
// 目标：
// 1. 理解泛型的作用：用同一套逻辑处理不同类型
// 2. 泛型函数、泛型结构体
// 3. 单态化（Monomorphization）：零成本抽象

// ==================== 任务 1：泛型函数 ====================
// TODO: 完成 largest 泛型函数，返回两个值中较大的那个
// 提示：需要 PartialOrd trait（可比较大小）

// fn largest<T: PartialOrd>(a: T, b: T) -> T {
//     if a > b { a } else { b }
// }

fn main() {
    // println!("largest(3, 7) = {}", largest(3, 7));
    // println!("largest(3.14, 2.71) = {}", largest(3.14, 2.71));
    // println!("largest('a', 'z') = {}", largest('a', 'z'));

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：泛型结构体 ====================
    // TODO: 完成 Point<T> 泛型结构体
    // 然后取消下面的代码

    // struct Point<T> {
    //     x: T,
    //     y: T,
    // }

    // let p1 = Point { x: 5, y: 10 };
    // let p2 = Point { x: 1.2, y: 3.4 };
    // println!("整数点: ({}, {})", p1.x, p1.y);
    // println!("浮点数点: ({}, {})", p2.x, p2.y);

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：泛型 + 方法 ====================
    // 为 Point<T> 实现一个 mixup 方法
    // 它将两个不同类型的 Point 混合成一个

    // TODO: 完成 Point 和 mixup 的实现

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：多个泛型参数 ====================
    // 定义一个 Pair<T, U> 结构体，包含两个不同类型的值
    // 为它实现一个 swap 方法，返回 Pair<U, T>

    // TODO: 完成 Pair 和 swap

    println!("--- 任务 4 结束 ---");
}

// ==================== 任务 5（挑战）：泛型栈 ====================
// 实现一个简易的泛型栈 Stack<T>
// 方法：new(), push(), pop() -> Option<T>, peek() -> Option<&T>, is_empty() -> bool
// 底层用 Vec<T> 存储

// struct Stack<T> {
//     items: Vec<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self { ... }
//     fn push(&mut self, item: T) { ... }
//     fn pop(&mut self) -> Option<T> { ... }
//     fn peek(&self) -> Option<&T> { ... }
//     fn is_empty(&self) -> bool { ... }
// }

// ==================== 思考题 ====================
// 1. Rust 的泛型和 Java/C++ 的泛型有什么不同？（提示：单态化）
// 2. 泛型中的 T 必须在编译时确定吗？为什么？
// 3. 单态化的优势和劣势分别是什么？

// 运行方法：cargo run --bin ex15_generics
