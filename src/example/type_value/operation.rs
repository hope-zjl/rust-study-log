/// ```  
/// # 启动命令
///  cargo run --example operation
///
/// ```
fn main() {
    println!("结果: {}", interproduct(10, 50, 60));
    // 拓展
    println!("a > s: {}", bool_fun(20, 5));
    // 浮点计算
    println!("f: {}", f_fun(2.22, 96.74));
}
// 运算函数
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}

fn bool_fun(a: i32, s: i32) -> bool {
    a > s
}

fn f_fun(a: f32, b: f32) -> f32 {
    a * b
}
