fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // 使用元组重构
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // 使用结构体重构
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    // 通过 派生 trait 增加实用功能
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // println!("rect1 is {}", rect1);
    // println!("rect1 is {:?}", rect1);
    // 在结构体定义之前加上外部属性来派生 Debug trait
    // #[derive(Debug)]
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // dbg! 返回表达式的值的所有权
    dbg!(&rect1);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// 使用元组重构
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 使用结构体重构
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}