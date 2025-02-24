use std::collections::HashMap;

/// ```
/// # 启动命令
///  cargo run --example standard_hashmap
///
/// ```

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("Adventures of Huckleberry Finn", 207);
    page_counts.insert("Grimms' Fairy Tales", 751);
    page_counts.insert("Pride and Prejudice", 303);

    if !page_counts.contains_key("Les Misérables") {
        println!(
            "We know about {} books, but not Les Misérables.",
            page_counts.len()
        );
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count} pages"),
            None => println!("{book} is unknown."),
        }
    }

    for book in ["Pride and Prejudice", "Alice's Adventure in Wonderland"] {
        let page_count: &mut i32 = page_counts.entry(book).or_insert(0);
        *page_count += 1;
    }

    let pc1 = page_counts
        .get("Harry Potter and the Sorcerer's Stone")
        .unwrap_or(&336);
    println!("{pc1:?}");

    let pc2 = page_counts.entry("The Hunger Games").or_insert(374);
    println!("{pc2:?}");

    println!("{page_counts:#?}");

    // 数组初始化哈希映射
    let page_counts2 = HashMap::from([
        ("Harry Potter and the Sorcerer's Stone".to_string(), 336),
        ("The Hunger Games".to_string(), 374),
    ]);

    println!("{page_counts2:?}");
}
