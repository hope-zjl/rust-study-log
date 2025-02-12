/// ```
/// # 启动命令
///  cargo run --example standard_vec
///
/// ```

fn main() {
    let mut v1 = Vec::new();
    v1.push(42);
    println!("len: {}, capacity: {}", v1.len(), v1.capacity());

    let mut v2 = Vec::new();
    v2.extend(v1);
    v2.push(9999);
    println!("len: {}, capacity: {}", v2.len(), v2.capacity());

    let mut v3 = vec![0, 0, 1, 2, 3, 4];

    v3.retain(|x| x % 2 == 0);
    println!("{v3:?}");
    v3.dedup();
    println!("{v3:?}");
}
