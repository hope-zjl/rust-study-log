use std::fs::File;
use std::io::Read;

/// ```
/// # 启动命令
///  cargo run --example standard_results
///
/// ```

fn main() {
    let file = File::open("README.md");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Error reading file");
            }
        }
        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }
}
