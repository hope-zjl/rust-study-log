/// ```  
/// # 启动命令
///  cargo run --example fibonacci
///
/// ```
fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

fn fib(n: u32) -> u32 {
    if n <= 2 {
        // The base case.
        1
    } else {
        // The recursive case.
        fib(n - 1) + fib(n - 2)
    }
}
