// 控制测试如何运行

// 并行或连续的运行测试
// cargo test -- --test-threads=1

// 显示函数输出
// 一个调用了 println! 的函数的测试
#[allow(dead_code)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value)
    }
}
// cargo test -- --show-output