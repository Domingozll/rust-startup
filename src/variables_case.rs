fn main() {    
    //creation
    // 32位整型 i32
    // 16位整型 i16
    // let a: i32  = 5;
    let a: i16 = 5;
    // let a: i16 = 5.0 //加了类型后编译器会做类型校验
    let c = 5.0; //没加类型编译器会自动判断类型
    
    //mutability
    // a = 10; //rust中的变量默认都是常量，需要加 mut关键字才能修改
    let mut b = 10;
    b = 20;

    // shadowing
    let c: i32 = 10;
    let c: i64 = 20; //c变量名会覆盖前面同名变量的值
    println!("c is: {c}"); //c is: 20

    /*
    * mutability是对同一个变量多次赋值
    * shadowing是新变量覆盖旧变量的值
    */

    //scope 作用域
    {
        let d = 30;
        println!("d is {d}"); // d is 30
    }
    // println!("d is {d}");// d不可见

    //标量数据类型
    //bool
    let b1: bool = true;

    // unsigned integers 无符号整数
    let i1: u8 = 1;
    let i2: u16 = 2;
    let i3: u32 = 3;
    let i4: u64 = 4;
    let i5: u128 = 5;

    // signed integers 有符号整数
    let i1: i8 = 1;
    let i2: i16 = -2;
    let i3: i32 = 3;
    let i4: i64 = -4;
    let i5: i128 = 5;

    println!("i4 is {i4}"); // i4 is -4

    //floating point numbers
    let f1: f32 = 1.0;
    let f2: f64 = 1.0;

    // platform specific integers
    let p1: usize = 1; // usize：无符号整数，用于表示内存中某个位置的大小或索引（64 位系统中是u64,32位系统中是 u32）
    let p2: isize = 1; // isize：有符号整数，用于表示偏移量或差值，也能指针运算（64 位系统中是i64,32位系统中是 i32）

    // characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");
}
