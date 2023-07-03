
pub fn fizzbuzz(num: u8) {
    for i in 1..=num {
        match (i % 3 == 0, i % 5 == 0) {
            (true, true) => println!("FizzBuzz"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (false, false) => println!("{i}"),
        };
    }
}

pub fn add_one_test(num: u8) -> u8 {
    num + 1
}
