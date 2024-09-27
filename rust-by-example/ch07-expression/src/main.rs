fn main() {
    let x = 5u32;

    let y = {
        x * x
    };

    let z = {
        x * x;
    };
    println!("x is: {x}");
    println!("y is: {y}");
    println!("z is: {:?}", z);
}
