// ex09_enums.rs — 第9课：枚举与模式匹配
//
// 目标：
// 1. 定义枚举类型
// 2. 掌握 match 表达式
// 3. 了解 if let 语法糖
// 4. 认识 Option<T>（消除空指针的关键）

// ==================== 任务 1：定义枚举 ====================
// TODO: 定义一个 TrafficLight 枚举，包含 Red、Yellow、Green 三个成员
enum TrafficLight {
    // 填上三个成员
    Red,
    Yellow,
    Green,
}

// TODO: 定义一个 Message 枚举，它的成员可以持有数据
// Quit                    — 不持有数据
// Write(String)           — 持有一个 String
// ChangeColor(u8, u8, u8) — 持有三个 u8（RGB）
enum Message {
    // 填上成员
    Quit,
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn main() {
    // ==================== 任务 2：match 表达式 ====================
    // TODO: 完成 traffic_light_action 函数，然后用不同信号灯测试
    let light = TrafficLight::Red;
    println!("{}", traffic_light_action(light));

    let light = TrafficLight::Green;
    println!("{}", traffic_light_action(light));

    let light = TrafficLight::Yellow;
    println!("{}", traffic_light_action(light));

    println!("--- 任务 2 ---");

    // ==================== 任务 3：带数据的枚举 + match ====================
    // TODO: 完成 process_message 函数，然后测试
    let msg1 = Message::Write(String::from("你好，Rust！"));
    process_message(msg1);

    let msg2 = Message::ChangeColor(255, 0, 0);
    process_message(msg2);

    let msg3 = Message::Quit;
    process_message(msg3);

    println!("--- 任务 3 ---");

    // ==================== 任务 4：Option<T> ====================
    // Option<T> 是 Rust 标准库的枚举，用来替代 null
    // enum Option<T> { Some(T), None }

    // TODO: 写一个函数 maybe_divide，返回 Option<f64>
    // - 如果除数为 0，返回 None
    // - 否则返回 Some(结果)
    fn maybe_divide(a:f64, b:f64) -> Option<f64> {
        if b == 0.0 { None } else { Some(a/b) }
    }

    // 测试你的函数
    match maybe_divide(10.0, 2.0) {
        Some(v) => println!("10 / 2 = {v}"),
        None => println!("不能除以零！"),
    }

    match maybe_divide(10.0, 0.0) {
        Some(v) => println!("10 / 0 = {v}"),
        None => println!("不能除以零！"),
    }

    println!("--- 任务 4 ---");

    // ==================== 任务 5：if let 语法糖 ====================
    // 当我们只关心一种模式时，if let 比 match 更简洁
    let maybe_name = Some(String::from("Alice"));

    // TODO: 用 if let 判断 maybe_name 是不是 Some
    if let Some(name) = maybe_name {
        println!("名字是 {name}");
    } else {
        println!("没有名字");
    }

    println!("--- 任务 5 ---");
}

// TODO: 实现 traffic_light_action，返回 &str
// Red → "停下来", Yellow → "准备", Green → "通行"
fn traffic_light_action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "停下来",
        TrafficLight::Yellow => "准备",
        TrafficLight::Green => "通行"
    }
}

// TODO: 实现 process_message，用 match 处理不同类型的 Message
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Write(s) => println!("Writing {}", s),
        Message::ChangeColor(r, g, b) => println!("R: {},G: {},B: {}", r, g, b),
    }
}

// TODO: 实现 maybe_divide，返回 Option<f64>
// fn maybe_divide(a: f64, b: f64) -> Option<f64> {
//     todo!("除数为零返回 None，否则返回 Some(a / b)")
// }

// ==================== 思考题 ====================
// 1. Rust 的 Option<T> 和 null 有什么本质区别？
// Option<T>标准库里定义的enum 它分为Some(T)和None 就是Option<T>包含了null,match必须穷尽匹配，所以不会出现空引用
// 2. match 是穷尽的（exhaustive），这意味着什么？
// 不会有空引用
// 3. 什么时候用 match，什么时候用 if let？
// 只匹配一种情况的时候用if let  其他时候用match

// 运行方法：cargo run --bin ex09_enums
