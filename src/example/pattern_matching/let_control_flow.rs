/// ```
/// # 启动命令
///  cargo run --example let_control_flow
///
/// ```

fn main() {
    sleep_for(-10.0);
    sleep_for(0.0);
}

fn sleep_for(secs: f64) {
    if let Ok(dur) = std::time::Duration::try_from_secs_f64(secs) {
        std::thread::sleep(dur);
        println!("Sleeping for {} seconds", secs);
    }
}
