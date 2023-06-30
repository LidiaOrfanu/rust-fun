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
}
