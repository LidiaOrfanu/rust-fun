fn fizzbuzz(num: u8) {
    for i in 1..=num {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
fn main() {
    fizzbuzz(20);
}
