// slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。slice 是一种引用，所以它没有所有权。

fn main() {
    let mut s = String::from("hello world");

    let world = first_word(&s);

    s.clear();

    string_slice()
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

fn string_slice() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}