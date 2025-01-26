/// ```
/// # 启动命令
///  cargo run --example traits
///
/// ```

trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("输出内容: {}", self.talk())
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("name: {} , age: {}", self.name, self.age)
    }
}

fn main() {
    let fido = Dog {
        name: String::from("Fido"),
        age: 3,
    };
    fido.greet();
}
