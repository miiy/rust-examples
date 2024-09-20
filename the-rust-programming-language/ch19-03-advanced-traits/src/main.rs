// 高级 trait


fn main() {
    // 实现 Add trait 重载 Point 实例的 + 运算符
    add_trait();

    // 在 Millimeters 上实现 Add, 以便能够将 Millimeters 与 Meters 相加
    add_meters();

    trait_example();

    trait_example2();

    outline_print_example();

    newtype();
}

// 关联类型在 trait 定义中指定占位符类型

// Iterator trait 定义中带有关联类型 Item
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter{}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(0)
    }
}

// 一个泛型的 Iterator trait 假想定义
pub trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

// 默认泛型类型参数和运算符重载

// 实现 Add trait 重载 Point 实例的 + 运算符
fn add_trait() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point{
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point{x: 1, y: 0} + Point{x: 2, y: 3},
        Point{x: 3, y: 3}
    );
}

// 在 Millimeters 上实现 Add, 以便能够将 Millimeters 与 Meters 相加
fn add_meters() {
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
}

// 完全限定语法与消除歧义：调用相同名称的方法

// 两个 trait 定义为拥有 fly 方法，并在直接定义有 fly 方法的 Human 类型上实现这两个 trait
fn trait_example() {
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // 调用 Human 实例的 fly
    let person = Human;
    person.fly();

    // 指定我们希望调用哪一个 trait 的 fly 方法

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}

// 一个带有关联函数的 trait 和一个带有同名关联函数并实现了此 trait 的类型
fn trait_example2() {
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("Puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // 尝试调用 Animal trait 的 baby_name 函数，不过 Rust 并不知道该使用哪一个实现
    // println!("A baby dog is called a {}", Animal::baby_name());

    // 使用完全限定语法来指定我们希望调用的是 Dog 上的 Animal trait 实现中的 baby_name 函数
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

// 父 trait 用于在另一个 trait 中使用某 trait 的功能

// 实现 OutlinePrint trait, 它要求来自 Display 的功能
fn outline_print_example() {
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

// newtype 模式用以在外部类型上实现外部 trait

// 创建 Wrapper 类型封装 Vec<String> 以便能够实现 Display
fn newtype() {
    use std::fmt;

    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}