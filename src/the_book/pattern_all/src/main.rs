fn main() {
    let v = vec!["a", "b", "c"];
    for (index, value) in v.iter().enumerate() {
        println!("{} is as index {}", value, index);
    }
}
