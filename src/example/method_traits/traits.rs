/// ```
/// # 启动命令
///  cargo run --example traits
///
/// ```

trait Pet {
    // 接口
    fn talk(&self) -> String;
    fn greet(&self) {
        println!("输出内容: {}", self.talk())
    }
}

// 结构体
struct Dog {
    name: String,
    age: i8,
}

// 方法的实现
impl Pet for Dog {
    // 用来实现接口的具体行为
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

fn _more() {
    todo!("需要更多练习")
}
