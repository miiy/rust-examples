// 一个 adder crate 中函数的集成测试
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
