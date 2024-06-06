use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args.clone());

    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

// cargo run
// cargo run -- needle haystack
// cargo run -- test sample.txt