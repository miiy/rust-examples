// https://doc.rust-lang.org/core/default/trait.Default.html

fn main() {
    example1();

    example2();

    example3();
}

fn example1() {
    #[allow(dead_code)]
    #[derive(Default, Debug)]
    struct SomeOptions {
        foo: i32,
        bar: f32,
    }

    let options: SomeOptions = Default::default();
    println!("{:?}", options);

    let options = SomeOptions{ foo: 42, ..Default::default()};
    println!("{:?}", options);
}

fn example2() {
    #[allow(dead_code)]
    #[derive(Debug)]
    enum Kind {
        A,
        B,
        C,
    }

    impl Default for Kind {
        fn default() -> Self {
            Kind::A
        }
    }

    let k: Kind = Default::default();
    println!("{:?}", k);
}

fn example3() {
    #[allow(dead_code)]
    #[derive(Default, Debug)]
    enum Kind {
        #[default]
        A,
        B,
        C,
    }

    let k: Kind = Default::default();
    println!("{:?}", k);
}