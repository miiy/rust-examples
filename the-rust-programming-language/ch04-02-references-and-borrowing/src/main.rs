// 引用与借用
#![allow(unused_variables)]

fn main() {
    // 引用
    reference_example();

    // 尝试修改借用的值
    borrowing_example();

    // 可变引用
    mut_example();

    // 可变引用的限制
    change_2mut();

    change_2mut_scope();

    // 不能再拥有不可变引用的同时拥有可变引用
    reference_mut();

    reference_mut2();

    // 悬垂引用
    reference_to_nothing();
}

// 引用
fn reference_example() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// 尝试修改借用的值
fn borrowing_example() {
    let s = String::from("hello");

    // change(&s);
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// 可变引用
fn mut_example () {
    let mut s = String::from("hello");

    mut_change(&mut s);
}

fn mut_change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 可变引用有一个很大的限制：如果你有一个对该变量的可变引用，你就不能再创建对该变量的引用。
fn change_2mut() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2)
}

// 使用大括号来创建一个新的作用域，以允许多个可变引用，只是不能同时拥有
fn change_2mut_scope() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

// 不能再拥有不可变引用的同时拥有可变引用
#[allow(unused_mut)]
fn reference_mut() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);
}

fn reference_mut2() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3)
}

// 悬垂引用
fn reference_to_nothing () {
    // 创建一个悬垂引用，Rust 会通过一个编译时错误来避免：通过生命周期（lifetimes）解决
    // let reference_to_nothing = dangle();

    let reference_to_nothing = no_dangle();
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用

//     let s = String::from("hello"); // s 是一个新字符串

//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃，其内存被释放。
//   // 危险！

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}