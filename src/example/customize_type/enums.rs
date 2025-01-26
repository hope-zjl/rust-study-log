/// ```
/// # 启动命令 
///  cargo run --example enums
///
/// ```

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,
    Run(Direction),
    Teleport { x: u32, y: u32 },
}

fn main() {
    let m = PlayerMove::Run(Direction::Left);
    println!("{:?}", m);

    let m = PlayerMove::Teleport { x: 1, y: 2 };
    println!("{:?}", m);

    let m = PlayerMove::Pass;
    println!("{:?}", m);
}
