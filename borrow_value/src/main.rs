use std::io;

fn main() {
    let test_string = String::from("hellow");

    let (test_string_copy, copy_len) = test_copy(test_string);
    println!("test_string: {}", test_string);
}

fn test_copy(str: String) -> (String, usize) {
    (str, str.len())
}
