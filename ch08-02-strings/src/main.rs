fn main() {
    // 新建字符串

    // 使用 new 函数
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");
    // 字符串字面值使用to_string()方法
    let s = "initial contents".to_string();
    println!("{s}");
    // 使用 String::from函数
    let s = String::from("initial contents");
    println!("{s}");

    // 更新字符串
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    // 内部表现
    index_string();
    // 遍历字符串
    fro_in_string();
}

fn index_string() {
    let hello = "3дравствуйте";
    // let answer = &hello[0];
}

fn fro_in_string() {
    // 字符
    for c in "3д".chars() {
        println!("{c}")
    }
    // 字节
    for b in "3д".bytes() {
        println!("{b}")
    }
}
