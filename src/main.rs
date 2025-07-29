fn main() {
    let s1: String = String::from("Rust");
    print_string(s1);
    // println!("s1 is: {s1}");// error: borrow of moved value: `s1`
    
    let s2: String = String::from("Rust");
    let r1: &String = &s2;
    print_string_2(r1);
    println!("s2 is: {s2}");


    let s3: String = String::from("Leo");
    add_to_string(s3);
    // println!("s3 is: {s3}");// error: borrow of moved value: `s3`
    
} 

fn print_string(p1: String) {
    println!("p1 is: {p1}");
}

fn print_string_2(p1: &String) {
    println!("p1 is: {p1}");
}


fn add_to_string(mut p1: String) -> String {
    p1.push_str("is awesome!!");
    p1
}