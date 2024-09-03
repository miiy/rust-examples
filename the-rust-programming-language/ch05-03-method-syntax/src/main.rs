// 方法语法

fn main() {
    // 在 Rectangle 结构体上定义 area 方法
    area_method();

    // 带有更多参数的方法
    can_hold();

    // 关联函数
    let sq = Rectangle::square(3);
    println!("{}", sq.width);

    // 多个 impl 块
    let rect1 = Rectangle{
        width: 40,
        height: 50,
    };
    let area2 = rect1.area2();
    println!("{}", area2);

}

// 定义方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 带有更多参数的方法
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// 在 Rectangle 结构体上定义 area 方法
fn area_method() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// 带有更多参数的方法
fn can_hold() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

// 多个 impl 块
impl Rectangle {
    fn area2(&self) -> u32 {
        self.width * self.height
    }
}