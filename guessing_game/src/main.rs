use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {

    let mut rng = thread_rng();

    loop{
        let random_data: u32 = rng.gen_range(1..101);

        println!("{}", random_data);


        println!("Guess the number!");
        println!("Please input your guess number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&random_data){
            Ordering::Greater => println!("to big"),
            Ordering::Less => println!("too less"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
}
