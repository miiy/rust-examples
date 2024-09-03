#![allow(unused_variables)]

fn main() {
    // 新建字符串
    create_string();

    // 更新字符串
    update_string();

    // 索引字符串
    index_string();

    // 字符串 slice
    string_slice();

    // 遍历字符串
    traversal_string();
}

// 新建字符串
fn create_string() {
    // 新建一个空的 String
    let s = String::new();

    // 使用 to_string 方法从字符串字面值创建 String
    let data = "initial contents";
    let s = data.to_string();

    // 该方法也可直接用于字符串字面值
    let s = "initial contents".to_string();

    // 使用 String::from 函数从字符串字面值创建 String
    let s = String::from("initial contents");
}

// 更新字符串
fn update_string() {
    // 使用 push_str 和 push 附加字符串 slice
    let mut s = String::from("foo");
    s.push_str("bar");

    // 将字符串 slice 的内容附加到 String 后使用它
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // 使用 push 将一个字符加入 String 值中
    let mut s = String::from("lo");
    s.push('l');

    // 使用 + 运算符将两个 String 值合并到一个新的 String 中
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
}

// 索引字符串
fn index_string() {
    // 尝试对字符串使用索引语法
    let s1 = String::from("hello");
    // let h = s1[0]; // Rust 字符串不支持索引

    // 内部变现
    let hello = "Hola";
    let hello = String::from("3дравствуйте");
    let hello = "3дравствуйте";
    // let answer = &hello[0];
}

// 字符串 slice
fn string_slice() {
    let hello = "3дравствуйте";
    // let s = &hello[0..4];
}

// 遍历字符串的方法
fn traversal_string() {
    // 字符
    for c in "3д".chars() {
        println!("{c}")
    }
    // 字节
    for b in "3д".bytes() {
        println!("{b}")
    }
}
