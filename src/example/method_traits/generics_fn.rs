/// ```
/// # 启动命令
///  cargo run --example generics_fn
///
/// ```

fn pick<T>(n: i32, even: T, odd: T) -> T {
    if n % 2 == 0 {
        even
    } else {
        odd
    }
}

fn main() {
    println!("{}", pick(100, 5, 5));
    println!("{:?}", pick(68, ('A', 90), ('C', 20)));
    println!("{:?}", pick(68, 'A', 'C'));
}
