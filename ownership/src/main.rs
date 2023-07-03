fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

// fn greet(g1: &String, g2: &String) {
//     println!("{} {}!", g1, g2);
// }

fn greet(g1: String, g2: String) -> (String, String) {
    println!("{} {}!", g1, g2);
    (g1, g2)
}

fn main() {
    // let a = [0; 1_000_000];
    // let b = a;
    // println!("{}", b[0]);

    //copies the pointer from c into d, but the pointed-to data is not copied
    // c own the box, then ownership is moved from c to d and when the scope ends:
    // Rust deallocates the box only once on behalf of b. not a
    let c = Box::new([0; 1_000_000]);
    let d = c;
    println!("{}", d[1]);

    let first = String::from("Ferris");
    let first_clone = first.clone();
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
    let m1 = String::from("Hi");
    let m2 = String::from("there");
    let (m1_again, m2_again) = greet(m1, m2);
    // greet(&m1, &m2);
    // let s = format!("{} {}", m1, m2);
    let s = format!("{} {}", m1_again, m2_again);
    println!("{s}");

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); //explicit dereference
    let x_abs2 = x.abs(); //implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let s = String::from("Rust");
    let s_len1 = str::len(&s); //explicit reference
    let s_len2 = s.len();  //implicit dereference
    assert_eq!(s_len1, s_len2);

    let mut vec: Vec<i32> = vec![1, 2, 3];
    vec.push(4);
    for i in 0..vec.len(){
        println!("{}", vec[i]);
    }
}
