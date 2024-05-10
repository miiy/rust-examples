// 结构体定义
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 创建 user 结构体实例
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

    // 创建可变结构体
    let mut user1 = User{
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    // 改变 user 实例 email 字段的值
    user1.email = String::from("anotheremail@example.com");
    println!("{}", user1.email);


    // 结构体更新语法 (struct update syntax)
    // 不适用结构体更新语法
    let user2 = User{
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("{}", user2.username);
    // user1.username 中的 String 移动到了user2上
    // println!("{}", user1.username);

    // 使用结构体更新语法
    // ..user1 必须放在最后
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };


}

// 返回一个带有给定的 email 和用户名的 User 结构体实例
fn bind_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
    // 参数名与字段名都完全相同，可以使用字段初始化简写语法 (field init shorthand)
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// 类单元结构体
struct AlwaysEqual;

fn unit_like_structs () {
    let subject = AlwaysEqual;
}