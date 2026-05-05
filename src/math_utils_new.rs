
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


pub mod geometry {
    /// 计算圆的面积
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }
}

pub use geometry::circle_area;


