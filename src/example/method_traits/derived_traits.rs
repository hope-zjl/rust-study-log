/// ```
/// # 启动命令
///  cargo run --example derived_traits
///
/// ```

#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default();
    let mut p2 = p1.clone();
    p2.name = String::from("测试");
    p2.strength = 7;
    p2.hit_points = 9;
    println!("{:?}, {:?}", p1, p2);
}