fn main() {
    // コンパイルエラー "value borrowed here after move"
    // let s1 = String::from("hello");
    let s1 = 10;
    let s2 = s1;

    println!("{}, world!", s1);

    // ヒープデータをコピー
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
