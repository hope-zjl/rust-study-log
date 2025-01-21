/// ```
/// # 启动命令
///  cargo run --example matching_values
///
/// ```

fn main() {
    let input = 'X';

    match input {
        'q' => println!("1"),
        'C' | 'S' | 'E' => println!("2"),
        '0'..'9' => println!("3"),
        key if key.is_lowercase() => println!("4"),
        _ => println!("5"),
    }
}