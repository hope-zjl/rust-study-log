/// ```
/// # 启动命令
///  cargo run --example consts
///
/// ```

const DIGEST_SIZE: usize = 3;
const ZERO: Option<u8> = Some(42);

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {:?}", digest);
}

fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
    for (idx, &b) in text.as_bytes().iter().enumerate() {
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
    digest
}

fn _more() {
    todo!("需要更多练习")
}
