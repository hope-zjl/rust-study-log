/// ```  
/// # 启动命令
/// cargo run --example derivation
/// ```
fn takes_u32(x: u32) {
    println!("u32: {x}");
}

fn takes_i8(y: i8) {
    println!("i8: {y}");
}

fn main() {
    // 不定义类型 类型可以自动推算
    let x = 10;
    let y = 20;

    takes_u32(x);
    takes_i8(y);
}
