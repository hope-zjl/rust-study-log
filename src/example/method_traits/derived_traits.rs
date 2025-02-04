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

    multiple_n(7, 4);
}

fn multiple_n(n: usize, init: usize) {
    for i in init..201 {
        if i % n == 0 && str_cont(i, n) {
            println!("过");
        }
    }
}

fn str_cont(x1: usize, x2: usize) -> bool {
    if x1.to_string().contains(&x2.to_string()) {
        true
    } else {
        false
    }
}
