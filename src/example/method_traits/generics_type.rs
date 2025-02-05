/// ```
/// # 启动命令
///  cargo run --example generics_type
///
/// ```

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn coords(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

fn main() {
    let integer = Point { x: 1, y: 2 };
    let float = Point { x: 1.0, y: 2.0 };

    println!("{integer:?} and {float:?}");
    println!("{:?}, {:?}", integer.coords(), float.coords());
}
