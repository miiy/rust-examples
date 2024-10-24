fn main() {
    from_str();
    from_example();
    into_example();
}

fn from_str() {
    let my_str = "hello";
    let my_string = String::from(my_str);
    println!("{my_string}");
}

// 为我们自己的类型定义转换机制：
#[allow(dead_code)]
fn from_example() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let num = Number::from(30);
    println!("My number is {:?}", num)
}

// 如果你为你的类型实现了 From，那么同时你也获得了 Into
// 使用 Into trait 通常要求指明要转换到的类型
#[allow(dead_code)]
fn into_example() {
    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
