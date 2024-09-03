// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。
#![allow(unused_variables)]

fn main() {
    // 存储 first_word 函数调用的返回值并接着改变 String 的内容
    first_word_example();

    // 字符串 slice
    string_slice();

    first_word2_example();

    // 字符串 slice 作为参数
    slice_params();
}

// first_word 函数返回 String 参数的一个字节索引值
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// 存储 first_word 函数调用的返回值并接着改变 String 的内容
fn first_word_example() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word 的值为 5

    s.clear(); // 这里清空了字符串，使其等于 ""

    // word 在此处的值仍然是 5，
    // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}

// 字符串 slice
fn string_slice() {
    // 字符串 slice 是 String 中一部分值的引用
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

// 重写 first_word 返回一个 slice
fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

#[allow(unused_mut)]
fn first_word2_example() {
    let mut s = String::from("hello world");

    let word = first_word2(&s);

    // s.clear(); // 错误

    // println!("the first word is: {}", word)
}

// 字符串字面值就是 slice
// let s = "Hello, world!";

// 字符串 slice 作为参数
// fn first_word(s: &String) -> &str {
//
// 通过将 s 参数的类型改为字符串 slice 来改进 first_word 函数
// fn first_word(s: &str) -> &str {

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_params() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），部分或全部
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word` 适用于 `String` 的引用
    // 这等价于整个 `String` 的 slice
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，部分或全部
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，这也是适用的，无需 slice 语法！
    let word = first_word3(my_string_literal);
}
