// 闭包类型推断和注解
use std::thread;
use std::time::Duration;

fn main() {
    // 为闭包的参数和返回值增加可选的类型注解
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let num = expensive_closure(1);
    println!("{num}");

}

// 闭包语法与函数语法的相似性
// fn  add_one_v1   (x: u32) -> u32 { x + 1 }
// let add_one_v2 = |x: u32| -> u32 { x + 1 };
// let add_one_v3 = |x|             { x + 1 };
// let add_one_v4 = |x|               x + 1  ;


// 尝试调用一个呗推断为两个不同类型的闭包
// fn closure_different_type() {
//     let expensive_closure = |x| x;

//     let s = expensive_closure(String::from("hello"));
//     let n = expensive_closure(5);
// }