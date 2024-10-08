mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    #[allow(dead_code)]
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_peyment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}