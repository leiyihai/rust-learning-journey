// ex19_testing.rs — 第19课：测试
//
// 目标：
// 1. 编写单元测试
// 2. 使用断言宏
// 3. 理解 #[cfg(test)] 和 #[test]

// ==================== 被测试的代码 ====================

/// 将两个数字相加
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 检查一个数是否为偶数
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// 除法：如果除数为 0，返回 None
pub fn safe_divide(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0.0 {
        None
    } else {
        Some(dividend / divisor)
    }
}

/// 字符串反转（支持 Unicode）
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
    // 导入父模块中的所有函数
    use super::*;

    // ==================== 任务 1：基本断言 ====================
    // TODO: 完成测试函数

    #[test]
    fn test_add() {
        // 用 assert_eq!（断言相等）来测试 add 函数
        // assert_eq!(add(2, 3), ???);
        // assert_eq!(add(-1, 5), ???);
        // assert_eq!(add(0, 0), ???);
    }

    // ==================== 任务 2：布尔断言 ====================
    // TODO: 用 assert! 测试 is_even

    #[test]
    fn test_is_even() {
        // assert!(is_even(2));   // 2 是偶数，assert! 期望 true
        // assert!(!is_even(3));  // 3 不是偶数，! 取反
        // assert!(is_even(0));
        // assert!(!is_even(101));
    }

    // ==================== 任务 3：Option 断言 ====================
    // TODO: 测试 safe_divide

    #[test]
    fn test_safe_divide() {
        // 正常除法
        // assert_eq!(safe_divide(10.0, 2.0), Some(5.0));

        // 除以零
        // assert_eq!(safe_divide(10.0, 0.0), None);

        // 浮点数比较（注意精度问题，这里巧合相等）
        // assert_eq!(safe_divide(1.0, 3.0), Some(1.0 / 3.0));
    }

    // ==================== 任务 4：字符串测试 ====================
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string(""), "");

        // TODO: 添加一个中文反转的测试用例
        // assert_eq!(reverse_string("你好世界"), ???);
    }

    // ==================== 任务 5：自定义失败信息 ====================
    #[test]
    fn test_with_message() {
        let result = add(2, 3);
        assert!(
            result == 5,
            "期望 2 + 3 = 5，但得到的是 {result}"  // 自定义错误消息
        );
    }

    // ==================== 任务 6（挑战）：#[should_panic] ====================
    // 有些测试期望代码 panic
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_panic() {
        let v = vec![1, 2, 3];
        let _ = v[99];  // 越界访问，会 panic
    }

    // TODO: 写一个测试，期望 safe_divide 在某个条件下 panic
    // 提示：可以用 unwrap() 让 None panic
}

// ==================== 思考题 ====================
// 1. 单元测试和集成测试有什么区别？在 Rust 中各自放哪里？
// 2. assert_eq! 和 assert! 有什么区别？
// 3. #[should_panic] 的 expected 参数有什么用？
// 4. 如何只运行某一个测试？（提示：cargo test test_name）

// 运行测试：cargo test --bin ex19_testing
