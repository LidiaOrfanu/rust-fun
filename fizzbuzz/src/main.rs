extern crate fizzbuzz;
use colored::Colorize;
use std::io;
use fizzbuzz::fizzbuzzfolder::fizzbuzzfile;

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
