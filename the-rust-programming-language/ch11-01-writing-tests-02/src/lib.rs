// 增加第二个因调用了 panic! 而失败的测试
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        panic!("Mke this test fail");
    }
}

// cargo test