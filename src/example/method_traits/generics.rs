/// ```
/// # 启动命令
///  cargo run --example generics
///
/// ```

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(x: u32) -> Foo {
        Foo(format!("u32:  {}", x))
    }
}

impl From<String> for Foo {
    fn from(x: String) -> Foo {
        Foo(format!("String:  {}", x))
    }
}

impl From<bool> for Foo {
    fn from(x: bool) -> Foo {
        Foo(format!("bool:  {}", x))
    }
}

fn main() {
    let u = Foo::from(20);
    let s = Foo::from("hello".to_string());
    let b = Foo::from(true);

    println!("{u:?}, {s:?}, {b:?}, \n {}, \n {}, \n {}", u.0, s.0, b.0);
}
