// 结构体示例程序
#![allow(unused_variables)]

fn main() {
    // 通过分别指定长方形的宽和高的变量来计算长方形面积
    rectangle_area();

    // 使用元组重构
    rectangle_area_tuple();

    // 使用结构体重构
    rectangle_area_struct();

    // 尝试打印出 Rectangle 实例
    print_instance();

    // 通过派生 trait 增加实用功能
    debug_trait();
}

// 通过分别指定长方形的宽和高的变量来计算长方形面积
fn rectangle_area() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 使用元组重构
// 使用元组来制定长方形的宽高
fn rectangle_area_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 使用结构体重构：赋予更多意义
// 定义 Rectangle 结构体
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_area_struct() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// 尝试打印出 Rectangle 实例
fn print_instance() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("rect1 is {}", rect1);

}

// 增加属性来派生 Debug trait，并使用调试格式打印 Rectangle 实例
// 在结构体定义之前加上外部属性来派生 Debug trait
// #[derive(Debug)]
#[derive(Debug)]
struct Rectangle1 {
    width: u32,
    height: u32,
}

fn debug_trait() {
    let rect1 = Rectangle1 {
        width: 30,
        height: 50,
    };
    println!("rect1: {}, {}", rect1.width, rect1.height);

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // dbg! 返回表达式的值的所有权
    dbg!(&rect1);
}