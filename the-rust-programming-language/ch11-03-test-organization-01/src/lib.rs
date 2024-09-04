// 测试的组织结构

// 单元测试

// 测试模块和 #[cfg(test)]
// 测试模块的 #[cfg(test)] 注解告诉 Rust 只在执⾏ cargo test 时才编译和运⾏测试代码

// 测试私有函数
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2))
    }
}