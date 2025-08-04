fn main() {
    // vector holds the elements of the same type
    // vector are growable
    // vector always allocate memory on the heap

    //create vector
    let v1a: Vec<String> = Vec::new();
    let mut v1b = Vec::new();
    v1b.push(String::from("One"));
    v1b.push(String::from("Two"));
    v1b.push(String::from("Three")); // vector has the ownership of these elements

    let mut v2 = vec![6, 7, 8];

    // index into a vector
    let s = &v2[0];// can panic(程序报错崩溃)
    println!("s is: {s}");// s is: 6
    let s2 = v2.get(0); // won't panic
    if let Some(value) = s2 {
        println!("s2 is: {value}");// s2 is: 6
    }
    
    // remove element from vector
    let s = v2.remove(0);  // it will shift all the elements after it to the left
    println!("s is: {s}");// s is: 6

    //iterating over the elements in the vector
    for s in &mut v1b {
        s.push_str("!");
    }

    for s in &mut v1b {
        println!("s in v1b is:{s}");
    }
    /*
        s in v1b is:One!
        s in v1b is:Two!
        s in v1b is:Three!
     */

    let mut v3 = vec![];
    for s in v1b { // after this for loop call. v1b is no longer vaild
        v3.push(s);
    }

    for s in v3 { // same as: for s in v3.into_iter()
        println!("s in v3 is:{s}");
    }

    // Syntactic sugar(语法糖)
    // for s in v3.into_iter() {
    //     println!("s in v3 is:{s}");
    // }

    /*
        s in v3 is:One!
        s in v3 is:Two!
        s in v3 is:Three!
    */
}