/// ```
/// # 启动命令
///  cargo run --example let_control_flow
///
/// ```

fn main() {
    sleep_for(-10.0);
    sleep_for(0.0);
    println!("result: {:?}", hex_or_die_trying(Some(String::from("foo"))));
}

fn sleep_for(secs: f64) {
    if let Ok(dur) = std::time::Duration::try_from_secs_f64(secs) {
        std::thread::sleep(dur);
        println!("Sleeping for {} seconds", secs);
    }
}

fn hex_or_die_trying(maybe_string: Option<String>) -> Result<u32, String> {
    let s = if let Some(s) = maybe_string {
        s
    } else {
        return Err(String::from("Must supply a string"));
    };

    let first_byte_char = if let Some(first_byte_char) = s.chars().next() {
        first_byte_char
    } else {
        return Err(String::from("Must be one byte char"));
    };

    if let Some(digit) = first_byte_char.to_digit(16) {
        Ok(digit)
    } else {
        Err(String::from("Must be 16 digits"))
    }
}
