use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}

// 提取参数解析器
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

// cargo run -- the poem.txt