// ex12_vectors.rs — 第12课：向量（Vec<T>）
//
// 目标：
// 1. 创建和初始化 Vec
// 2. 掌握增删改查操作
// 3. 遍历 Vec 的各种方式

fn main() {
    // ==================== 任务 1：创建 Vec ====================
    // 三种常见创建方式：
    let v1: Vec<i32> = Vec::new();      // 空的 Vec
    let v2 = vec![1, 2, 3, 4, 5];       // vec! 宏
    let v3 = vec![0; 5];                 // 5 个 0：[0, 0, 0, 0, 0]

    println!("v1: {v1:?}");
    println!("v2: {v2:?}");
    println!("v3: {v3:?}");

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：增删改查 ====================
    // TODO: 完成以下操作
    let mut numbers: Vec<i32> = vec![];

    // 添加元素
    // numbers.push(10);
    // numbers.push(20);
    // numbers.push(30);
    println!("添加后: {numbers:?}");

    // 访问元素（注意边界检查）
    // println!("第一个: {}", numbers[0]);         // 越界会 panic
    // println!("第一个（安全）: {:?}", numbers.get(0));  // 返回 Option
    // println!("第100个（安全）: {:?}", numbers.get(100)); // 不会 panic！

    // 修改元素
    // if let Some(first) = numbers.get_mut(0) {
    //     *first = 100;
    // }
    // println!("修改后: {numbers:?}");

    // 删除元素
    // let last = numbers.pop();  // 移除最后一个元素并返回
    // println!("弹出: {last:?}, 剩余: {numbers:?}");

    // numbers.remove(0);  // 移除索引 0 的元素
    // println!("移除后: {numbers:?}");

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：遍历 Vec ====================
    let nums = vec![10, 20, 30, 40, 50];

    // TODO: 用三种方式遍历
    // 方式 1：不可变遍历
    // for n in &nums { ... }

    // 方式 2：可变遍历
    // let mut nums = nums.clone();
    // for n in &mut nums { *n *= 2; }

    // 方式 3：带索引遍历
    // for (i, n) in nums.iter().enumerate() { ... }

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4（挑战）：统计 ====================
    // 给定一个 Vec<i32>，计算：
    // 1. 平均值（用 f64 表示）
    // 2. 最大值和最小值
    // 3. 所有偶数的和

    let data = vec![15, 8, 23, 42, 16, 4, 7, 30];

    // TODO: 实现下面的三个计算

    // 平均值
    // let sum: i32 = data.iter().sum();
    // let avg = sum as f64 / data.len() as f64;

    // 最大值最小值（用 iter().max() 和 iter().min()）

    // 偶数和（用 filter）
    // let even_sum: i32 = data.iter().filter(|&&x| x % 2 == 0).sum();

    println!("--- 任务 4 结束 ---");
}

// ==================== 思考题 ====================
// 1. vec[3] 和 vec.get(3) 有什么区别？各适用于什么场景？
// 2. Vec<T> 和数组 [T; N] 有什么区别？什么时候用哪个？
// 3. push 和 pop 的时间复杂度是多少？

// 运行方法：cargo run --bin ex12_vectors
