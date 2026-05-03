// ex03_types.rs — 第3课：数据类型
//
// 目标：
// 1. 掌握标量类型：整数、浮点数、布尔、字符
// 2. 掌握复合类型：元组、数组
// 3. 理解 Rust 是静态类型语言

fn main() {
    // ==================== 任务 1：标量类型 ====================
    // 补全下面的变量类型标注
    // 提示：Rust 通常可以自动推断类型，但显式标注能帮助理解

    // TODO: 补全类型（i32, f64, bool, char）
    let age: u8 = 25;             // u8 = 0~255 的无符号整数
    let temperature: f64 = 36.5;  // f64 = 64位浮点数
    let is_rust_fun: bool = true; // bool = 布尔值
    let grade: char = 'A';        // char = Unicode 字符（4字节）

    println!("年龄: {age}, 体温: {temperature}, 有趣吗: {is_rust_fun}, 等级: {grade}");

    // ==================== 任务 2：元组 ====================
    // TODO: 创建一个元组 person，包含 (姓名, 年龄, 身高)
    // 姓名用 &str，年龄用 u8，身高用 f64
    // 然后用解构取出每个元素并打印

    // let person: (???, ???, ???) = ("小明", 20, 1.75);
    // let (name, age, height) = person;
    // println!("{} 今年 {} 岁，身高 {} 米", name, age, height);

    // 也可以通过索引访问元组元素
    // println!("姓名: {}", person.0);

    // ==================== 任务 3：数组 ====================
    // TODO: 创建一个包含 5 个 i32 的数组 scores
    // 用 for 循环遍历打印每个分数
    // 提示：数组长度固定，类型为 [i32; 5]

    // let scores: [i32; 5] = [90, 85, 78, 92, 88];
    // 遍历数组的推荐方式：
    // for score in scores {
    //     println!("分数: {score}");
    // }
    // 也可以按索引遍历
    // for i in 0..scores.len() {
    //     println!("第 {} 个分数: {}", i + 1, scores[i]);
    // }

    // ==================== 任务 4（挑战）：整数溢出 ====================
    // 下面的代码在 debug 模式下会 panic，在 release 模式下会回绕
    // let mut small: u8 = 255;
    // small = small + 1;  // debug 模式: panic!  release 模式: 0
    // 试试看！然后试着用 wrapping_add 方法来安全处理溢出
    // println!("溢出后: {}", 255u8.wrapping_add(1));

    // ==================== 思考题 ====================
    // 1. 数组和元组有什么区别？什么时候用哪个？
    // 2. Rust 有多少种整数类型？为什么要分这么多种？
}

// 运行方法：cargo run --bin ex03_types
