fn main() {
    /* String型の文字列追加 */
    let mut s = String::from("foo");
    // 複数文字追加可能
    s.push_str("bar");
    // 1文字追加可能
    s.push('!');
    println!("{}", s);

    /* 文字列の結合  */
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // s3にs1の所有権が移動
    let s3 = s1 + &s2;
    // format!("{}-{}-{}", s1, s2, s3);

    /* 文字列のindexアクセス */
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}
