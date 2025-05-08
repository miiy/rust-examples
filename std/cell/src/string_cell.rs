// use std::cell::Cell;
// use std::string::String;

// struct StringCell {
//     value: Cell<String>,
// }

// impl StringCell {
//     fn new() -> Self {
//         StringCell {
//             value: Cell::new(String::new()),
//         }
//     }

//     fn set(&self, value: String) {
//         self.value.set(value);
//     }

//     fn get(&self) -> String {
//         self.value.get()
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_string_cell() {
//         let sc = StringCell::new();
//         assert_eq!(sc.get(), "");
//         sc.set("Hello".to_string());
//         assert_eq!(sc.get(), "Hello");
//         sc.set("Hello, World!".to_string());
//         assert_eq!(sc.get(), "Hello, World!");
//     }
// }