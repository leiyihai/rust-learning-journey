// ex18_closures.rs — 第18课：闭包与迭代器
//
// 目标：
// 1. 理解闭包的语法和用途
// 2. 掌握 Fn 系列 trait
// 3. 熟练掌握迭代器方法链（map/filter/fold/collect）

fn main() {
    // ==================== 任务 1：闭包基础 ====================
    // 闭包是匿名函数，可以捕获环境中的变量
    let x = 10;

    // 完整的闭包语法：
    let add_x = |n: i32| -> i32 { n + x };  // 捕获了 x
    println!("5 + 10 = {}", add_x(5));

    // 简写形式（类型可以推断）：
    let double = |n| n * 2;
    println!("double(5) = {}", double(5));

    // TODO: 写一个闭包，接受两个 i32，返回它们的乘积
    // let multiply = |???, ???| ??? * ???;
    // println!("6 * 7 = {}", multiply(6, 7));

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：将闭包传给函数 ====================
    // 函数可以接受闭包作为参数

    fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,  // Fn 表示闭包只读取（不修改）环境
    {
        f(value)
    }

    let triple = |n| n * 3;
    println!("triple(7) = {}", apply(triple, 7));

    // TODO: 写一个闭包 square（平方），传给 apply 函数
    // println!("square(8) = {}", apply(???, 8));

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：迭代器基础 ====================
    let numbers = vec![1, 2, 3, 4, 5];

    // 最基础的迭代
    for n in &numbers {
        print!("{n} ");
    }
    println!();

    // 迭代器是惰性的：不调用消费方法就不会执行
    let iter = numbers.iter().map(|n| n * 10);  // 还没执行！
    let result: Vec<i32> = iter.collect();       // collect() 触发执行
    println!("每个数 ×10: {result:?}");

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：迭代器方法链 ====================
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // TODO: 用迭代器方法链实现：
    // 1. 过滤出偶数
    // 2. 每个乘以 3
    // 3. 只取前 3 个
    // 4. 收集到 Vec

    // let result: Vec<i32> = nums.iter()
    //     .filter(|&&n| n % 2 == 0)
    //     .map(|&n| n * 3)
    //     .take(3)
    //     .collect();
    // println!("结果: {result:?}");  // 应该输出 [6, 12, 18]

    println!("--- 任务 4 结束 ---");

    // ==================== 任务 5（挑战）：常用迭代器方法 ====================
    let data = vec![1, 2, 3, 4, 5];

    // TODO: 用迭代器方法计算以下结果
    // sum(): 求和
    // fold(): 累加（初始值 0，累加操作）
    // any(): 是否存在满足条件的元素
    // all(): 是否所有元素都满足条件
    // find(): 查找第一个满足条件的元素

    // 请自行探索并取消注释运行：
    // let sum: i32 = data.iter().sum();
    // let product: i32 = data.iter().fold(1, |acc, x| acc * x);
    // let has_even = data.iter().any(|&x| x % 2 == 0);
    // let all_positive = data.iter().all(|&x| x > 0);
    // let first_even = data.iter().find(|&&x| x % 2 == 0);

    println!("--- 任务 5 结束 ---");
}

// ==================== 思考题 ====================
// 1. 闭包和函数有什么区别？什么时候用闭包？
// 2. Fn、FnMut、FnOnce 有什么区别？
// 3. 迭代器为什么是"零成本抽象"？
// 4. iter() 和 into_iter() 有什么区别？

// 运行方法：cargo run --bin ex18_closures
