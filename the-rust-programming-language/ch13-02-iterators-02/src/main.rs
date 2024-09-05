// 产生其他迭代器的方法
fn main() {
    // 调用迭代器适配器 map 来创建一个新的迭代器
    create_iter1();

    // 调用 map ⽅法创建⼀个新迭代器，接着调⽤ collect ⽅法消费新迭代器并创建⼀个 vector
    create_iter2();
}

// 调用迭代器适配器 map 来创建一个新的迭代器
fn create_iter1() {
    let v1: Vec<i32> = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1);
    let _ = v1.iter().map(|x| x + 1);
}

// 调用 map ⽅法创建⼀个新迭代器，接着调⽤ collect ⽅法消费新迭代器并创建⼀个 vector
fn create_iter2() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}