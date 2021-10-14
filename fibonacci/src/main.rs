use std::io;

fn main() {
    let test_string = String::from("hellow");

    let (test_string_copy, copy_len) = test_copy(test_string);
    println!("test_string: {}", test_string);

    println!("please enter fibonacci number");

    let mut number = String::new();

    io::stdin().read_line(&mut number)
        .expect("failed to read fibonacci number");

    let number: u64 = number.trim().parse()
        .expect("failed to parse fibonacci number");

    println!("fibonacci number: {}", fibonacci(number));
}

fn fibonacci(number: u64) -> u64 {
    if number == 0 {
        0
    } else if number == 1{
        1
    } else{
        number + fibonacci(number - 1)
    }
}

fn test_copy(str: String) -> (String, usize) {
    (str, str.len())
}
