fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x: u32 = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    // shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("shadowing: {}", spaces);

    // compile error
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("shadowing: {}", spaces);
}
