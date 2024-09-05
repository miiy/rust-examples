// 使用迭代器处理元素序列

fn main() {
    // 创建一个迭代器
    create_iter();

    // 在一个 for 循环中使用迭代器
    loop_iter();

    // Iterator trait 和 next 方法
    // src/lib.rs
}

// 创建一个迭代器
#[allow(unused_variables)]
fn create_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
}

// 在一个 for 循环中使用迭代器
fn loop_iter() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}