/// ```
/// # 启动命令
///  cargo run --example tup_structs
///
/// ```

struct Point(u32, u32);

struct PoundsOfForce(f64);
struct Newtons(f64);

fn compute_thruster_force() -> PoundsOfForce {
    PoundsOfForce(77.6)
}

impl PoundsOfForce {
    fn to_newtons(&self) -> Newtons {
        Newtons(self.0 * 1.8)
    }
}

fn set_thruster_force(force: Newtons) {
    println!("force: {}", force.0);
}

fn main() {
    let p = Point(7, 50);
    println!("p1: {},p2: {}", p.0, p.1);

    let force = compute_thruster_force();
    set_thruster_force(force.to_newtons());
}
