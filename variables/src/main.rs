use colored::Colorize;

fn convert_fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    let celsius = (fahrenheit - 32) * 5 / 9;
    celsius
}

fn fibonacci(mut num: i32) {
    if num == 0 {
        println!("{num}");
        return;
    } else if num == 1 {
        println!("{num}");
        return;
    }
    let mut n1: i32 = 0;
    let mut n2: i32 = 1;
    let mut next: i32 = n1 + n2;
    println!("{n1}");
    println!("{n2}");
    println!("{next}");
    num -= 3;
    while num != 0 {
        n1 = n2;
        n2 = next;
        next = n1 + n2;
        println!("{next}");
        num -= 1;
    }
}

fn print_the_twelve_days_of_christmas() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for i in 0..days.len() {
        println!("On the {} of Christmas", days[i]);
        println!("My true love sent to me");
        for j in (0..=i).rev() {
            if j == 0 && i != 0 {
                let words = gifts[j].split_whitespace();
                let mut first_word = true;
                for word in words {
                    if first_word {
                        print!("And {}", word.to_lowercase());
                        first_word = false;
                    }
                    else {
                        print!(" {word}");
                    }
                }
                println!();
            } else {
                println!("{}", gifts[j]);
            }
        }
    }
}

fn main() {
    let t = ([1; 2], [3; 4]);
    let (a, _) = t;
    println!("{}", a[0]);
    println!("{}", t.1[0]);
    println!("{}", a[0] + t.1[0]);
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
    println!("____________________");
    println!(
        "Temperature converted is: {}",
        convert_fahrenheit_to_celsius(-13).to_string().red()
    );
    println!("Fibb sequence is: ");
    fibonacci(0);
    print_the_twelve_days_of_christmas();
}
