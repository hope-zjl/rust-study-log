struct Foo {
    x: (u32, u32),
    y: u32,
}

fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = {}, y = {}, b = {} ", foo.x.0, y, b),
        Foo { y: 2, x: i } => println!(
            "y: {}, x.0: {}, x.1: {}, i: {:?}",
            foo.y, foo.x.0, foo.x.1, i
        ),
        Foo { y: 3, .. } => println!("y: {}, x.0: {}", foo.y, foo.x.0),
        _ => {}
    }
}
