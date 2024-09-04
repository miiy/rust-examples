#[allow(unused_variables)]
fn main() {
    // 创建一个迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // 在一个 for 循环中使用迭代器
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }
}
