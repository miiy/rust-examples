fn main() {

    if_expression();
    println!();

    else_if_expression();
    println!();

    if_in_let_statement();
    println!();

    repeating_with_loop();
    println!();

    return_from_loop();
    println!();

    loop_label();
    println!();

    while_statement();
    println!();

    traverse_while();
    println!();

    traverse_for_in();
    println!();

    traverse_rev();
}

// if 表达式
fn if_expression() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 条件必须是布尔值
    // if number {
    //     println!("number was three");
    // }
}

// 使用 else if 处理多重条件
fn else_if_expression() {
    let number = 6;

    if number %4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

// 在 let 语句中使用 if
// 将if 表达式的返回值赋给一个变量
fn if_in_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // 变量必须只有一个类型
    // let condition = true;
    // let number = if condition { 5 } else { "six" };
    // println!("The value of number is: {number}");
}


// 使用循环
fn repeating_with_loop() {
    // loop {
    //     println!("again!");
    // }
}

// 从循环返回值
// 使用 break 关键字返回值 counter * 2。循环之后，我们通过分号结束赋值给 result 的语句。
fn return_from_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// 标签循环：在多个循环之间消除歧义
fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// while 条件循环
fn while_statement() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// 使用 for 遍历集合

// 使用 while 循环遍历集合中的元素
fn traverse_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

// 使用 for 循环遍历集合中的元素
fn traverse_for_in() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }
}

// rev 反转 range
fn traverse_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}