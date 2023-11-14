#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 複数に分割可能
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.set_width(40);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
    println!("The area of the rectangle is {} square pixels.", sq.area());
}
