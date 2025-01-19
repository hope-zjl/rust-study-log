/// ```  
/// # 启动命令
///  cargo run --example labels
///
/// ```

fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    // 标签的命名 《 '标签名: 》 在这里调用标签可以直接跳出所指定的循环
    'labe: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'labe;
            }
        }
    }
    println!("elements searched: {elements_searched}");
}
