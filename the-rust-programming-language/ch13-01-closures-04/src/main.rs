#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用 sort_by_key 对长方形按宽度排序
fn main() {
    sort_by_key1();
    // sort_by_key2();
    sort_by_key3();
}

fn sort_by_key1() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");
}

/*
fn sort_by_key2() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        num_sort_operations.push(value);
        r.width
    });
    println!("{list:#?}");
}
*/

fn sort_by_key3() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}