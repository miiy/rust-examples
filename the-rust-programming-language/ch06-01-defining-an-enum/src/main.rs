// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

// 定义函数
fn route(ip_kind: IpAddrKind) {}

// 定义结构体
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 定义枚举
enum IpAddr2 {
    V4(String),
    V6(String),
}

enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Movie { x: i32, y: i32 }, // 类似结构体包含命名字段
    Write(String),
    ChangeColor(i32, i32, i32)
}

fn main() {
    // 枚举值
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // 使用成员调用函数
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // 结构体
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 将数据放入枚举成员
    let home = IpAddr2::V4(String::from("127.0.0.1"));

    let loopback = IpAddr2::V6(String::from("::1"));

    //
    let home = IpAddr3::V4(127, 0, 0, 1);
    let loopback = IpAddr3::V6(String::from("::1"));

    // Option 枚举
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

}





