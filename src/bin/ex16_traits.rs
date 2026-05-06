// ex16_traits.rs — 第16课：特性（Traits）
//
// 目标：
// 1. 定义和实现 trait
// 2. trait bound：约束泛型的行为
// 3. derive 宏和常见标准库 trait

// ==================== 任务 1：定义和实现 Trait ====================
// 为 Dog 和 Cat 实现 Speak trait

trait Speak {
    fn speak(&self) -> &str;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// TODO: 为 Dog 实现 Speak（提示：返回 "汪汪！"）
impl Speak for Dog {
    fn speak(&self) -> &str {
        todo!("返回 \"汪汪！\"")
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

    // TODO: 完成 Speak 实现后，调用 dog.speak() 和 cat.speak() 并打印

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：trait 作为参数 ====================
    // 写一个 make_it_speak 函数，接受 &impl Speak，打印 animal.speak()
    // 提示：fn make_it_speak(animal: &impl Speak) { ... }

    // TODO

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：derive 宏 ====================
    // 观察 #[derive] 自动生成的 Debug、Clone、PartialEq
    #[derive(Debug, Clone, PartialEq)]
    struct Book {
        title: String,
        pages: u32,
    }

    let book1 = Book { title: String::from("Rust 入门"), pages: 300 };
    let book2 = book1.clone();

    println!("{:?}", book1);
    println!("两本书相同？{}", book1 == book2);

    // TODO: 给 Dog 和 Cat 加上 #[derive(Debug)]

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：Display trait ====================
    // 为 Book 手动实现 Display trait，输出格式类似《书名》(xxx 页)
    // 提示：impl std::fmt::Display for Book { fn fmt(...) -> fmt::Result { write!(...) } }
    // 实现后用 println!("{book1}") 验证

    // TODO

    println!("--- 任务 4 结束 ---");
}

// ==================== 任务 5（挑战）：多个 trait bound ====================
// 写一个 print_and_compare 函数：
// - 接受两个实现了 Debug + PartialEq 的参数
// - 打印两个值，返回它们是否相等

// TODO

// ==================== 思考题 ====================
// 1. Trait 和 Java 的 interface 有什么异同？
// 2. 什么是"孤儿规则"（orphan rule）？它为什么存在？
// 3. Debug 和 Display 的适用场景有什么不同？

// 运行方法：cargo run --bin ex16_traits
