// 将错误信息输出到标准错误而不是标准输出
use std::env;
use std::process;

use minigrep::Config;

// 将错误打印到标准错误
// 使用 eprintln! 将错误信息写入标准错误而不是标准输出
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


// cargo run -- the poem.txt