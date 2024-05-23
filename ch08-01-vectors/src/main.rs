fn main() {
    // 新建 vector

    // 使用 Vec::new 创建 vector
    let v: Vec<i32> = Vec::new();
    // 用初始值创建一个 Vec<T>，Rust 会推断出储存值的类型
    let v = vec![1, 2, 3];

    // 更新 vector

    // 使用 push 方法增加元素
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 读取 vector
    // 使用索引语法或get方法来访问 vector 中的项
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third elemtnt.")
    }

    range_vector();

    enum_vector();

    // vector 在离开作用域时会被释放
    {
        let v = vec![1, 2, 3, 4];
    } // v 离开作用域被释放
}

fn range_vector() {
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
}
