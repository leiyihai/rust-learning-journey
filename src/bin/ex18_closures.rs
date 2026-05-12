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

    let add_x = |n: i32| -> i32 { n + x };
    println!("5 + 10 = {}", add_x(5));

    // 写一个闭包 multiply，接受两个 i32，返回它们的乘积
    let multiply = |a:i32, b:i32| return a * b;
    println!("5 * 10 = {}", multiply(5, 10));
    // 写一个闭包 square，接受一个 i32，返回它的平方
    let square = | s:i32| -> i32 { s * s };


    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：将闭包传给函数 ====================
    // apply 函数已经写好，写一个闭包传给 apply


    fn apply<F>(f: F, value: i32) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(value)
    }


    let triple = |n| n * 3;
    println!("triple(7) = {}", apply(triple, 7));

    println!("{}", apply(square, 7));

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：迭代器基础 ====================
    let numbers = vec![1, 2, 3, 4, 5];

    for n in &numbers {
        print!("{n} ");
    }
    println!();

    // 试试 .iter().map().collect() 把每个元素乘以 10 收集到新 Vec

    let new_numbers = numbers.iter().map(|x| x * 10).collect::<Vec<i32>>();
    println!("{:?}", new_numbers);

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：迭代器方法链 ====================
    // 对 nums 实现：过滤出偶数 → 每个乘以 3 → 只取前 3 个 → 收集到 Vec
    // 提示：.filter().map().take().collect()

    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_nums = nums.iter().filter(|x| *x % 2 == 0).map(|x| x * 3).take(3).collect::<Vec<i32>>();
    println!("{:?}", new_nums);

    println!("--- 任务 4 结束 ---");

    // ==================== 任务 5（挑战）：常用迭代器方法 ====================
    // 用迭代器方法计算：求和、累乘、是否存在偶数、是否全部正数、查找第一个偶数
    // 提示：sum() / fold() / any() / all() / find()

    let data = vec![1, 2, 3, 4, 5];

    let sum: i32 = data.iter().sum();
    println!("求和：{}", sum);
    let fold: i32 = data.iter().fold(1, |acc, x| acc * x);
    println!("累乘：{}", fold);
    let any  = data.iter().any(|x| *x % 2 == 0);
    println!("是否存在偶数：{}", any);
    let all = data.iter().all(|x| *x > 0);
    println!("是否全部正数：{}", all);
    let first_even = data.iter().find(|x| **x % 2 == 0);
    println!("第一个偶数：{:?}", first_even);

    println!("--- 任务 5 结束 ---");
}

// ==================== 思考题 ====================
// 1. 闭包和函数有什么区别？什么时候用闭包？
// 闭包可以捕获函数外的变量。使用到迭代器需要
// 2. Fn、FnMut、FnOnce 有什么区别？
// Fn只读捕获的变量，FnMut修复捕获的数量，FnOnce转移所有权
// 3. 迭代器为什么是"零成本抽象"？
// 不知道
// 4. iter() 和 into_iter() 有什么区别？
// iter()拿到的是&T, into_iter()拿到的是T?

// 运行方法：cargo run --bin ex18_closures
