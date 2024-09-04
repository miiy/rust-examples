// 接受命令行参数

// 读取参数值
use std::env;

fn main() {
    // 将命令行参数收集到一个 vector 中并打印出来
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    // 将参数值保存进变量
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

// cargo run
// cargo run -- needle haystack
// cargo run -- test sample.txt