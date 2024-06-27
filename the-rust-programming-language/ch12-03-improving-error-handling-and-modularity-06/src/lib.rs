use std::error::Error;
use std::fs;


// 组合配置值
pub struct Config {
    pub query: String,
    pub file_path: String,
}

// 创建一个 Config 的构造函数
impl Config {
    // 从 Config::build 中返回 Result
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");
    Ok(())
}
