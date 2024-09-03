#![allow(unused_variables)]

fn main() {
    // 新建 vector
    create_vector();

    // 更新 vector
    update_vector();

    // 读取 vector 的元素
    get_vector();

    // 遍历 vector 中的元素
    traversal_vector();

    // 使用枚举来储存多种类型
    enum_vector();

    // 丢弃 vector 时也会丢弃其所有元素
    drop_vector();
}

// 新建 vector
fn create_vector() {
    // 新建一个空的 vector 来存储 i32 类型的值
    let v: Vec<i32> = Vec::new();

    // 新建一个包含值的 vector
    let v = vec![1, 2, 3];
}

// 更新 vector
fn update_vector() {
    // 使用 push 方法向 vector 增加值
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

// 读取 vector 的元素
#[allow(unused_mut)]
fn get_vector() {
    // 使用索引语法或 get 方法来访问 vector 中的项
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third elemtnt.")
    }

    // 尝试访问一个包含 5 个元素的 vector 的索引 100 处的元素
    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // 在拥有 vector 中项的引用的同时向其增加一个元素
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    // v.push(6); // cannot borrow `v` as mutable because it is also borrowed as immutable mutable borrow occurs here

    println!("The first element is: {first}");
}

// 遍历 vector 中的元素
fn traversal_vector() {
    // 通过 for 循环遍历 vector 的元素并打印
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    // 遍历 vector 中元素的可变引用
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50
    }
}

// 使用枚举来储存多种类型
fn enum_vector() {
    // 定义一个枚举，以便在vector中存放不同类型的数据
    enum SpareadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpareadsheetCell::Int(3),
        SpareadsheetCell::Text(String::from("blue")),
        SpareadsheetCell::Float(10.12),
    ];

    // 读取
    for cell in &row {
        match cell {
            SpareadsheetCell::Int(i) => println!("Integer: {}", i),
            SpareadsheetCell::Float(f) => println!("Float: {}", f),
            SpareadsheetCell::Text(s) => println!("Text: {}", s),
        }
    }
}

// 丢弃 vector 时也会丢弃其所有元素
fn drop_vector() {
    //展示 vector 和其元素于何时被丢弃
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v 离开作用域在这里被释放
}
