// ex05_control_flow.rs — 第5课：控制流
//
// 目标：
// 1. 掌握 if/else if/else
// 2. 理解 if 是表达式
// 3. 掌握三种循环：loop、while、for

fn main() {
    // ==================== 任务 1：if/else ====================
    // TODO: 根据 age 判断是否可以投票（>= 18）
    let age = 20;

    // 补充 if 条件
    if age >= 18 {
        println!("你可以投票！");
    } else {
        println!("你还不能投票。");
    }

    // ==================== 任务 2：if 是表达式 ====================
    // 在 Rust 中，if 可以返回值！下面用 if 表达式给 status 赋值
    // TODO: 补全
    let score = 85;
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else {
        'D'
    };
    println!("分数 {score} → 等级 {grade}");

    // ==================== 任务 3：loop 循环 ====================
    // TODO: 用 loop 实现一个计数器，从 1 数到 5，然后 break
    // 提示：break 可以带返回值！

    let mut count = 0;
    let result = loop{
        count += 1;
        println!("{}", count);
        if count == 5 { break count;}
    };
    println!("循环了{}次", result);

    // let mut count = 0;
    // let result = loop {
    //     count += 1;
    //     if count == 5 {
    //         break count * 2;  // break 带返回值
    //     }
    // };
    // println!("loop 结果: {result}");

    // ==================== 任务 4：while 循环 ====================
    // TODO: 用 while 实现从 5 倒数到 1

    let mut countdown = 5;
    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1;
    }

    // println!("倒数开始！");
    // let mut n = 5;
    // while n > 0 {
    //     println!("{n}...");
    //     n -= 1;
    // }
    // println!("发射！");

    // ==================== 任务 5：for 循环 ====================
    // TODO: 用 for 循环遍历数组 colors
    let colors = ["红", "橙", "黄", "绿", "蓝", "靛", "紫"];

    for color in colors {
        println!("{}", color);
    }

    // ==================== 任务 6（挑战）：FizzBuzz ====================
    // 用 for 循环遍历 1..=100（包含 100）
    // - 能被 3 整除：输出 "Fizz"
    // - 能被 5 整除：输出 "Buzz"
    // - 能被 3 和 5 整除：输出 "FizzBuzz"
    // - 否则输出数字本身

    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 { println!("FizzBuzz"); }
        else if i % 3 == 0 { println!("Fizz"); }
        else if i % 5 == 0 { println!("Buzz"); }
        else { println!("{}", i); }

    }

    // ==================== 思考题 ====================
    // 1. loop、while、for 分别适用于什么场景？
    // loop和while适用于不知道循环次数但知道什么时候结束循环的时候使用，for适用于固定长度的数组元组等
    // 2. for i in 1..=10 和 for i in 1..10 有什么区别？
    //左闭右开  for i in 1..=10是遍历从1到10   for i in 1..10是遍历从1到9
}

// 运行方法：cargo run --bin ex05_control_flow
