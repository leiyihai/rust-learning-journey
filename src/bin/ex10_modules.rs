// ex10_modules.rs — 第10课：模块系统
//
// 目标：
// 1. 理解 mod、use、pub
// 2. 了解 crate 和模块树
// 3. 掌握可见性控制

// ==================== 任务 1：定义模块 ====================

mod math_utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }


    #[allow(dead_code)]
    fn helper() {
        println!("调用到helper函数");
    }


    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    // 任务 4：嵌套模块
    pub mod geometry {
        /// 计算圆的面积
        pub fn circle_area(radius: f64) -> f64 {
            std::f64::consts::PI * radius * radius
        }
    }

    // 任务 5：重导出
    pub use geometry::circle_area;  //提示未使用的import  文本灰色
}

fn main() {
    // ==================== 任务 2：使用模块 ====================
    println!("3 + 5 = {}", math_utils::add(3, 5));

    // 试试调用 math_utils::helper()，观察编译错误
    // 不用编译已经提示红额波浪线helper是私有的了

    println!("--- 任务 2 ---");

    // ==================== 任务 3：use 关键字 ====================
    // TODO: 用 use 导入 math_utils::multiply，这样就可以直接用 multiply() 了

    use math_utils::multiply;
    println!("3 * 5 = {}", multiply(3, 5));

    println!("--- 任务 3 ---");

    // ==================== 任务 4：嵌套模块 ====================
    println!("半径为 3 的圆面积: {}", math_utils::geometry::circle_area(3.0));

    println!("--- 任务 4 ---");

    // ==================== 任务 5：pub use（重导出）====================
    // 在 math_utils 模块中，用 pub use 将 geometry::circle_area
    // 重导出，让它可以直接通过 math_utils::circle_area 访问
    math_utils::circle_area(2.0);
}

// ==================== 任务 6（挑战）：组织真实项目 ====================
// 把这个练习的模块拆分成两个文件：
// 1. 把 math_utils 模块的内容移到 src/math_utils_new
// 2. 在 src/bin/ex10_modules.rs 中用 mod math_utils; 引入
// 为了避免去掉上面做了的题目，这题我额外写在ex10_modules_task6.rs里了
// 又因为项目没有main.rs和lib.rs  没办法通过crate::绝对路径来找所以移动到了bin里，也可能是我搞错了？
// 还尝试过用super  也没成功，反正有报错。

// ==================== 思考题 ====================
// 1. mod, use, pub 分别起什么作用？
// mod可以把普通的代码文件变成模块（在lib和bin中挂载）;use可以缩短模块路径的调用，不用写太长的调用链;
// pub决定文件夹模块里的模块和方法是否需要公开，需要公开就加pub
// 2. crate 是什么？crate root 又是什么？
//最小的可编译的单元文件；crate root指的是bin(main.rs和bin文件夹里的rs文件)和lib(lib.rs)
// 3. Rust 的模块系统和 Python/JavaScript 的 import 有什么不同？
//没感受出来区别

// 运行方法：cargo run --bin ex10_modules

