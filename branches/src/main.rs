fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if number {
    //     println!("number was three")
    // }

    if number != 0 {
        println!("number was something other than zero");
    }

    if_else();

    let_if();

    // loop_statement();

    loop_result();

    loop_label();

    while_statement();

    for_statement();

    for_in();

    for_in_rev();
}

fn if_else() {
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

fn let_if() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}")
}

// fn let_if_error() {
//     let condition = true;
//     let number = if condition { 5 } else { "six" };
//     println!("The value of number is: {number}")
// }

fn loop_statement() {
    loop {
        println!("again!");
    }
}

fn loop_result() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
            // 使用 break 关键字返回值 counter * 2。循环之后，我们通过分号结束赋值给 result 的语句。
        }
    };

    println!("The result is {result}");
}

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

fn while_statement() {
    let mut number = 3;
    
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
}

fn for_statement() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}

fn for_in() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}")
    }
}

fn for_in_rev() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}