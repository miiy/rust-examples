use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

// // Result 枚举由两个成员 Ok 和 Err
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

fn main() {
    match_example();
    match_example2();
    unwrap_or_else_example();
    unwrap_example();
    except_example();
    read_username_from_file();
    read_username_from_file2();
    read_username_from_file3();
    read_username_from_file4();
}

// 使用 match 表达式处理可能会返回的 Result 成员
fn match_example() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// 使用不同的方式处理不同类型的错误
fn match_example2() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn unwrap_or_else_example() {
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

// 如果 Result 值是成员 Ok 返回 Ok 中的值
// 如果 Result 值是成员 Err, unwrap 会调用 panic!
fn unwrap_example() {
    let greeting_file = File::open("hello.txt").unwrap();
}

// expect 类似于 unwrap, 允许我们选择 panic! 的错误信息
fn except_example() {
    let greeting_file = File::open("hello.txt")
    .expect("hello.txt should be included in thid project");
}

// 传播错误
// 一个函数使用 match 将错误返回给代码调用者
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 传播错误的简写 :? 运算符
// 一个使用 ? 运算符向调用者返回错误的函数
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// ? 运算符之后的链式方法调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// 使用 fs::read_to_string 而不是打开后读取文件
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}