/// ```
/// # 启动命令
///  cargo run --example shared
///
/// ```

fn main() {
    let a = 'A';
    let b = 'B';

    let mut r = &a;
    println!("r: {}", *r);

    r = &b;
    println!("r: {}", *r);
}