// ex02_variables.rs — 第2课：变量与可变性
//
// 目标：
// 1. 掌握 let 绑定
// 2. 理解 mut 可变性
// 3. 了解常量和变量遮蔽（shadowing）

fn main() {
    // ==================== 任务 1：不可变变量 ====================
    let name = "Rust";
    println!("我正在学习 {}！", name);

    // 下面这行代码取消注释后，编译器会报错。为什么？
    // name = "Python";
    // 试试看，阅读编译器的错误信息。

    // ==================== 任务 2：可变变量 ====================
    // 用 let mut 声明一个可变变量 counter，初始值为 0
    // 然后让它加 1，再打印出来
    // TODO: 完成下面的代码
    // let mut counter = ???;
    // counter = counter + 1;
    // println!("counter = {}", counter);

    // ==================== 任务 3：变量遮蔽（Shadowing）====================
    let value = 5;
    println!("初始 value = {}", value);

    // 下面的代码用新的 let 声明"遮蔽"了之前的 value
    // TODO: 取消注释，观察输出
    // let value = value + 3;
    // println!("遮蔽后 value = {}", value);

    // Shadowing 甚至可以改变类型！
    // TODO: 取消注释，让 value 从数字变成字符串
    // let value = format!("数字是 {}", value);
    // println!("{}", value);

    // ==================== 任务 4：常量 ====================
    // 用 const 声明一个常量 MAX_SCORE，值为 100
    // 注意：常量必须标注类型，且不能是运行时计算的值
    // TODO:
    // const MAX_SCORE: u32 = ???;
    // println!("满分是 {}", MAX_SCORE);

    // ==================== 思考题 ====================
    // 1. let mut 和 shadowing 都能"改变"变量的值，它们的本质区别是什么？
    // 2. 什么时候用 const，什么时候用 let？
}

// 运行方法：cargo run --bin ex02_variables
