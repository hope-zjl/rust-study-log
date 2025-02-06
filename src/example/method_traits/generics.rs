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

fn main() {
    let u = Foo::from(20);
    let s = Foo::from("hello".to_string());

    println!("{u:?}, {s:?}, \n {}, \n {}", u.0, s.0);
}
