// 不安全 Rust
#![allow(unused_variables)]

use std::slice;

fn main() {
    // 通过引用创建裸指针
    example();

    // 创建指向任意内存地址的裸指针
    example2();

    // 在 unsafe 块中解引用裸指针
    example3();

    // 使用安全的 split_at_mut 函数
    example4();

    // 通过任意内存地址创建 slice
    create_slice();

    // 声明并调用另一个语言中定义的 extern 函数
    extern_example();

    // 定义和使用一个不可变静态变量
    static_variable();

    // 读取或修改一个可变静态变量是不安全的
    mut_static();
}

// 不安全的超能力

// 解引用裸指针

// 通过引用创建裸指针
fn example() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}

// 创建指向任意内存地址的裸指针
fn example2() {
    let address = 0x012345usize;
    let r = address as *const i32;
}

// 在 unsafe 块中解引用裸指针
fn example3() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("t2 is: {}", *r2);
    }
}

// 调用不安全函数或方法
// unsafe fn dangerous() {}

// unsafe {
//     dangerous();
// }

// 创建不安全代码的安全抽象
// 使用安全的 split_at_mut 函数
fn example4() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// 尝试只使用安全 Rust 来实现 split_at_mut
// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (& mut values[..mid], &mut values[mid..])
// }

// 在 split_at_mut 函数的实现中使用不安全代码
#[allow(dead_code)]
fn split_at_mut(values: &mut[i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// 通过任意内存地址创建 slice
fn create_slice() {
    let address = 0x01234usize;
    let r = address as *mut i32;

    let value: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
}

// 使用 extern 函数调用外部代码
// 声明并调用另一个语言中定义的 extern 函数
fn extern_example() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 访问或修改可变静态变量

// 定义和使用一个不可变静态变量
static HELLO_WORLD: &str = "Hello, world!";

fn static_variable() {
    println!("name is: {HELLO_WORLD}");
}

// 读取或修改一个可变静态变量是不安全的
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mut_static() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}

// 实现不安全 trait

// 定义并实现不安全 trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

// 访问联合体中的字段

// 何时使用不安全代码
