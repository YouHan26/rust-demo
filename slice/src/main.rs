fn main() {
    let mut string = String::from("hello world");
    let word = first_word(&string);
    string.clear(); // 会失败，因为已经有不可变引用，不能直接修改
    println!("first word: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            // 不可变引用
            return &s[0..i];
        }
    }
    // 不可变引用
    &s[..]
}
