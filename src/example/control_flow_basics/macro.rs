/// ```
/// # 启动命令
///  cargo run --example macro
///
/// ```

fn main() {
    let n = 4;
    println!("{n}! = {}", factorial(n));
}

fn factorial(n: u32) -> u32 {
    let mut x = 1;
    for i in 1..=n {
        x *= dbg!(i);
    }
    x
}

fn _fizzbuzz() {
    todo!("未被执行")
}
