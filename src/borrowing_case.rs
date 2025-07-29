fn main() {
    let s1: String = String::from("Rust");
    print_string(s1);
    // println!("s1 is: {s1}");// error: borrow of moved value: `s1`
    
    let mut s2: String = String::from("Rust");
    let r1: &String = &s2;
    // let r2: &mut String = &mut s2; // cannot borrow `s2` as mutable because it is also borrowed as immutable
    print_string_2(r1); // after this line. r1 will never been used again.
    let r2: &mut String = &mut s2; // can be borrow as mutable
    add_to_string_2(r2);
    println!("r2 is: {r2}");// r2 is: Rust is awesome!!


    let s3: String = String::from("Leo");
    /**
     * add_to_string(s3);
     * println!("s3 is: {s3}");// error: borrow of moved value: `s3`
    */

    let s3: String = add_to_string(s3);
    println!("s3 is: {s3}"); // s3 is: Leo is awesome!!

    let r2: &mut String = &mut s2;
    
} 

fn print_string(p1: String) {
    println!("p1 is: {p1}");
}

fn print_string_2(p1: &String) {
    println!("p1 is: {p1}");
}

// not efficient.no need to take ownership of p1
fn add_to_string(mut p1: String) -> String {
    p1.push_str(" is awesome!!");
    p1
}

fn add_to_string_2(mut p1: &mut String) {
    p1.push_str(" is awesome!!");
}