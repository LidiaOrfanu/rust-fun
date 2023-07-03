extern crate fizzbuzz;
use colored::Colorize;
use std::io;
use fizzbuzz::fizzbuzzfolder::fizzbuzzfile;

// pub fn fizzbuzz(num: u8) {
//     for i in 1..=num {
//         match (i % 3 == 0, i % 5 == 0) {
//             (true, true) => println!("FizzBuzz"),
//             (true, false) => println!("Fizz"),
//             (false, true) => println!("Buzz"),
//             (false, false) => println!("{i}"),
//         };
//     }
// }

fn main() {
    println!("{}", "Please provide a number: 0 - 255".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u8 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", "Only positive numbers: 0 - 255!".red());
            return;
        }
    };
    fizzbuzzfile::fizzbuzz(input);
}
