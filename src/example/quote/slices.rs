/// ```
/// # 启动命令
///  cargo run --example slices
///
/// ```

fn main() {
    let a = [40, 60, 40, 82, 90, 77];
    println!("a: {a:?}");

    let s = &a[2..4];
    println!("s: {s:?}");

    let str = String::from("Hello, world!");
    println!("str: {str}");

    let str2 = &str[..5];
    println!("str2: {str2}");

    let str3 = &str[6..];
    println!("str3: {str3}");
}