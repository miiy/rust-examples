// 通过指定名字来运行部分测试

// 不同名称的三个测试
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100))
    }
}

// 运行单个测试
// cargo test one_hundred

// 过滤运行多个测试
// cargo test add