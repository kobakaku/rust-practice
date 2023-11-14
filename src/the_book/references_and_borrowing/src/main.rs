fn main() {
    // 参照を渡すことによって、String値を変更可
    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("{}", s1);

    // sはdangle関数が完了したら開放されてしまう。その参照を返しているためコンパイルエラーが起こる
    let reference_to_nothing = dangle();

    // 2つ以上可変の参照を持てない
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);

    // 普遍の参照がある場合、可変の参照を持つことはできない
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
