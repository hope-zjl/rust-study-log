/// ```
/// # 启动命令
///  cargo run --example generics_impl_traits
///
/// ```

fn add_42_millions(x: impl Into<i32>) -> i32 {
    x.into() + 42_000_000
}

fn pair_of(x: i32) -> impl std::fmt::Debug {
    (x + 1, x - 1)
}

fn main() {
    let many = add_42_millions(42_i8);
    println!("{many}");
    let many_more = add_42_millions(10_000_000);
    println!("{many_more}");
    let debuggable = pair_of(27);
    println!("{debuggable:?}");
}


fn _more() {
    todo!("需要多练习   * 这里有很多我都是跟文档写了 有些代码我目前还是看不懂 * ")
}