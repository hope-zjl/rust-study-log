/// ```  
/// # 启动命令
/// cargo run --example scope
/// ```
fn main() {
    let z = 10;
    let x = {
        let y = 9;
        println!("y: {y}");
        z - y
    };
    println!("x: {x}");
    shadowing();
}

// 作用域喝遮蔽
fn shadowing() {
    let a = 10;
    {
        let a = "Hello";
        println!("a: {a}");

        let a = true;
        println!("a:{a}");
    }
    println!("a: {a}");
}
