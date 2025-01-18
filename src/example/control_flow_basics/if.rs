/// ```  
/// # 启动命令
/// cargo run --example if
/// ```
fn main() {
    example_1();
    example_2();
}

// 示例 1
fn example_1() {
    let x = 15;

    if x == 0 {
        println!("zero!");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }
}
// 示例 2 if 表达式中必须包含 ;
fn example_2() {
    let x = 10;
    let size = if x < 20 { "smail" } else { "large" }; // 必须包含 ;
    println!("number size: {}", size);
}
