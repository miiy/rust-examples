use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    let team = "Yellow";
    let score = scores.get(team).copied().unwrap_or(0);
    println!("{score}")
}
