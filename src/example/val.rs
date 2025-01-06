/// ```  
/// # 启动命令
/// cargo run --example val
/// ```
fn main() {
    // 有符号
    let a: i8 = -9;
    println!("a: {a}");
    // 无符号
    let u: u8 = 9;
    println!("u: {u}");
    // 浮点数
    let f: f32 = 3.14;
    println!("f: {f}");
    // 浮点数
    let c: char = '😂';
    println!("c: {c}");
    // 浮点数
    let b: bool = 1 == 2;
    println!("b: {b}");
    // 拓展
    let num: u32 = 1_000_000; // 方便阅读
    println!("num: {num}");
    let inum: i64 = 666i64;
    println!("inum: {inum}");
}