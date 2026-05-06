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

    // TODO

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：访问和更新 ====================
    // 用 .get() 获取某个键的值（注意返回类型）
    // 用 .insert() 覆盖更新已有键
    // 用 .entry().or_insert() 只在键不存在时插入

    // TODO

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：遍历 HashMap ====================
    // 用 for 循环遍历 team_scores，打印每支队伍和分数

    // TODO

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4（挑战）：词频统计 ====================
    // 统计 "apple banana apple orange banana apple" 中每个单词出现次数
    // 提示：
    // 1. 用 .split_whitespace() 分割单词
    // 2. 用 HashMap<String, u32>
    // 3. 用 .entry(word).or_insert(0) 然后 *count += 1
    // 结果应输出：{"apple": 3, "banana": 2, "orange": 1}

    let text = "apple banana apple orange banana apple";
    println!("文本: {text}");

    // TODO

    println!("--- 任务 4 结束 ---");
}

// ==================== 思考题 ====================
// 1. HashMap 的键可以是任意类型吗？有什么限制？
// 2. entry API 相比先 get 再 insert 有什么优势？
// 3. HashMap 在 Rust 中不是预导入的（需要 use），而 Vec 不需要，为什么？

// 运行方法：cargo run --bin ex13_hashmaps
