fn main() {
    let var = "four";
    if var.len() == 4 {
        println!("Hello, world!");
    }
    let var2 = String::from("four");
    if var2.chars().nth(1).expect("I expect an error") == 'o' {
        println!("Hello {}", var2);
    }
    let mut num = 0;
    loop {
        num += 1;
        println!("test");
        if num == 10 {
            break;
        }
    }
    let pi: f64 = 3.141592;
    println!("Hello, {0} is {1:.3}", "pi", pi);
    println!("{}'{pi:.*}' has 2 fractional digits", "Hello", 2,pi= 3.124);
    println!("{}'{pi:.*}' has 3 fractional digits", "Hello", 3,pi= 3.124);
    println!("{}'{pi:.*}' has 3 characters", "Hello", 3,pi= "3.124");
    // assert_eq!(format!("Hello {{}}"), "Hello{}");
    assert_eq!(format!("{{ Hello"), "{ Hello");

}
