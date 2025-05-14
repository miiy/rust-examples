fn main() {
    // https://rustwiki.org/zh-CN/std/keyword.const.html

    // 常量必须显式地输入。与 let 不同，您不能忽略它们的类型并让编译器确定它的类型。
    // 类型推导的局限性：const 在编译时就必须知道它的类型。如果是整数可以是 i32, i64, u32, u64 等等。
    // 避免歧义：明确指明类型
    const STR1: &str = "abc";

    // 常量中唯一允许的生命周期是 'static
    // 多亏了静态生命周期省略，您通常不必显式使用 'static：
    // const STR1: &'static str = "abc";
    println!("str1: {}", STR1);

    // const 项看起来与 static 项非常相似
    // 常量在任何使用它们的地方都被内联，使用它们与简单地用它的值替换 const 的名称是相同的。
    // 静态变量指向内存中所有访问共享的单个位置。 
}
