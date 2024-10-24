use time::OffsetDateTime;
use serde::{Serialize};
use serde_json;

fn main() {
    example1();
    struct_example();
    json_example();
}

fn example1() {
    let now = OffsetDateTime::now_utc();
    // let local = OffsetDateTime::now_local();

    println!("{now}");
}


fn struct_example() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct User {
        id: u64,
        name: String,
        created_at: time::OffsetDateTime,
    }

    let u = User {
        id: 1,
        name: "test".to_string(),
        created_at: OffsetDateTime::now_utc(),
    };

    println!("{:?}", u);
}

fn json_example() {
    #[allow(dead_code)]
    #[derive(Debug, Serialize)]
    struct User {
        id: u64,
        name: String,
        created_at: time::OffsetDateTime,
    }

    let u = User {
        id: 1,
        name: "test".to_string(),
        created_at: OffsetDateTime::now_utc(),
    };

    let serialized = serde_json::to_string(&u).unwrap();
    println!("{}", serialized);
}