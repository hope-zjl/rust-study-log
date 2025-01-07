/// ```  
/// # 启动命令
/// cargo run --example operation
/// ```
fn main() {
    println!("结果: {}", interproduct(10, 50, 60));
}
// 运算函数
fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    a * b + b * c + c * a
}
