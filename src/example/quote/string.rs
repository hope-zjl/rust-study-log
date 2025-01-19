/// ```
/// # 启动命令
///  cargo run --example string
/// ```
fn main() {
    let s1 = "World";
    println!("{s1}");
    let mut s2 = String::from("Hello ");
    println!("{s2}");
    s2.push_str(s1);
    println!("s2 is {}", s2);

    let s3 = &s2[s2.len() - s1.len()..];
    println!("s3 is {}", s3);
}
