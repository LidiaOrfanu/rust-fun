
pub fn fizzbuzz(num: u8) -> Vec<String>{
    if num < 1 {
        panic!("Number value must be between 1 and 100, got {}.", num)
    }
    if num > 100 {
        panic!("Number value must be between 1 and 100, got {}", num)
    }
    
    let mut result = Vec::new();
    for i in 1..=num {
        let x = match (i % 3 == 0, i % 5 == 0) {
            (true, true) => "FizzBuzz".to_string(),
            (true, false) => "Fizz".to_string(),
            (false, true) => "Buzz".to_string(),
            (false, false) => i.to_string(),
        };
        println!("{}", x);
        result.push(x);
    }
    result
}
