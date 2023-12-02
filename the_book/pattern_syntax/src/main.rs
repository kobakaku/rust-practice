fn main() {
    // 参照を分配
    struct Point {
        x: i32,
        y: i32,
    }
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];
    let _sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();

    // 構造体とタプルを分配
    let ((_feet, _inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // アンダースコア
    let s = Some(String::from("Hello!"));
    // _sにmoveされるのでコンパイルエラー
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    //　..で値の残りの部分を無視する
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // ref・ref mutでパターンに参照を生成する
    let mut robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name),
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);

    // @束縛
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
