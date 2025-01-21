/// ```
/// # 启动命令
///  cargo run --example tup_structs
///
/// ```

#[derive(Debug)]
enum CarryableConcreteItem {
    Left,
    Right,
}

type Item = CarryableConcreteItem;

fn main() {
    println!("{:?}", CarryableConcreteItem::Left);
    println!("{:?}", CarryableConcreteItem::Right);
    println!("{:?}", Item::Right);
    println!("{:?}", Item::Left);
}
