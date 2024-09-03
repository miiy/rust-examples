// Trait：定义共同行为
// trait 定义了某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共同行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。
// trait 类似于其他语言中的常被称为 接口 (interface) 的功能，虽然有一些不同。
#![allow(unused_variables)]

use std::fmt::{Debug, Display};
use aggregator::{Summary, Tweet};

fn main() {
    // 定义 Trait
    // lib.rs

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    // 默认实现
    default_trait();

    // Trait bound 语法
    trait_bound();

    // 通过 + 指定多个 trait bound
    trait_bound2();

    // 通过 where 简化 trait bound
    trait_bound3();

    // 返回实现了 trait 的类型
    return_trait();

    // 根据 trait bound 在泛型上有条件的实现方法
    trait_bound4();
}

// 默认实现
fn default_trait() {
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    pub trait Summary {
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    impl Summary for NewsArticle {}

    let article = NewsArticle {
        headline: String::from("Penguins win Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize())
}

// trait 作为参数
// item 参数支持任何实现了指定 trait 的类型
#[allow(dead_code)]
fn trait_params() {
    pub fn notify (item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
}

// Trait Bound 语法
// impl Trait 很方便，适用于短小的例子
#[allow(dead_code)]
fn trait_bound() {
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify2(item1: &impl Summary, item2: &impl Summary) {
        println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
    }

    pub fn notify3<T: Summary>(item1: &T, item2: &T) {
        println!("Breaking news! {}, {}", item1.summarize(), item2.summarize());
    }
}

// 通过 + 指定多个 trait bound
#[allow(dead_code)]
fn trait_bound2() {
    pub fn notify(item: &(impl Summary + Display)) {
        println!("Breaking news! {}", item.summarize());
    }

    pub fn notify2<T: Summary + Display>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }
}

// 通过 where 简化 trait bound
#[allow(dead_code)]
fn trait_bound3() {
    fn some_function<T: Default + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
        return 0
    }

    fn some_function2<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
    {
        return 0
    }
}

// 返回实现了 trait 的类型
#[allow(dead_code)]
fn return_trait() {
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    // 这只适用于返回单一类型的情况。例如，这段代码的返回值类型指定为返回 impl Summary，但是返回了 NewsArticle 或 Tweet 就⾏不通：
    // fn returns_summarizable2(switch: bool) -> impl Summary {
    //     if switch {
    //         NewsArticle {
    //             headline: String::from("Penguins win Stanley Cup Championship!"),
    //             location: String::from("Pittsburgh, PA, USA"),
    //             author: String::from("Iceburgh"),
    //             content: String::from(
    //                 "The Pittsburgh Penguins once again are the best \
    //                 hockey team in the NHL.",
    //             ),
    //         }
    //     } else {
    //         Tweet {
    //             username: String::from("horse_ebooks"),
    //             content: String::from(
    //                 "of course, as you probably already know, people",
    //             ),
    //             reply: false,
    //             retweet: false,
    //         }
    //     }
    // }
}

// 使用 trait bound 有条件地实现方法
// 根据 trait bound 在泛型上有条件的实现方法
#[allow(dead_code)]
fn trait_bound4() {
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

    impl <T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x)
            } else {
                println!("The largest member is y = {}", self.y)
            }
        }
    }
}