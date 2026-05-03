// ex06_ownership.rs — 第6课：所有权（Ownership）
//
// 这是 Rust 最核心、最独特的概念！
//
// 所有权三大规则：
// 1. Rust 中每一个值都有一个所有者（owner）
// 2. 同一时间只能有一个所有者
// 3. 当所有者离开作用域，值会被自动丢弃（释放内存）
//
// 目标：理解 move、clone、Copy trait

fn main() {
    // ==================== 任务 1：移动（Move）====================
    // s1 拥有字符串 "hello" 的所有权
    let s1 = String::from("hello");

    // TODO: 取消下面的注释，观察编译错误
    // let s2 = s1;  // ← 所有权从 s1 移动到了 s2
    // println!("{s1}");  // 编译错误！s1 已经失效了
    // println!("{s2}");  // 这是可以的

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：克隆（Clone）====================
    // TODO: 如果你真的需要一份"深拷贝"，可以用 clone()
    let original = String::from("Rust");
    // let copy = original.???();  // 填上正确的方法名
    // println!("original: {original}, copy: {copy}");  // 两个都可以用

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：Copy Trait ====================
    // 整数等简单类型自动实现了 Copy trait，
    // 所以赋值时不会发生移动，而是复制
    let x = 42;
    let y = x;   // x 被复制了一份，没有移动
    println!("x = {x}, y = {y}");  // 两个都可以用

    // TODO: 试试把上面的 String 换成 i32，观察区别
    // 思考：为什么 String 需要 clone()，而 i32 不需要？

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：函数与所有权 ====================
    // TODO: 完成 take_ownership 和 give_ownership 两个函数
    // 然后取消 main 中的调用代码

    // let message = String::from("你好");
    // take_ownership(message);  // message 的所有权被移动到函数里
    // println!("{message}");  // 编译错误！message 已失效

    // let new_msg = give_ownership();  // 函数返回一个新 String 的所有权
    // println!("新消息: {new_msg}");

    println!("--- 任务 4 结束 ---");
}

// TODO: 实现 take_ownership —— 接收一个 String，打印它
// 函数结束后，这个 String 被 drop（释放）

// TODO: 实现 give_ownership —— 创建并返回一个新的 String
// 所有权从函数内部移动给调用者

// ==================== 思考题 ====================
// 1. 为什么 Rust 要设计所有权系统？（提示：想想 C++ 的 double free 和 GC 的开销）
// 2. 哪些类型实现了 Copy trait？哪些没有？规律是什么？
// 3. 如果把一个 String 传给函数后又想继续使用它，有哪几种办法？

// 运行方法：cargo run --bin ex06_ownership
