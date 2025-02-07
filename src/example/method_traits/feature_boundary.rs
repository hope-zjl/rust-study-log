/// ```
/// # 启动命令
///  cargo run --example feature_boundary
///
/// ```

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

fn main() {
    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");
}
