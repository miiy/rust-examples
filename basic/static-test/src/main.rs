use std::ptr::addr_of;
fn main() {
    // https://rustwiki.org/zh-CN/std/keyword.static.html
    // static 项与 const 非常相似：两者都包含一个值，都需要类型注解，并且都只能使用常量函数和值进行初始化。
    // static 与众不同之处在于它们表示内存中的一个位置。 这意味着您可以引用 static 项，甚至可以对其进行修改，从而使它们本质上是变量。
    static STR1: &str = "abc";
    // static STR1: &'static str = "abc";
    println!("str1: {}", STR1);

    static mut STR2: &str = "abc";
    // 访问和修改 static mut 变量需要在 unsafe 块中进行。
    // println!("str2: {}", STR2);
    unsafe {
        println!("str2: {}", STR2);
        // println!("str2: {:p}", &STR2);
        println!("str2: {:p}", addr_of!(STR2));
        STR2 = "def";
        println!("str2: {}", STR2);
        println!("str2: {:p}", addr_of!(STR2));
    }
}

