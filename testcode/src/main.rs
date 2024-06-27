#[derive(Debug)]
struct Retangle {
    width: i32,
    height:i32,
}

fn main() {
    let r = Retangle{
        width: 100,
        height: 100
    };
    println!("{r:?}");
    dbg!(r);
    // println!("{r:?}");
}
