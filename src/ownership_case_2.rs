fn main() {
    let s1: String = String::from("Rust"); // heap allocated string
    println!("s1 is: {s1}"); // s1 is: Rust
    print_string(s1);// p1 is: Rust
    // println!("s1 is: {s1}"); //error: value borrowed here after move

    let s2: String = String::from("Rust"); // heap allocated string
    println!("s2 is: {s2}"); // s2 is: Rust
    print_string(s2.clone()); // p1 is: Rust
    println!("s2 is: {s2}"); // s2 is: Rust
    
    let s3 = generate_string();
    println!("s3 is: {s3}"); // s3 is: Leo
    
    let s4 = String::from("Leo");
    println!("s4 is: {s4}"); // s4 is: Leo
    let s5 = add_to_string(s4);
    // println!("s4 is: {s4}"); // error: value borrowed here after move
    println!("s5 is: {s5}");//s5 is: Leois awesome!!


    let x: i32 = 10;
    print_integer(x);// i is 10
    println!("x is: {x}");// x is: 10

} 

fn print_integer(i:i32) {
    println!("i is {i}");
}

fn print_string(p1: String) {
    println!("p1 is: {p1}");
} // p1 is droped

fn generate_string() -> String {
    String::from("Leo")
}

fn add_to_string(mut p1: String) -> String {
    p1.push_str("is awesome!!");
    p1
}