use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args);
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}")
}

// 组合配置值
struct Config {
    query: String,
    file_path: String,
}

// 创建一个 Config 的构造函数
impl Config {
    fn new(args: &[String]) -> Config {
        // 增加一个参数数量检查
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }

    // 从 Config::build 中返回 Result
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}

// cargo run -- the poem.txt