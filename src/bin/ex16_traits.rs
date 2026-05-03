// ex16_traits.rs — 第16课：特性（Traits）
//
// 目标：
// 1. 定义和实现 trait
// 2. trait bound：约束泛型的行为
// 3. derive 宏和常见标准库 trait

use std::fmt;

// ==================== 任务 1：定义和实现 Trait ====================
// TODO: Speak trait 定义好了，请为 Dog 和 Cat 实现它

trait Speak {
    fn speak(&self) -> &str;
}

// TODO: 为 Dog 和 Cat 实现 Speak trait
struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> &str {
        "汪汪！"
    }
}

// TODO: 为 Cat 实现 Speak（提示：返回 "喵喵！"）
impl Speak for Cat {
    fn speak(&self) -> &str {
        todo!("返回 \"喵喵！\"")
    }
}

fn main() {
    let dog = Dog { name: String::from("旺财") };
    let cat = Cat { name: String::from("咪咪") };

    println!("{} 说: {}", dog.name, dog.speak());
    // println!("{} 说: {}", cat.name, cat.speak());  // Cat 完成后取消注释

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：trait 作为参数 ====================
    // 写一个函数 make_it_speak，接受任何实现了 Speak 的类型
    // TODO: 用 impl Speak 语法
    // fn make_it_speak(animal: &impl Speak) {
    //     println!("动物说: {}", animal.speak());
    // }
    // make_it_speak(&dog);

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：derive 宏 ====================
    #[derive(Debug, Clone, PartialEq)]
    struct Book {
        title: String,
        pages: u32,
    }

    let book1 = Book { title: String::from("Rust 入门"), pages: 300 };
    let book2 = book1.clone();

    println!("{:?}", book1);         // Debug 打印
    println!("两本书相同？{}", book1 == book2);  // PartialEq 比较

    // TODO: 给你的 Dog 和 Cat 加上 #[derive(Debug)]

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：Display trait ====================
    // TODO: 为 Book 手动实现 Display trait（类似 Java 的 toString）
    // impl fmt::Display for Book {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    //         write!(f, "《{}》({} 页)", self.title, self.pages)
    //     }
    // }
    // println!("{book1}");

    println!("--- 任务 4 结束 ---");
}

// ==================== 任务 5（挑战）：多个 trait bound ====================
// 写一个泛型函数 print_and_compare，接受两个参数：
// - 参数必须同时实现 Debug 和 PartialEq
// - 打印两个值，返回它们是否相等
// fn print_and_compare<T: fmt::Debug + PartialEq>(a: &T, b: &T) -> bool {
//     println!("比较 {:?} 和 {:?}", a, b);
//     a == b
// }

// ==================== 思考题 ====================
// 1. Trait 和 Java 的 interface 有什么异同？
// 2. 什么是"孤儿规则"（orphan rule）？它为什么存在？
// 3. Debug 和 Display 的适用场景有什么不同？

// 运行方法：cargo run --bin ex16_traits
