// ex13_hashmaps.rs — 第13课：哈希映射（HashMap<K,V>）
//
// 目标：
// 1. 创建和初始化 HashMap
// 2. 掌握插入、访问、更新操作
// 3. 了解 entry API

use std::collections::HashMap;

fn main() {
    // ==================== 任务 1：创建 HashMap ====================
    // TODO: 创建一个空的 HashMap（team_scores），键是球队名（String），值是分数（i32）

    // let mut team_scores: HashMap<String, i32> = ???;

    // 插入数据
    // team_scores.insert(String::from("红队"), 10);
    // team_scores.insert(String::from("蓝队"), 8);
    // team_scores.insert(String::from("绿队"), 15);
    // println!("队伍和分数: {team_scores:?}");

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：访问和更新 ====================
    // TODO: 用 .get() 获取某个键的值

    // let team_name = String::from("蓝队");
    // match team_scores.get(???) {
    //     Some(score) => println!("{team_name}: {score} 分"),
    //     None => println!("{team_name} 不存在"),
    // }

    // TODO: 覆盖更新
    // team_scores.insert(String::from("蓝队"), 25);  // 覆盖旧值

    // TODO: 只在键不存在时插入
    // team_scores.entry(String::from("黄队")).or_insert(12);

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：遍历 HashMap ====================
    // TODO: 用 for 循环遍历 HashMap

    // for (team, score) in &team_scores {
    //     println!("{team}: {score} 分");
    // }

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4（挑战）：词频统计 ====================
    // 实现一个词频统计器：给定一段文本，统计每个单词出现的次数
    // 提示：
    // 1. 用 .split_whitespace() 分割单词
    // 2. 用 HashMap<String, u32>
    // 3. 用 .entry(word).or_insert(0) 然后 *count += 1

    let text = "apple banana apple orange banana apple";
    println!("文本: {text}");

    // TODO: 完词频统计逻辑
    let mut word_count: HashMap<String, u32> = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = word_count.entry(word.to_string()).or_insert(0);
    //     *count += 1;
    // }

    // println!("词频统计结果: {word_count:?}");

    // 如果完成，应该输出：{"apple": 3, "banana": 2, "orange": 1}

    println!("--- 任务 4 结束 ---");
}

// ==================== 思考题 ====================
// 1. HashMap 的键可以是任意类型吗？有什么限制？
// 2. entry API 相比先 get 再 insert 有什么优势？
// 3. HashMap 在 Rust 中不是预导入的（需要 use），而 Vec 不需要，为什么？

// 运行方法：cargo run --bin ex13_hashmaps
