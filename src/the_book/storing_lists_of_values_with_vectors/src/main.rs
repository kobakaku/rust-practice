fn main() {
    let v = vec![1, 2, 3, 4, 5];

    /* vecの値を取得する方法*/

    // 1. indexを指定する
    let third = &v[2];
    println!("The third element is {}", third);

    // 2. getメソッドを使用
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    /* 可変と不変の参照は同時に存在できない */
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    // Vectorのループ処理 (可変処理も可能) ※参照外しを行っている
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // 複数の型をもつ必要がある場合、enumを使う例
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
