
use std::cell::Cell;

mod counter;
mod counter_cell;
mod string_cell;

fn main() {
    interior_mutability();
}

// https://doc.rust-lang.org/std/cell/struct.Cell.html
// Cell 是一种提供内部可变性的类型，允许在不可变的上下文种修改数据。
// Cell 不提供直接的引用借用，通过 get 和 set 方法来访问和修改值，数据是通过值的复制来修改的。
// Cell 只能用于实现 Copy trait 的类型。
fn interior_mutability() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct SomeStruct {
        regular_field: i32,
        special_field: Cell<i32>,
    }

    let my_struct = SomeStruct {
        regular_field: 10,
        special_field: Cell::new(1),
    };
    println!("mystruct: {:?}", my_struct);

    let new_value = 100;

    // ERROR: `my_struct` is immutable
    // my_struct.regular_field = new_value;

    // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
    // which can always be mutated
    my_struct.special_field.set(new_value);
    assert_eq!(my_struct.special_field.get(), new_value);
    println!("mystruct: {:?}", my_struct);
}
