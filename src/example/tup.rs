/// ```
/// # 启动命令
/// cargo run --example tup
/// ```
fn main() {
    let x: (i32, bool, char) = (7, true, '😏');
    println!("x.0: {:?}", x.0);
    println!("x.1: {:?}", x.1);
    println!("x.2: {:?}", x.2);
}