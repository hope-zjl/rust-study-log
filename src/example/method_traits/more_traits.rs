/// ```
/// # 启动命令
///  cargo run --example more_traits
///
/// ```

trait Animal {
    // 定义接口
    fn leg_count(&self) -> u32;
}

// 继承Animal
trait Pet: Animal {
    fn name(&self) -> String;
}

// 定义结构体
struct Dog(String);

impl Animal for Dog {
    // 实现接口
    fn leg_count(&self) -> u32 {
        4
    }
}

impl Pet for Dog {
    // 实现接口
    fn name(&self) -> String {
        self.0.clone()
    }
}

fn main() {
    let puppy = Dog(String::from("Rex"));
    println!("够够的名字：{}, 狗狗的年龄：{}", puppy.name(), puppy.leg_count());
}

fn _mores() {
    todo!("需要多练习")
}
