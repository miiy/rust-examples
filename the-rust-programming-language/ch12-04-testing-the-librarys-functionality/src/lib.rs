use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}

// 在 run 函数中使用 search 函数
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}")
    }
    Ok(())
}

// 刚好足够使测试通过编译的 search 函数定义
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// 编写使测试通过的代码
// 使用 lines 遍历 contents 的每一行
// 增加检查文本行是否包含 query 中字符串的功能
// 存储匹配的行一遍可以返回它们
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 创建一个我们期望的 search 函数的失败测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick there.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
