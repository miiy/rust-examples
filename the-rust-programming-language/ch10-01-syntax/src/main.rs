fn main() {
    // 两个函数，不同点只是名称和签名类型
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // 使用泛型参数
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // 结构体中定义泛型
    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    // 使用两个泛型
    let both_integer = Point2 {x: 5, y: 10};
    let both_float = Point2 {x: 1.0, y: 4.0};
    let integer_and_float = Point2 {x: 5, y: 4.0};

    // 方法定义中的泛型
    let p = Point3 {x: 5, y: 10};

    println!("p.x = {}", p.x());

    // 方法中使用了与结构体定义中不同类型的泛型
    let p1 = Point4 {x: 5, y: 10.4};
    let p2 = Point4 {x: "Hello", y: "c"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 结构体中定义泛型
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

// 枚举定义中的泛型
// enum Option<T> {
//     Some(T),
//     None,
// }

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法定义中的泛型
struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 方法中使用了与结构体定义中不同类型的泛型
struct Point4<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point4<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point4<X2, Y2>) -> Point4<X1, Y2> {
        Point4 {
            x: self.x,
            y: other.y,
        }
    }
}