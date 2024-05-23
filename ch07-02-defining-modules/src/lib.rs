mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use create::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

