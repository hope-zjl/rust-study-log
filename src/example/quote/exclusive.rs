/// ```
/// # 启动命令
///  cargo run --example exclusive
///
/// ```

fn main() {
    let mut po = (2, 7);
    let x_coo = &mut po.0;
    // po.0 = 12;   ps:错误实力  在引用期间 x_coo不允许被修改
    *x_coo = 2;
    println!("x_coo: {:?}", x_coo);
}
