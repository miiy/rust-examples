#![allow(dead_code)]
#![allow(unused_variables)]

use std::vec;

// 另一个使用gui 的 crate 中，在 SelectBox 结构体上实现 Draw trait
use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// 使用 trait 对象来存储实现了相同 trait 的不同类型的值
use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

// 尝试使用一种没有实现 trait 对象的 trait 类型
// fn main() {
//     let screen = Screen {
//         components: vec![
//             Box::new(String::from("Hi")),
//         ]
//     };
// }
