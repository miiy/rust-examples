// match 控制流结构
#![allow(unused_variables)]

fn main() {
    match_example();

    // 绑定值的模式
    bind_value();

    // 匹配 Option<T>
    match_option_t();

    // 通配模式和 _ 占位符
    match_other();

    match_underline();

    match_underline2();
}

// 一个枚举和一个以枚举成员作为模式的 match 表达式
fn match_example() {

    #[allow(dead_code)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    value_in_cents(Coin::Penny);
}

// 绑定值的模式
#[allow(dead_code)]
fn bind_value() {
    // Quarter 成员也存放了一个 UsState 值的 Coin 枚举
    #[derive(Debug)] // 这样可以立刻看到州的名称
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

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    value_in_cents(Coin::Quarter(UsState::Alabama));

}

// 匹配 Option<T>

// 一个在 Option<i32> 上使用 match 表达式的函数
fn match_option_t() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

// 匹配 Some<T>

// 匹配是穷尽的
// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         Some(i) => Some(i + 1),
//     }
// }

// 通配模式和 _ 占位符
// 最后一个分支涵盖了所有其他可能的值，模式是我们命名为 other 的一个变量
fn match_other() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// 不需要使用这个值，用 _ 替代变量 other
fn match_underline() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

// 使用单元值
fn match_underline2() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}