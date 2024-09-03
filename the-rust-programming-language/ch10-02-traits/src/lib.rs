// 定义 trait
// Summary trait 定义，它包含由 summarize 方法提供的行为
pub trait Summary {
    fn summarize(&self) -> String;
}

// 为类实现 trait
// 在 NewsArticle 和 Tweet 类型上实现 Summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
