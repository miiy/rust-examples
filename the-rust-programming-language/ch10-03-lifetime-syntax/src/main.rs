// 生命周期确保引用有效
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // 生命周期避免了悬垂引用

    // 借用检查器

    // 一个有效的引用，因为数据比引用有这更长的生命周期
    lifetime1();

    // 函数中的泛型生命周期
    func_lifetime();

    // 生命周期注解语法

    // 函数签名中的生命周期注解
    func_sign_lifetime();

    // 深入理解生命周期
    deep_understanding_lifetime();

    // 结构体定义中的生命周期注解
    struct_lifetime();

    // 生命周期省略 (Lifetime Elision)
    lifetime_elision();

    // 方法定义中的生命周期注解
    method_lifetime();

    // 静态生命周期
    static_lifetime();

    // 结合泛型类型参数、trait bounds 和生命周期
    trait_lifetime();
}

// 生命周期避免了悬垂引用

// 尝试使用离开作用域的值的引用
// fn main() {
//     let r;
//     {
//         let x = 5;
//         r = &x;
//     }
//     println!("r: {}", r);
// }

// 借用检查器

// r 和 x 的生命周期注解，分别叫做 'a 和 'b
// fn main() {
//     let r;                // ---------+-- 'a
//                           //          |
//     {                     //          |
//         let x = 5;        // -+-- 'b  |
//         r = &x;           //  |       |
//     }                     // -+       |
//                           //          |
//     println!("r: {}", r); //          |
// }                         // ---------+

// 一个有效的引用，因为数据比引用有这更长的生命周期
fn lifetime1() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

// 函数中的泛型生命周期
fn func_lifetime() {
    // // main 函数调用 longest 函数来寻找两个字符串 slice 中较长的一个
    // fn main() {
    //     let string1 = String::from("abcd");
    //     let string2 = String::from("xyz");

    //     let result = longest(string1.as_str(), string2);
    //     println!("The longest string is {}", result);
    // }

    // // 一个 longest 函数的实现，它返回两个字符串 slice 中较长者，现在还不能编译
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
}

// 生命周期注解语法
// &i32        // 引用
// &'a i32     // 带有显示生命周期的引用
// &'a mut i32 // 带有显示生命周期的可变引用

// 函数签名中的生命周期注解
fn func_sign_lifetime() {
    // longest 函数定义指定了签名中所有的引用必须有相同的生命周期 'a
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // 通过拥有不同的具体生命周期的 String 值调用 longest 函数
    fn main() {
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    // 尝试在 string2 离开作用域之后使用 result
    // fn main() {
    //     let string1 = String::from("long string is long");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     println!("The longest string is {}", result);
    // }
}

// 深入理解生命周期
fn deep_understanding_lifetime() {
    fn longest<'a>(x: &'a str, y: &str) -> &'a str {
        x
    }

    // 返回值的生命周期与参数完全没有关联
    // fn longest<'a>(x: &str, y: &str) -> &'a str {
    //     let result = String::from("really long string");
    //     result.as_str()
    // }
}

// 结构体定义中的生命周期注解
fn struct_lifetime() {
    struct ImportantExcerpt<'a> {
        part: &'a str
    }

    // 一个存放引用的结构体，所以其定义需要生命周期注解
    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split(".").next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    main();
}

// 生命周期省略 (Lifetime Elision)
fn lifetime_elision() {
    // 定义了一个没有使用生命周期注解的函数，即便其参数和返回值都是引用
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i]
            }
        }
        &s[..]
    }
    // 编译器采⽤三条规则来判断引⽤何时不需要明确的注解。
    // 第⼀条规则是编译器为每⼀个引⽤参数都分配⼀个⽣命周期参数。
    // 第⼆条规则是如果只有⼀个输⼊⽣命周期参数，那么它被赋予所有输出⽣命周期参数。
    // 第三条规则是如果⽅法有多个输⼊⽣命周期参数并且其中⼀个参数是 或 &mut self ，说明是个对象的⽅法 (method)(译者注：这⾥涉及 rust 的⾯向对象参⻅17 &self 章) ，那么所有输出⽣命周期参数被赋予 self 的⽣命周期。
}

// 方法定义中的生命周期注解
fn method_lifetime() {
    struct ImportantExcerpt<'a> {
        part: &'a str
    }

    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }
    }

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
}

// 静态生命周期
fn static_lifetime() {
    let s: &'static str = "I have a static lifetime.";
}

// 结合泛型类型参数、trait bounds 和生命周期
fn trait_lifetime() {
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}