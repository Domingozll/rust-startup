fn main() {
    {
        let s1: String = String::from("Rust"); // heap allocated string
        println!("s1 is: {s1}");
    } // s1 is droped

    let s2: String = String::from("Rust"); // heap allocated string
    println!("s2 is: {s2}");
    
    let s3: String = s2;
    // println!("s2 is: {s2}");//error value borrowed here after move
    println!("s3 is: {s3}");

    let s4: String = s3.clone();// heap allocated a copy of string
    println!("s4 is: {s4}");


    // 标量类型（基础类型）数据不会在堆上分配，不存在所有权问题
    let x: i32 = 10;
    let y: i32 = x; // 栈上直接复制
    println!("x is: {x}");
    println!("y is: {y}");
} // s2、s3、s4 is droped
