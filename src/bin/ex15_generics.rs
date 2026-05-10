// ex15_generics.rs — 第15课：泛型（Generics）
//
// 目标：
// 1. 理解泛型的作用：用同一套逻辑处理不同类型
// 2. 泛型函数、泛型结构体
// 3. 单态化（Monomorphization）：零成本抽象

fn main() {
    // ==================== 任务 1：泛型函数 ====================
    // 写一个 largest 泛型函数，返回两个值中较大的那个
    // 用 i32、f64、char 三种类型测试
    // 提示：泛型参数需要 PartialOrd trait（才能用 > 比较）

    println!("较大的值是：{}", largest(1.0, 2.0));
    println!("较大的值是：{}", largest(4, 3));
    println!("较大的值是：{}", largest('a', 'z'));

    println!("--- 任务 1 结束 ---");

    // ==================== 任务 2：泛型结构体 ====================
    // 定义 Point<T> 结构体，包含 x 和 y 两个字段
    // 创建 Point<i32> 和 Point<f64> 的实例并打印

    let p1 = Point{x: 6, y: 8};
    println!("p1.x = {}, p1.y = {}", p1.x, p1.y);
    let p2 = Point{x: 9.5, y: 10.3};
    println!("p2.x = {}, p2.y = {}", p2.x, p2.y);

    println!("--- 任务 2 结束 ---");

    // ==================== 任务 3：泛型 + 方法 ====================
    // 为 Point<T> 实现一个 mixup 方法，将两个不同类型的 Point 混合
    // 提示：需要引入第二个泛型参数

    let p3 = p1.mixup(p2);
    println!("混合后的x:{},y:{}",p3.0, p3.1);

    println!("--- 任务 3 结束 ---");

    // ==================== 任务 4：多个泛型参数 ====================
    // 定义一个 Pair<T, U>，实现一个 swap 方法，返回 Pair<U, T>

    let pair = Pair{first: "a", second: "b"};
    let swap_pair = pair.swap();
    println!("{},{}", swap_pair.first, swap_pair.second);

    println!("--- 任务 4 结束 ---");

    let mut my_stack = Stack::new();
    my_stack.push("hello".to_string());
    my_stack.push("world".to_string());
    my_stack.push("hi".to_string());
    my_stack.push("rust".to_string());
    my_stack.pop();
    println!("{:?}", my_stack.peek());
    println!("{:?}", my_stack.is_empty());

}
fn largest<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn mixup<U>(self, other: Point<U>) -> (T, U){
        (self.x, other.y)
    }
}

struct Pair<T, U>{
    first: T,
    second: U,
}
impl<T, U> Pair<T, U> {
    fn swap(self) -> Pair<U, T> {
        Pair{
            first: self.second,
            second: self.first,
        }
    }
}

// ==================== 任务 5（挑战）：泛型栈 ====================
// 实现一个简易的泛型栈 Stack<T>
// 方法：new(), push(), pop() -> Option<T>, peek() -> Option<&T>, is_empty() -> bool
// 底层用 Vec<T> 存储

// 泛型栈：底层用 Vec<T> 存数据
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>, // 底层存储
}

// 给 Stack<T> 实现方法
impl<T> Stack<T> {
    // 1. 创建空栈
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // 2. 压入元素
    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // 3. 弹出元素（返回 Option<T>）
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // 4. 查看栈顶（返回引用，不拿走）
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // 5. 判断是否为空
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// ==================== 思考题 ====================
// 1. Rust 的泛型和 Java/C++ 的泛型有什么不同？（提示：单态化）
// 就是替我们手动写了所有的接收不同类型参数的同名函数
// 2. 泛型中的 T 必须在编译时确定吗？为什么？
// 确定。因为单态化？
// 3. 单态化的优势和劣势分别是什么？
// 不知道。运行时没有额外开销,但是函数多编译会久一点？

// 运行方法：cargo run --bin ex15_generics
