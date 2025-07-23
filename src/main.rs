fn main() {
    // println!("Hello, world!");
    
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
    let p1: usize = 1; // usize：指针大小的整数，用于存储内存地址或数组索引等
    let p2: isize = 1; // isize：指针大小的整数，用于存储内存地址或数组索引等

    // characters, &str, and String
    let c1: char = 'c';
    let s1: &str = "hello";
    let s2: String = String::from("hello");

    //复合数据类型
    //arrays （只能存储通类型数据）
    let a1: [i32; 5] = [1,2,3,4,5];
    let i1: i32 = a1[2];
    println!("i1 is:{i1}"); // i1 is:3

    // tuples (三元组，可存储不同类型数据)
    let t1: (i32, i32, i32) = (1,2,3);
    let t1: (i32, f64, &str) = (5,5.0,"05");
    let s1: &str = t1.2;
    println!("s1 is:{s1}");// s1 is:05
    let (i1, f1, s1) = t1;
    println!("i1 is:{i1},f1 is:{f1},s1 is:{s1}");//i1 is:5,f1 is:5,s1 is:05

    // empty tuples
    let uint:() = ();

    // type aliasing. a new name for a existing type（类型别名,可增强代码可读性）
    type age = u8;
    let a1: age = 57;
}
