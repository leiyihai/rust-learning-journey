#[path = "../math_utils_new.rs"]
mod math_utils_new;
fn main() {
    println!("1 + 2 = {}", math_utils_new::add(1, 2));
    math_utils_new::multiply(3, 4);
    math_utils_new::circle_area(5.0);
}