fn main() {
    let my_string_literal = "hello world";
    let first_word_result = first_word(my_string_literal);
    // 別解
    // let first_word_result = first_word(&my_string_literal[..]);
    println!("First word: {}", first_word_result);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
