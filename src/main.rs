use std::env;
mod fizz_buzz;

fn main() {
    let args: Vec<String> = env::args().collect();
    let query = &args[1] as *const String as i32;

    fizz_buzz::fizzbuzz(query);
}
