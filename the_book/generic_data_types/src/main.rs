fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointDetail<X1, Y1> {
    x: X1,
    y: Y1,
}
impl<X1, Y1> PointDetail<X1, Y1> {
    fn mixup<X2, Y2>(self, other: PointDetail<X2, Y2>) -> PointDetail<X1, Y2> {
        PointDetail {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    /* generic 関数 */
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    /* generic 構造体（メソッド定義） */
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    let p = Point { x: 5.0, y: 10.0 };
    println!("square = {}", p.distance_from_origin());

    /* generic 構造体(複数generic) */
    let p1 = PointDetail { x: 5, y: 10.4 };
    let p2 = PointDetail { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
