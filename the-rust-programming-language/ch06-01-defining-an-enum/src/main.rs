// 枚举的定义
#![allow(unused_variables)]
#![allow(dead_code)]



fn main() {
    // 枚举值
    enum_value();

    //
    ip_struct();

    //
    enum_value2();

    //
    enum_value3();

    // Option 枚举
    option_enum();
}



// 枚举值
fn enum_value() {
    // 定义枚举
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 定义函数
    fn route(ip_kind: IpAddrKind) {}

    // 使用成员调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

// 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
fn ip_struct() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

fn enum_value2() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    // 将数据放入枚举成员
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}

fn enum_value3() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// 一个 Message 枚举，其每个成员都存储了不同数量和类型的值
enum Message {
    Quit,
    Movie { x: i32, y: i32 }, // 类似结构体包含命名字段
    Write(String),
    ChangeColor(i32, i32, i32)
}

// Option 枚举和其相对于空值的优势
fn option_enum() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}