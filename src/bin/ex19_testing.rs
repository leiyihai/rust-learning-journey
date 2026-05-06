// ex19_testing.rs — 第19课：测试
//
// 目标：
// 1. 编写单元测试
// 2. 使用断言宏
// 3. 理解 #[cfg(test)] 和 #[test]
//
// 下面的函数是写好待测的代码，你需要在测试模块中为它们编写测试。

// ==================== 被测试的代码 ====================

pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

pub fn safe_divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn main() {
    println!("这个练习的重点是测试，请用以下命令运行测试：");
    println!("  cargo test --bin ex19_testing");
    println!();
    println!("在下面的 #[cfg(test)] 模块中完成测试代码！");
}

// ==================== 测试模块 ====================
// #[cfg(test)] 表示下面的代码只在 cargo test 时编译
// #[test] 标注一个函数为测试函数

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== 任务 1：基本断言 ====================
    // 用 assert_eq! 测试 add 函数

    #[test]
    fn test_add() {
        // TODO
    }

    // ==================== 任务 2：布尔断言 ====================
    // 用 assert! 测试 is_even（测试偶数、奇数、0）

    #[test]
    fn test_is_even() {
        // TODO
    }

    // ==================== 任务 3：Option 断言 ====================
    // 用 assert_eq! 测试 safe_divide（正常除法、除以零）

    #[test]
    fn test_safe_divide() {
        // TODO
    }

    // ==================== 任务 4：字符串测试 ====================
    // 测试 reverse_string（英文、空串、中文）
    // 提示：英文的 "hello" 反转是 "olleh"

    #[test]
    fn test_reverse_string() {
        // TODO
    }

    // ==================== 任务 5：自定义失败信息 ====================
    // 用 assert! 带自定义错误消息

    #[test]
    fn test_with_message() {
        // TODO
    }

    // ==================== 任务 6（挑战）：#[should_panic] ====================
    // 写一个期望 panic 的测试
    // 提示：访问 vec 越界索引，或用 unwrap() 让 None panic

    #[test]
    #[should_panic]
    fn test_panic() {
        // TODO
    }
}

// ==================== 思考题 ====================
// 1. 单元测试和集成测试有什么区别？在 Rust 中各自放哪里？
// 2. assert_eq! 和 assert! 有什么区别？
// 3. #[should_panic] 的 expected 参数有什么用？
// 4. 如何只运行某一个测试？

// 运行测试：cargo test --bin ex19_testing
