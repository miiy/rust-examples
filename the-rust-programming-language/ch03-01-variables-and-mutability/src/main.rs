// 变量和可变性
fn main() {
    immutable();

    mutable();

    shadowing();
    shadowing2();
    shadowing3();
}

fn immutable() {
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
}

fn mutable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

// 常量
#[allow(dead_code)]
// 常量必须注明值的类型
// const THREE_HOURS_IN_SECONDS = 60 * 60 * 3;
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// 隐藏
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn shadowing2() {
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}

// 不能改变变量的类型
fn shadowing3() {
    // let mut spaces = "    ";
    // spaces = spaces.len();
}