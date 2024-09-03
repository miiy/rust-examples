// 泛型、Trait 和生命周期
use std::vec;

fn main() {
    // 提取函数来减少重复

    // 在一个数字列表中寻找最大值的函数
    find_largest1();
    // 寻找 两个 数字列表最大值的代码
    find_largest2();
    // 抽象后的寻找两个数字列表最大值的代码
    find_largest3();
}

// 提取函数来减少重复

// 在一个数字列表中寻找最大值的函数
fn find_largest1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

// 寻找 两个 数字列表最大值的代码
fn find_largest2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];
    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

}

// 抽象后的寻找两个数字列表最大值的代码
fn find_largest3() {

    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
