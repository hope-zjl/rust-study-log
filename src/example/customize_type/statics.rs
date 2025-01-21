/// ```
/// # 启动命令
///  cargo run --example statics
///
/// ```

static BANNER: &str = "Welcome to RustOS 3.14"; // static不能自动推断类型

fn main() {
    println!("{BANNER}");
}
