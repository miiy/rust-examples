// 自定义失败信息

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn greeting2(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contains_name2() {
        let result = greeting2("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        )
    }
}
