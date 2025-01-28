/// ```
/// # 启动命令
///  cargo run --example more_traits
///
/// ```

trait Animal {
    fn leg_count(&self) -> u32;
}

// 继承Animal
trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog(String);

impl Animal for Dog {
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.0.clone()
    }
}

fn main() {
    let puppy = Dog(String::from("Rex"));
    println!("够够的名字：{}, 狗狗的年龄：{}", puppy.name(), puppy.leg_count());
}
