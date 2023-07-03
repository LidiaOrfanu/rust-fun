extern crate phrases;

// use phrases::english::greetings;
// use phrases::english::farewells;
use phrases::english::{greetings, farewells};
use phrases::norwegian;

fn main() {
    println!("Hello in English: {}", greetings::hello());
    println!("Goodbye in English: {}", farewells::goodbye());
    println!("Hello in Norwegian is: {}", norwegian::greetings::hello());
    println!("Goodbye in Norwegian is: {}", norwegian::farewells::goodbye());
}