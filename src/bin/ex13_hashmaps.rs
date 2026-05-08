// ex13_hashmaps.rs — 第13课：哈希映射（HashMap<K,V>）
//
// 目标：
// 1. 创建和初始化 HashMap
// 2. 掌握插入、访问、更新操作
// 3. 了解 entry API

use std::collections::HashMap;

fn main() {
    // ==================== 任务 1：创建 HashMap ====================
    // 创建一个空的 HashMap（team_scores），键是球队名（String），值是分数（i32）
    // 插入三支队伍的数据
    // 提示：用 HashMap::new() 创建，用 .insert() 添加

    let mut team_scores = HashMap::new();
    team_scores.insert(String::from("Team A"), 10);
    team_scores.insert(String::from("Team B"), 20);
    team_scores.insert(String::from("Team C"), 30);

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：访问和更新 ====================
    // 用 .get() 获取某个键的值（注意返回类型）
    // 用 .insert() 覆盖更新已有键
    // 用 .entry().or_insert() 只在键不存在时插入

    let team_a = String::from("Team A");
    let team_a_scores = team_scores.get(&team_a);
    println!("Team A scores: {:?}", team_a_scores);
    team_scores.insert(String::from("Team A"), 100);
    println!("Team A scores after update: {:?}", team_scores.get(&team_a));

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：遍历 HashMap ====================
    // 用 for 循环遍历 team_scores，打印每支队伍和分数

    for (team, score) in &team_scores {
        println!("{}的分数是：{}", team, score)
    }

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4（挑战）：词频统计 ====================
    // 统计 "apple banana apple orange banana apple" 中每个单词出现次数
    // 提示：
    // 1. 用 .split_whitespace() 分割单词
    // 2. 用 HashMap<String, u32>
    // 3. 用 .entry(word).or_insert(0) 然后 *count += 1
    // word_counts.entry(word)意思去 HashMa p里找单词键（存在返回值，不存在返回空）
    // or_insert(0) 是entry()的配套方法，如果这个单词键已经在 HashMap 里了 → 就返回它的值
    // 不存在就先插入 0，然后再返回0
    // 结果应输出：{"apple": 3, "banana": 2, "orange": 1}

    let text = "apple banana apple orange banana apple";
    println!("文本: {text}");

    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    for (word, count) in &word_counts {
        println!("{word}: {count}");
    }

    println!("--- 任务 4 结束 ---");
}

// ==================== 思考题 ====================
// 1. HashMap 的键可以是任意类型吗？有什么限制？
// 不是，不能为浮点类型
// 2. entry API 相比先 get 再 insert 有什么优势？
// 可以一步到位，有就返回值，没有就插入并返回, get返回Option，还需要判断
// 3. HashMap 在 Rust 中不是预导入的（需要 use），而 Vec 不需要，为什么？
// 这个我也不知道
// 运行方法：cargo run --bin ex13_hashmaps
