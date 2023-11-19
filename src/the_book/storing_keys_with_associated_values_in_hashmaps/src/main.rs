fn main() {
    use std::collections::HashMap;

    /* Hashmapを作成する方法 */
    // 1
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);
    let score = scores.get("Blue");
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 2
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}
