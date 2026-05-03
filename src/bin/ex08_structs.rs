// ex08_structs.rs — 第8课：结构体（Structs）
//
// 目标：
// 1. 定义和实例化结构体
// 2. 结构体方法（impl 块）
// 3. 关联函数（"静态方法"）

// ==================== 任务 1：定义一个结构体 ====================
// TODO: 补全 Student 结构体的字段
// - name: String
// - age: u8
// - score: f64
struct Student {
    name: String,
    age: u8,
    score: f64,
}

// ==================== 任务 2：结构体方法 ====================
// TODO: 为 Student 实现方法
impl Student {
    // 关联函数（构造函数）：接受 name 和 age，创建 Student（score 默认 0.0）
    fn new(name: String, age: u8) -> Student {
        todo!("返回一个新的 Student 实例，score 设为 0.0")
    }

    // 方法：显示学生信息
    fn display(&self) {
        todo!("打印学生的姓名、年龄和分数，用 println!")
    }

    // 方法：更新分数
    fn update_score(&mut self, new_score: f64) {
        todo!("将 new_score 赋值给 self.score")
    }

    // 方法：判断是否及格（分数 >= 60.0）
    fn is_passing(&self) -> bool {
        todo!("返回 self.score >= 60.0")
    }
}

fn main() {
    // TODO: 取消下面的注释，然后完成上面的方法
    // let mut alice = Student::new(String::from("Alice"), 20);
    // let bob = Student::new(String::from("Bob"), 22);

    // alice.display();
    // alice.update_score(85.5);
    // alice.display();
    // println!("Alice 及格了吗？{}", alice.is_passing());

    // bob.display();
    // println!("Bob 及格了吗？{}", bob.is_passing());

    println!("完成 TODO 后取消 main 中的注释来测试！");

    // ==================== 任务 3（挑战）：元组结构体 ====================
    // 定义两个元组结构体：
    // struct Color(u8, u8, u8);   // RGB
    // struct Point(i32, i32);     // 坐标 (x, y)

    // 创建 Color 和 Point 的实例，并打印它们的字段

    // ==================== 思考题 ====================
    // 1. 方法第一个参数 &self 和 &mut self 的区别是什么？
    // 2. 关联函数（如 new）和普通方法有什么不同？怎么调用它们？
    // 3. Rust 没有 class 关键字，struct + impl 能达到 OOP 的效果吗？
}

// 运行方法：cargo run --bin ex08_structs
