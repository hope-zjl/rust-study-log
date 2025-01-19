/// ```  
/// # 启动命令
///  cargo run --example koraz
///
/// ```

fn main() {
    println!("len: {}", collatz_length(4564685));
}

fn collatz_length(mut n: i32) -> i32 {
    let mut len = 1;
    let mut vec = vec![];
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
        vec.push(n);
    }
    println!("vec: {:?}", vec);
    len
}
