mod fizz_buzz;

fn main() {
    let n = 1000000;
    let mut fizzbuzz;
    for i in 1..=n {
        fizzbuzz = fizz_buzz::fizzbuzz(i);
        println!("{}", fizzbuzz);
    }
}
