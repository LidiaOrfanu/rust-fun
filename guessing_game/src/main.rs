use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::Colorize;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=3);
    let mut guessed = false;
    println!("{}", "Guess the number (between 1-5)!".yellow());
    
    while !guessed {
        println!("{}", "Please input your guess:".yellow());

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        println!("Your guess was: {}", guess.yellow());
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Only positive numbers accepted!".red());
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small".red()),
            Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Equal => {
                println!("{}","Congrats! You guess the number!".green());
                guessed = true;
            }
        }
    }
}
