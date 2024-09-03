// 泛型数据类型
#![allow(unused_variables)]

fn main() {
    // 在函数定义中使用泛型

    // 两个函数，不同点只是名称和签名类型
    function_define();

    // 一个使用泛型参数的 largest 函数定义
    function_generics();

    // 结构体中定义泛型
    struct_generics();

    // 使用两个泛型的 Point，这样 x 和 y 可能是不同类型
    struct_generics2();

    // 方法定义中的泛型
    method_generics();

    // 方法中使用了与结构体定义中不同类型的泛型
    method_generics2();

}

// 在函数定义中使用泛型

// 两个函数，不同点只是名称和签名类型
fn function_define() {
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

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}

// 一个使用泛型参数的 largest 函数定义，尚不能编译
fn function_generics() {
    // fn largest<T>(list: &[T]) -> &T {
    //     let mut largest = &list[0];

    //     for item in list {
    //         if item > largest {
    //             largest = item;
    //         }
    //     }

    //     largest
    // }
    fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
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

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 结构体中定义泛型
// Point 结构体存放了两个 T 类型的值 x 和 y
#[allow(dead_code)]
fn struct_generics() {
    struct Point<T> {
        x: T,
        y: T,
    }

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
}

// 使用两个泛型的 Point，这样 x 和 y 可能是不同类型
#[allow(dead_code)]
fn struct_generics2() {
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let both_integer = Point {x: 5, y: 10};
    let both_float = Point {x: 1.0, y: 4.0};
    let integer_and_float = Point {x: 5, y: 4.0};
}


// 枚举定义中的泛型

// Option<T> 枚举定义
// enum Option<T> {
//     Some(T),
//     None,
// }
//
// Result 枚举定义
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// 方法定义中的泛型
// 在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
fn method_generics() {
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y)
}


// 方法中使用了与结构体定义中不同类型的泛型
fn method_generics2() {
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point {x: 5, y: 10.4};
    let p2 = Point {x: "Hello", y: "c"};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
