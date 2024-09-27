// 数据类型
#![allow(unused_variables)]

fn main() {
    float_type();
    bool_type();
    char_type();

    numeric_operations();

    tuple_type();
    destruct_tuple();
    array_type();
}

// 标量类型

// 整型

// 浮点型
fn float_type() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

// 数值运算
fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 结果为 -1

    // remainder
    let remainder = 43 % 5;
}

// 布尔型
fn bool_type() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

// 字符型
fn char_type() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

// 复合类型

// 元组类型
fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}

// 使用模式匹配来解构元组值
fn destruct_tuple() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;
    println!("The value of y: {y}");
}

// 数组类型
fn array_type() {
    let a = [1, 2, 3, 4, 5];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    let a = [3; 5];
    println!("a: {:?}", a);
}