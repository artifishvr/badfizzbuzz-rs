use std::env;
mod fizz_buzz;

fn main() {
    let args: Vec<String> = env::args().collect();

    let Some(query) = args.get(1) else {
        println!("Please provide a valid i32");

        return;
    };

    let Ok(query) = query.parse::<i32>() else {
        println!("Argument isn't a valid i32");

        return;
    };

    for i in 0..query {
        let fizzbuzz: &str = fizz_buzz::fizzbuzz(i);

        println!("{}", fizzbuzz);
    }
}
