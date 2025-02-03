/// ```
/// # 启动命令
///  cargo run --example shared_types
///
/// ```

#[derive(Debug)]
struct Meters(i32);
#[derive(Debug)]
struct MetersSquared(i32);

// 接口
trait Multiply {
    type Output;
    fn multiply(&self, other: &Self) -> Self::Output;
}

// 接口实现
impl Multiply for Meters {
    type Output = MetersSquared;

    fn multiply(&self, other: &Self) -> Self::Output {
        MetersSquared(self.0 * other.0)
    }
}

fn main() {
    println!("{:?}", Meters(10).multiply(&Meters(20)));
    println!("{:?}", MetersSquared(20).0);
}

fn _mores() {
    todo!("更多练习")
}