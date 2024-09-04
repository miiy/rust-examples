// 除非特别指定否则忽略某些测试

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // 需要运行一个小时的代码
    }
}
// 运行测试，排除 ignore
// cargo test

// 只运行被忽略的测试
// cargo test -- --ignored

// 运行全部测试
// cargo test -- --include-ignored