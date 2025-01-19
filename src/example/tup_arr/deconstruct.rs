/// ```
/// # 启动命令
///  cargo run --example deconstruct
///
/// ```

fn main() {
    dup_deconstruct((15, 20));
    deconstruct_dup((70, 80));
}

fn dup_deconstruct(tup: (i32, i32)) {
    let left = tup.0;
    let right = tup.1;
    println!("left1: {left}, right1: {right}");
}

fn deconstruct_dup(tup: (i32, i32)) {
    let (left, right) = tup;
    println!("left2: {left}, right2: {right}");
}
