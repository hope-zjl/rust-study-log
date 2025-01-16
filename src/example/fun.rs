/// ```
/// # 启动命令
/// cargo run --example fun
/// ```
fn main() {
    println!("gcb: {}", gcb(99, 70));
}

fn gcb(a: u32, b: u32) -> u32 {
    if b > 0 {
        gcb(b, a % b)
    } else {
        a
    }
}