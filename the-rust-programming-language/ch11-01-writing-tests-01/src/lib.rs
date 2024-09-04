// 如何编写测试

// 测试函数剖析
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// 使用 cargo 新建一个库项目时，它会自动生成一个测试模块和一个测试函数
// cargo new ch11-01-writing-tests-01 --lib
//
// 运行自动生成测试的输出
// cargo test