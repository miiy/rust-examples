use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");

    if let Some(value) = map.get("key") {
        println!("Found: {}", value);
    } else {
        println!("Key not found");
    }

    println!("map: {:?}", map);
}
