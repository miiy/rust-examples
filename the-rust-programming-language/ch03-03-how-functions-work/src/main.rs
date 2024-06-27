fn main() {
    println!("Hello, world!");

    another_function();

    another_function_arg(5);

    print_labeled_measurement(5, 'h');

    expressions();

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");

}

fn another_function() {
    println!("Another function.");
}

fn another_function_arg(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, uint_label: char) {
    println!("The measurement is: {value}{uint_label}");
}

fn expressions() {
    let y = {
        let x = 3;
        x + 1 // 表达式的结尾没有分号，如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// fn plus_one_error(x: i32) -> i32 {
//     x + 1;
// }
//
// build:
// fn plus_one_error(x: i32) -> i32 {
//     |    --------------            ^^^ expected `i32`, found `()`
//     |    |
// 语句并不会返回值，使用单位类型 () 表示不返回值。因为不返回值与函数定义相矛盾，从而出现一个错误。