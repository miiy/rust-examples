// https://doc.rust-lang.org/std/boxed/index.html
// move 指的是所有权的转移，如果实现了 Copy trait 是复制，否则是移动
fn main() {
    // Move a value from the stack to the heap by creating a Box:
    // 通过创建 Box 将值从栈移动到堆
    {
        let val: u8 = 5;
        let boxed = Box::new(val);
        println!("{boxed}");
    }


    // Move a value from a Box back to the stack by dereferencing:
    // 通过解引用将值从 Box 移回栈
    {
        let boxed: Box<u8> = Box::new(5);
        let val: u8 = *boxed;
        println!("{val}");
    }

    // Creating a recursive data structure:
    // 创建递归数据结构：
    {
        #[allow(dead_code)]
        #[derive(Debug)]
        enum List<T> {
            Cons(T, Box<List<T>>),
            Nil,
        }
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        println!("{list:?}");
    }

    {
        let mut boxed: Box<u8> = Box::new(5);
        *boxed = 6;
        println!("{boxed}");
    }
}
