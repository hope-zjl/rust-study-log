/// ```
/// # 启动命令
///  cargo run --example standard_string
///
/// ```

fn main() {
    let mut s1 = String::new();
    s1.push_str("Hello");
    println!("len: {}, capacity:{}", s1.len(), s1.capacity());

    let mut s2 = String::with_capacity(s1.capacity() + 1);
    s2.push_str(&s1);
    s2.push('!');
    println!("len: {}, capacity:{}", s2.len(), s2.capacity());

    let s3 = String::from("CH");
    println!("len: {}, count:{}", s3.len(), s3.chars().count());
}
