#![allow(unused_variables)]
use std::collections::HashMap;

fn main() {
    // 新建一个哈希 map
    create_map();

    // 访问哈希 map 中的值
    get_map();

    // 遍历哈希 map
    traversal_map();

    // 哈希 map 和所有权
    map_owner();

    // 更新哈希 map
    update_map();
}

// 新建一个哈希map
fn create_map() {
    // 使用 new 创建一个空的 HashMap，并使用 insert 增加元素。
    // 所有键必须是相同类型，值也必须是相同类型。
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

// 访问哈希 map 中的值
fn get_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
}

// 遍历哈希 map
fn traversal_map() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

// 哈希 map 和所有权
fn map_owner() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不在有效
}

// 更新哈希 map
fn update_map() {
    // 覆盖一个值
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // 只在键没有对应值时插入键值对
    // 使用 entry 方法只在建没有对应一个值时插入
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // 根据旧值更新一个值
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map)
}
