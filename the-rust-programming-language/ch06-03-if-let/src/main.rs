// if let 简洁控制流
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_assignments)]

fn main() {
    // match 只关心当值为 Some 时执行代码
    match_some();

    // if let
    if_let();

    // macth other
    match_other();

    // if let 和 else 表达式
    if_let_else();

}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// match 只关心当值为 Some 时执行代码
fn match_some() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {:?}", max),
        _ => (),
    }
}

// if let
fn if_let() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {:?}", config_max);
    }
}

// match
fn match_other() {
    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}

// if let 和 else 表达式
fn if_let_else() {
    let coin = Coin::Penny;
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}