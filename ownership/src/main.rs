fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s = String::from("hello");

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_thing = String::from("yours");
    some_thing
}
