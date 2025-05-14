#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

fn test1() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    let Person { name, age } = person;
    println!("Name: {}, Age: {}", name, age);
    // The following line will cause a compile-time error because `person` has been moved
    // println!("Person: {:?}", person);
}

fn test2() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let Person { ref name, age } = person;
    println!("Name: {}, Age: {}", name, age);
    println!("Person: {:?}", person);
}


fn test3() {
    let mut person = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // let Person { ref mut name, ref mut age } = &mut person;
    let Person { ref mut name, ref mut age } = person;
    println!("Name: {}, Age: {}", name, age);
    
    // println!("Person: {:?}", person);
    
    *name = String::from("Bob");
    *age = 31;
    
    println!("Person: {:?}", person);
}

fn main() {
    println!("");
    test1();
    println!("");
    test2();
    println!("");
    test3();
}
