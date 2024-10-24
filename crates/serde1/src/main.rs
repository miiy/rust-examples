use serde::{Deserialize, Serialize};

fn main() {
    example1();
    option_serialized();
    option_deserialize();
}

fn example1() {
    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point{ x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    println!("");
}

fn option_serialized() {
    #[derive(Debug, Default, Serialize)]
    struct Point {
        x: i32,
        y: Option<i32>,
        z: Option<i32>,
    }

    let point = Point{ x: 1, y: Some(0), z: None };
    println!("{:?}", point);
    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);
    println!("");
}

fn option_deserialize() {
    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Point {
        x: i32,
        y: Option<i32>,
        z: Option<i32>,
    }

    let data = r#"{"x":1, "y": 2}"#;
    let deserialized: Point = serde_json::from_str(&data).unwrap();
    println!("data = {}", data);
    println!("deserialized = {:?}", deserialized);
    println!("");
}