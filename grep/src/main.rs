use std::env;
use std::fs;
use std::process;

struct Config{
    query: String,
    filename: String,
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("error args")
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok( Config{
            query,
            filename,
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("error happen: {}", err);
        process::exit(1);
    });

    println!("{}- {}", config.query, config.filename);
    let contents = fs::read_to_string("./src/main.rs").unwrap();
    println!("contents: {}", contents);

}
