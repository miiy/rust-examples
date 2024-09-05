// 重构改进模块性和错误处理
use std::env;
use std::fs;

// 提取参数解析器

// 从 main 中提取出 parse_config 函数
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);
    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

// cargo run -- the poem.txt