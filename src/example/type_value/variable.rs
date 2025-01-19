/// ```  
/// # 启动命令
///  cargo run --example variable
///
/// ```
fn main() {
    let x: i32 = 10;
    println!("x: {x}");
    // 添加mut关键字 把变量设置成可变的
    let mut s: i32 = 90;
    println!("s: {s}");
    s = 70;
    println!("s: {s}");
}
