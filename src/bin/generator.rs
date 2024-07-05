use std::fs::File;
use std::io::prelude::*;

fn main() {
    let n = 1000000;
    let filename = "./src/fizz_buzz.rs";

    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(b"pub fn fizzbuzz(num: i32) -> &'static str {\n")
        .expect("Unable to write to file");

    for i in 0..=n {
        let fizzbuzz = if i % 3 == 0 && i % 5 == 0 {
            "FizzBuzz"
        } else if i % 3 == 0 {
            "Fizz"
        } else if i % 5 == 0 {
            "Buzz"
        } else {
            ""
        };

        let line = if fizzbuzz.is_empty() {
            format!("\tif num == {} {{ return \"{}\"; }}\n", i, i)
        } else {
            format!("\tif num == {} {{ return \"{}\"; }}\n", i, fizzbuzz)
        };

        file.write_all(line.as_bytes())
            .expect("Unable to write to file");
    }

    file.write_all(b"\telse { return \"\"; }\n")
        .expect("Unable to write to file");

    file.write_all(b"}\n").expect("Unable to write to file");

    println!("File has been created successfully. Enjoy!");
}
