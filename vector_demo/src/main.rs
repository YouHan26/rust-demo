use std::collections::HashMap;
use std::io;

fn main() {
    // println!("please enter a list numbers : ");
    // let mut number = String::new();
    // io::stdin().read_line(&mut number)
    //     .expect("error reading numbers");
    // let number = number.split_whitespace()
    //     .map(|&x| x.parse::<u32>().unwrap());

    let origin_number = [1, 2, 1, 1, 5, 8, 6, 1, 10, 9];
    let mut numbers = origin_number.to_vec();
    numbers.sort();
    let len = numbers.len();
    let mut total = 0;


    let mut map = HashMap::new();
    for i in numbers {
        total += i;
        let number_count = map.entry(i).or_insert(0);
        * number_count += 1;
    }
    println!("average number: {}", total / len);

    let mut median_index = Vec::new();
    if len %2 == 0{
        median_index.push(len / 2 - 1);
        median_index.push(len / 2);
    } else {
        median_index.push(len / 2);
    }

    for index in median_index{
        println!("median number is : {}", &origin_number[index])
    }

    let mut count_max = 0;
    for (number, count) in &map{
        println!("{}-{}", number, count);
        if *count > count_max {
            count_max = *number;
        }
    }
    println!("appear number max: {}", count_max);
}
