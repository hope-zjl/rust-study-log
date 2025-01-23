/// ```
/// # 启动命令
///  cargo run --example deconstruct_match
///
/// ```

struct Foo {
    x: (u32, u32),
    y: u32,
}

enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("n is divisible by 2{n}"))
    }
}

fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = {}, y = {}, b = {} ", foo.x.0, y, b),
        Foo { y: 2, x: i } => println!(
            "y: {}, x.0: {}, x.1: {}, i: {:?}",
            foo.y, foo.x.0, foo.x.1, i
        ),
        Foo { y: 3, .. } => println!("y: {}, x.0: {}", foo.y, foo.x.0),
        _ => {}
    }

    let n = 100;
    match divide_in_two(n) {
        Result::Ok(result) => println!("result: {}", result),
        Result::Err(e) => println!("error: {}", e),
    }
}
