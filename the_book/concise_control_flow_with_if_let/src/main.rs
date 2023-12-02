fn main() {
    let some_u8_value = Some(0u8);
    // 1と2は同義
    /* 1 */
    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("not three"),
    }

    /* 2 */
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("not three");
    }
}
