/// ```
/// # 启动命令
///  cargo run --example arr_for
///
/// ```

fn main() {
    let arr = [7, 9, 22, 76, 15, 44, 3];
    for s in arr {
        for x in s..5 {
            dbg!(x);
        }
    }
}
