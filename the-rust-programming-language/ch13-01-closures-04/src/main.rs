// 将被捕获的值移出闭包和 Fn trait
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // 使用 sort_by_key 对长方形按宽度排序
    sort_by_key1();

    // 尝试在 sort_by_key 上使用一个 FnOnce 闭包
    // sort_by_key2();

    // 允许在 sort_by_key 上使用一个 FnMut 闭包
    sort_by_key3();
}

// 使用 sort_by_key 对长方形按宽度排序
fn sort_by_key1() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    println!("{}", list[0].height);
}

// 尝试在 sort_by_key 上使用一个 FnOnce 闭包
// fn sort_by_key2() {
//     let mut list = [
//         Rectangle { width: 10, height: 1 },
//         Rectangle { width: 3, height: 5 },
//         Rectangle { width: 7, height: 12 },
//     ];

//     let mut num_sort_operations = vec![];
//     let value = String::from("closure called");

//     list.sort_by_key(|r| {
//         num_sort_operations.push(value);
//         r.width
//     });
//     println!("{list:#?}");
// }

// 允许在 sort_by_key 上使用一个 FnMut 闭包
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