// 结构体定义和实例化
#![allow(unused_variables)]

// User 结构体定义
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 创建 user 结构体实例
    create_user();

    // 改变 User 实例 email 字段的值
    update_user();

    // 结构体更新
    struct_update();
    struct_update2();

    // 元组结构体
    tuple_structs();

    // 类单元结构体
    unit_like_structs();

}

// 创建 user 结构体实例
fn create_user() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 使用结构体
    println!("{}", user1.email);

    let user1 = bind_user(String::from("someone@example.com"), String::from("someusername123"));
    println!("{}", user1.email);

}

// 改变 User 实例 email 字段的值
fn update_user() {
    let mut user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);
}

// bind_user 函数获取 email 和用户名并返回 User 实例
fn bind_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
    //
    // 使用字段初始化简写语法
    // 参数名与字段名都完全相同，可以使用字段初始化简写语法 (field init shorthand)
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 使用 结构体更新语法 (struct update syntax) 从其他示例创建实例

// 使用 user1 中的一个值创建一个新的 User 实例
fn struct_update() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user2.username);
    // user1.username 中的 String 移动到了user2上
    // println!("{}", user1.username);

}

// 使用结构体更新语法为一个 User 实例设置一个新的 email 值，不过其余值来自 user1 变量中实例的字段
fn struct_update2() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // 使用结构体更新语法
    // ..user1 必须放在最后
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // println!("{}", user1.username);

    // 结构体更新语法就像带有 = 的赋值，因为它移动了数据
    // 在创建 user2 后不能就再使用 user1 了
}

// 使用没有命名字段的元组结构体来创建不同的类型
// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, {}, {}", black.0, black.1, black.2);
    println!("origin: {}, {}, {}", origin.0, origin.1, origin.2);
}

// 没有任何字段的类单元结构体
// 类单元结构体
struct AlwaysEqual;

fn unit_like_structs () {
    let subject = AlwaysEqual;
}