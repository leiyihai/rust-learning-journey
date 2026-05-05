// ex09_extra.rs — 第9课附加练习：枚举与模式匹配
//
// 5 道练习题，由浅入深。每道题都有提示，不提供完整代码。
// 完成后运行：cargo run --bin ex09_extra

// fn main() {
//     println!("完成所有函数后，取消 main 中对应测试的注释来验证。");
// }

// ================================================================
// 练习 1：硬币分类器
// ================================================================
// 定义一个 Coin 枚举：Penny、Nickel、Dime、Quarter
// 写一个 value_in_cents 函数，返回每种硬币的面值（美分）：
//   Penny → 1, Nickel → 5, Dime → 10, Quarter → 25
//
// 提示：
// - 函数签名：fn value_in_cents(coin: Coin) -> u8
// - 用 match 匹配四种硬币
// - 无需携带数据，普通枚举即可

enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 测试（完成后取消注释）：
fn test_coin() {
    assert_eq!(value_in_cents(Coin::Penny), 1);
    assert_eq!(value_in_cents(Coin::Quarter), 25);
    println!("✓ 练习 1 通过");
}

// ================================================================
// 练习 2：Option 安全计算
// ================================================================
// 写一个 safe_sqrt 函数，接受 f64，返回 Option<f64>
// - 如果输入 < 0.0，返回 None（不能对负数开平方根，暂时忽略虚数）
// - 否则返回 Some(x.sqrt())
//
// 然后用 match 写一个 print_sqrt 函数，调用 safe_sqrt 并打印结果：
// - Some(v) → "平方根是 {v}"
// - None → "不能对负数开平方！"
//
// 提示：
// - f64 有 .sqrt() 方法
// - 函数签名：fn safe_sqrt(x: f64) -> Option<f64>
// - fn print_sqrt(x: f64)

fn safe_sqrt(x: f64) -> Option<f64> {
    if x < 0.0 { None} else { Some(x.sqrt()) }
}

fn print_sqrt(x: f64) {
    match safe_sqrt(x) {
        None => { println!("不能对负数开平方！") },
        Some(x) => println!("平方根是{}", x),
    }
}

// 测试（完成后取消注释）：
fn test_sqrt() {
    print_sqrt(9.0);   // 应输出"平方根是 3"
    print_sqrt(-1.0);  // 应输出"不能对负数开平方！"
}

// ================================================================
// 练习 3：形状面积计算
// ================================================================
// 定义一个 Shape 枚举，包含三种形状：
//   Circle(f64)              — 携带半径
//   Rectangle(f64, f64)      — 携带宽、高
//   Triangle(f64, f64)       — 携带底、高
//
// 写一个 area 函数，返回面积：
//   Circle(r)      → π * r²
//   Rectangle(w,h) → w * h
//   Triangle(b,h)  → b * h / 2.0
//
// 提示：
// - 用 std::f64::consts::PI
// - 函数签名：fn area(shape: Shape) -> f64
// - 每种形状携带的数据不同，match 时解构

enum Shape{
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

fn area(shape: Shape) -> f64{
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle(b, h) => b * h /2.0,
    }
}

// 测试（完成后取消注释）：
fn test_shape() {
    let circle = Shape::Circle(3.0);
    let rect = Shape::Rectangle(4.0, 5.0);
    let tri = Shape::Triangle(6.0, 3.0);
    println!("圆形面积: {:.2}", area(circle));      // 28.27
    println!("矩形面积: {:.2}", area(rect));        // 20.00
    println!("三角形面积: {:.2}", area(tri));        // 9.00
}

// ================================================================
// 练习 4：天气信息解读
// ================================================================
// 定义一个 Weather 枚举：
//   Sunny                    — 晴天（无数据）
//   Cloudy(u8)              — 多云，携带云量百分比（0~100）
//   Rainy(f64)              — 下雨，携带降水量（毫米）
//   Windy { speed: f64, direction: String } — 刮风，携带风速和方向
//
// 写一个 describe 函数，返回 &str 描述：
//   Sunny             → "晴天，适合出门"
//   Cloudy(p)         → 如果 p > 80 返回"阴天，可能要下雨"，否则返回"多云"
//   Rainy(mm)         → 如果 mm > 10.0 返回"大雨，别出门"，否则返回"小雨"
//   Windy { speed, .. } → 如果 speed > 50.0 返回"大风预警"，否则返回"微风"
//
// 提示：
// - 函数签名：fn describe(weather: Weather) -> &'static str
// - 在 match 分支中可以用 if 守卫：Cloudy(p) if p > 80 => ...
// - Windy { speed, direction } 解构后 direction 如果不用，可以用 ..

enum Weather{
    Sunny,
    Cloudy(u8),
    Rainy(f64),
    Windy{ speed: f64, direction: String },
}

fn describe(weather: Weather) -> &'static str {
    match weather {
        Weather::Sunny => { "晴天，适合出门" },
        Weather::Cloudy(p) => { if p>80 { "阴天，可能要下雨" } else { "多云" } },
        Weather::Rainy(mm) => { if mm > 10.0 { "大雨，别出门" } else { "小雨" }},
        Weather::Windy{speed, direction} => {
            if speed >50.0 { "大风预警" } else { "微风" }
        }
    }
}

// 测试（完成后取消注释）：
fn test_weather() {
    assert_eq!(describe(Weather::Sunny), "晴天，适合出门");
    assert_eq!(describe(Weather::Cloudy(85)), "阴天，可能要下雨");
    assert_eq!(describe(Weather::Cloudy(30)), "多云");
    assert_eq!(describe(Weather::Rainy(15.0)), "大雨，别出门");
    assert_eq!(describe(Weather::Rainy(3.0)), "小雨");
    let wind = Weather::Windy { speed: 60.0, direction: String::from("北") };
    assert_eq!(describe(wind), "大风预警");
    println!("✓ 练习 4 通过");
}

// ================================================================
// 练习 5：if let 实战
// ================================================================
// 用 Option<i32> 数组表示一串可能缺失的数值：
//   let values = [Some(3), None, Some(7), None, Some(11)];

//
// 写两段代码：
// 1. 用 match 遍历数组，打印所有非 None 的值
// 2. 用 if let 遍历数组，实现同样的效果
//
// 提示：
// - for value in &values { ... }  注意 &values 的类型是 &[Option<i32>; 5]，每个元素是 &Option<i32>
// - match 写法：match value { Some(v) => ..., None => () }
// - if let 写法：if let Some(v) = value { ... }
// - () 是空元组，表示"什么都不做"

// TODO: 写两个函数
fn print_with_match(values: &[Option<i32>]){
    for value in values {
        match value {
            Some(v) => println!("{}", v),
            None => {},
        }
    }
}
fn print_with_iflet(values: &[Option<i32>]){
    for value in values {
        if let Some(v) = value { println!("{}", v); }
    }
}

// 测试（完成后取消注释）：
fn test_print() {
    let values = [Some(3), None, Some(7), None, Some(11)];
    println!("match 方式:");
    print_with_match(&values);
    println!("if let 方式:");
    print_with_iflet(&values);
}

// ================================================================
// 全部完成后取消 main 中的注释来测试
// ================================================================
fn main() {
    test_coin();
    test_sqrt();
    test_shape();
    test_weather();
    test_print();
}
