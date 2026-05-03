// ex10_modules.rs — 第10课：模块系统
//
// 目标：
// 1. 理解 mod、use、pub
// 2. 了解 crate 和模块树
// 3. 掌握可见性控制

// ==================== 任务 1：定义模块 ====================
// TODO: 在下面的模块中添加公开函数
mod math_utils {
    // TODO: 添加一个公开函数 add(a: i32, b: i32) -> i32
    pub fn add(a: i32, b: i32) -> i32 {
        todo!("返回 a + b")
    }

    // TODO: 添加一个私有函数 helper()，尝试从外部访问它
    // 看看编译器会说什么
    #[allow(dead_code)]
    fn helper() {
        println!("我是私有函数");
    }

    // TODO: 添加一个公开函数 multiply(a: i32, b: i32) -> i32
    pub fn multiply(a: i32, b: i32) -> i32 {
        todo!("返回 a * b")
    }

    // 任务 4：嵌套模块
    pub mod geometry {
        /// 计算圆的面积
        pub fn circle_area(radius: f64) -> f64 {
            todo!("返回 std::f64::consts::PI * radius * radius")
        }
    }

    // 任务 5：重导出
    // TODO: 取消下面的注释
    // pub use geometry::circle_area;
}

fn main() {
    // ==================== 任务 2：使用模块 ====================
    // TODO: 取消下面的注释
    // println!("3 + 5 = {}", math_utils::add(3, 5));

    // 试试调用 math_utils::helper()，观察编译错误

    println!("--- 任务 2 ---");

    // ==================== 任务 3：use 关键字 ====================
    // TODO: 用 use 导入 math_utils::multiply，这样就可以直接用 multiply() 了

    // use ???;
    // println!("3 * 5 = {}", multiply(3, 5));

    println!("--- 任务 3 ---");

    // ==================== 任务 4：嵌套模块 ====================
    // TODO: 取消下面的注释
    println!("半径为 3 的圆面积: {}", math_utils::geometry::circle_area(3.0));

    println!("--- 任务 4 ---");

    // ==================== 任务 5：pub use（重导出）====================
    // 在 math_utils 模块中，用 pub use 将 geometry::circle_area
    // 重导出，让它可以直接通过 math_utils::circle_area 访问
    // TODO: 在 math_utils 里取消 pub use geometry::circle_area 的注释
}

// ==================== 任务 6（挑战）：组织真实项目 ====================
// 把这个练习的模块拆分成两个文件：
// 1. 把 math_utils 模块的内容移到 src/math_utils.rs
// 2. 在 src/bin/ex10_modules.rs 中用 mod math_utils; 引入

// ==================== 思考题 ====================
// 1. mod, use, pub 分别起什么作用？
// 2. crate 是什么？crate root 又是什么？
// 3. Rust 的模块系统和 Python/JavaScript 的 import 有什么不同？

// 运行方法：cargo run --bin ex10_modules
