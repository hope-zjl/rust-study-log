/// ```  
/// # 启动命令
///  cargo run --example cycle
///
/// ```

fn main() {
    // while
    while_cycle();
    // for
    for_cycle();
    // loop
    loop_cycle();
}

fn while_cycle() {
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    println!("Final x: {x}");
}

fn for_cycle() {
    for x in 1..5 {
        println!("x: {x}");
    }

    for elem in [1, 2, 3, 4, 5] {
        println!("elem: {elem}");
    }
}

fn loop_cycle() {
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break; // break  跳出循环
        }
    }
}
