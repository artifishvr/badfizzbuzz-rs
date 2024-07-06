use std::env;
mod fizz_buzz;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = args[1].parse::<i32>().expect("Not a valid i32");

    let fizzbuzz: &str = fizz_buzz::fizzbuzz(query);
    println!("{}", fizzbuzz.to_string());
}
