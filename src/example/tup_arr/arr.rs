/// ```
/// # 启动命令
///  cargo run --example arr
///
/// ```

fn main() {
    let mut a: [i32; 10] = [19; 10];
    a[4] = 2;
    println!("a: {:?}", a);
    println!("a[1]: {:?}", a[1]);
}